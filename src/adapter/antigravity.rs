use serde_json::Value;

use crate::event::{AgentEvent, AgentEventKind, EventAdapter};
use crate::tmux::ANTIGRAVITY_AGENT;

use super::{HookRegistration, json_str, json_value_or_null, optional_str};

pub struct AntigravityAdapter;

fn get_cwd(input: &Value) -> String {
    if let Some(paths) = input.get("workspacePaths").and_then(|v| v.as_array())
        && let Some(first) = paths.first().and_then(|v| v.as_str())
    {
        return first.to_string();
    }
    String::new()
}

impl AntigravityAdapter {
    pub const HOOK_REGISTRATIONS: &'static [HookRegistration] = &[
        HookRegistration {
            trigger: "PreInvocation",
            matcher: None,
            kind: AgentEventKind::UserPromptSubmit,
        },
        HookRegistration {
            trigger: "PreToolUse",
            matcher: None,
            kind: AgentEventKind::ActivityLog,
        },
        HookRegistration {
            trigger: "Stop",
            matcher: None,
            kind: AgentEventKind::Stop,
        },
    ];
}

use crate::tool_name::CanonicalTool;

fn normalize_tool_name(raw: &str) -> String {
    let canonical = match raw {
        "run_command" => CanonicalTool::Bash,
        "view_file" => CanonicalTool::Read,
        "write_to_file" => CanonicalTool::Write,
        "replace_file_content" | "multi_replace_file_content" => CanonicalTool::Edit,
        "grep_search" => CanonicalTool::Grep,
        "search_web" => CanonicalTool::WebSearch,
        "read_url_content" => CanonicalTool::WebFetch,
        "ask_question" => CanonicalTool::AskUserQuestion,
        "invoke_subagent" => CanonicalTool::Agent,
        other => return other.to_string(),
    };
    canonical.as_str().to_string()
}

fn normalize_tool_input(tool_name: &str, input: Value) -> Value {
    let Value::Object(mut map) = input else {
        return input;
    };
    let rewrites: &[(&str, &str)] = match tool_name {
        "Bash" => &[("CommandLine", "command")],
        "Read" => &[("AbsolutePath", "file_path")],
        "Write" | "Edit" => &[("TargetFile", "file_path")],
        "Grep" => &[("Query", "pattern")],
        "WebSearch" => &[("query", "query")],
        "WebFetch" => &[("Url", "url")],
        _ => &[],
    };
    for (src, dst) in rewrites {
        if !map.contains_key(*dst)
            && let Some(value) = map.get(*src).cloned()
        {
            map.insert((*dst).to_string(), value);
        }
    }
    Value::Object(map)
}

impl EventAdapter for AntigravityAdapter {
    fn parse(&self, event_name: &str, input: &Value) -> Option<AgentEvent> {
        let cwd = get_cwd(input);
        let session_id = optional_str(input, "conversationId");

        match event_name {
            "user-prompt-submit" => Some(AgentEvent::UserPromptSubmit {
                agent: ANTIGRAVITY_AGENT.into(),
                cwd,
                permission_mode: String::new(),
                prompt: String::new(),
                worktree: None,
                agent_id: None,
                session_id,
            }),
            "activity-log" => {
                if let Some(tool_call) = input.get("toolCall") {
                    let raw_name = json_str(tool_call, "name");
                    if raw_name.is_empty() || raw_name == "NO_TOOL_CALL" {
                        return None;
                    }
                    let tool_name = normalize_tool_name(raw_name);
                    let tool_input =
                        normalize_tool_input(&tool_name, json_value_or_null(tool_call, "args"));
                    Some(AgentEvent::ActivityLog {
                        tool_name,
                        tool_input,
                        tool_response: Value::Null,
                    })
                } else {
                    None
                }
            }
            "stop" => {
                let error = json_str(input, "error");
                if !error.is_empty() {
                    Some(AgentEvent::StopFailure {
                        agent: ANTIGRAVITY_AGENT.into(),
                        cwd,
                        permission_mode: String::new(),
                        error: error.into(),
                        worktree: None,
                        agent_id: None,
                        session_id,
                    })
                } else {
                    Some(AgentEvent::Stop {
                        agent: ANTIGRAVITY_AGENT.into(),
                        cwd,
                        permission_mode: String::new(),
                        last_message: json_str(input, "terminationReason").into(),
                        response: None,
                        worktree: None,
                        agent_id: None,
                        session_id,
                    })
                }
            }
            _ => None,
        }
    }
}
