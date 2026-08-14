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
        let raw_model = optional_str(input, "modelName").or_else(|| optional_str(input, "model"));
        let explicit_effort = optional_str(input, "effort");
        let (model, effort) = match raw_model {
            Some(m) => {
                let (m_clean, eff) =
                    crate::tool_name::split_model_and_effort(&m, explicit_effort.as_deref());
                (Some(m_clean), eff)
            }
            None => (None, explicit_effort),
        };

        match event_name {
            "user-prompt-submit" => {
                let mode_str = optional_str(input, "permissionMode")
                    .or_else(|| optional_str(input, "mode"))
                    .unwrap_or_default();
                let permission_mode = match mode_str.as_str() {
                    "accept-edits" | "acceptEdits" => "acceptEdits".to_string(),
                    "plan" => "plan".to_string(),
                    "bypassPermissions" | "dangerously-skip-permissions" => {
                        "bypassPermissions".to_string()
                    }
                    "dontAsk" | "dont-ask" => "dontAsk".to_string(),
                    "auto" => "auto".to_string(),
                    other => other.to_string(),
                };
                let prompt = optional_str(input, "prompt")
                    .or_else(|| optional_str(input, "userPrompt"))
                    .unwrap_or_default();
                Some(AgentEvent::UserPromptSubmit {
                    agent: ANTIGRAVITY_AGENT.into(),
                    cwd,
                    permission_mode,
                    prompt,
                    worktree: None,
                    agent_id: None,
                    session_id,
                    model,
                    effort,
                })
            }
            "activity-log" => {
                let tool_call = input.get("toolCall");
                let raw_name = tool_call
                    .map(|tc| json_str(tc, "name"))
                    .unwrap_or_else(|| json_str(input, "tool_name"));
                if raw_name.is_empty() || raw_name == "NO_TOOL_CALL" {
                    return None;
                }
                let tool_name = normalize_tool_name(raw_name);
                if tool_name.is_empty() || tool_name == "NO_TOOL_CALL" {
                    return None;
                }
                let tool_input = if let Some(tc) = tool_call {
                    normalize_tool_input(&tool_name, json_value_or_null(tc, "args"))
                } else {
                    normalize_tool_input(&tool_name, json_value_or_null(input, "tool_input"))
                };
                Some(AgentEvent::ActivityLog {
                    tool_name,
                    tool_input,
                    tool_response: Value::Null,
                })
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
                        model,
                        effort,
                    })
                } else {
                    Some(AgentEvent::Stop {
                        agent: ANTIGRAVITY_AGENT.into(),
                        cwd,
                        permission_mode: String::new(),
                        last_message: String::new(),
                        response: None,
                        worktree: None,
                        agent_id: None,
                        session_id,
                        model,
                        effort,
                    })
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn hook_registrations_match_parse_arms() {
        super::super::assert_table_drift_free(
            "antigravity",
            AntigravityAdapter::HOOK_REGISTRATIONS,
        );
    }

    #[test]
    fn parse_user_prompt_submit_modes() {
        let adapter = AntigravityAdapter;

        // plan mode
        let input = json!({
            "workspacePaths": ["/path/to/repo"],
            "conversationId": "conv-123",
            "permissionMode": "plan",
            "prompt": "my test prompt"
        });
        let event = adapter.parse("user-prompt-submit", &input).unwrap();
        match event {
            AgentEvent::UserPromptSubmit {
                cwd,
                permission_mode,
                prompt,
                session_id,
                ..
            } => {
                assert_eq!(cwd, "/path/to/repo");
                assert_eq!(permission_mode, "plan");
                assert_eq!(prompt, "my test prompt");
                assert_eq!(session_id, Some("conv-123".to_string()));
            }
            _ => panic!("Expected UserPromptSubmit"),
        }

        // accept-edits mode via mode field
        let input = json!({
            "mode": "accept-edits",
            "userPrompt": "edit files"
        });
        let event = adapter.parse("user-prompt-submit", &input).unwrap();
        match event {
            AgentEvent::UserPromptSubmit {
                permission_mode,
                prompt,
                ..
            } => {
                assert_eq!(permission_mode, "acceptEdits");
                assert_eq!(prompt, "edit files");
            }
            _ => panic!("Expected UserPromptSubmit"),
        }

        // bypassPermissions mode
        let input = json!({
            "permissionMode": "dangerously-skip-permissions"
        });
        let event = adapter.parse("user-prompt-submit", &input).unwrap();
        match event {
            AgentEvent::UserPromptSubmit {
                permission_mode, ..
            } => {
                assert_eq!(permission_mode, "bypassPermissions");
            }
            _ => panic!("Expected UserPromptSubmit"),
        }
    }

    #[test]
    fn parse_user_prompt_submit_model_and_effort() {
        let adapter = AntigravityAdapter;
        let input = json!({
            "modelName": "gemini-3.7-flash-medium",
            "prompt": "hello"
        });
        let event = adapter.parse("user-prompt-submit", &input).unwrap();
        match event {
            AgentEvent::UserPromptSubmit { model, effort, .. } => {
                assert_eq!(model, Some("gemini-3.7-flash".into()));
                assert_eq!(effort, Some("medium".into()));
            }
            _ => panic!("Expected UserPromptSubmit"),
        }
    }
}
