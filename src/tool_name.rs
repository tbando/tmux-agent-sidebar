/// Canonical tool-name vocabulary used across agents. Claude and Codex emit
/// these PascalCase names natively; OpenCode's lowercase IDs are normalised to
/// this vocabulary in `src/adapter/opencode.rs`. Keeping the list as an enum
/// means typos in adapters or the strategy table become compile errors rather
/// than silently unmatched tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalTool {
    Bash,
    Read,
    Edit,
    Write,
    NotebookEdit,
    PowerShell,
    Monitor,
    PushNotification,
    Glob,
    Grep,
    WebFetch,
    WebSearch,
    ToolSearch,
    Skill,
    SendMessage,
    TeamCreate,
    Lsp,
    CronCreate,
    CronDelete,
    EnterWorktree,
    ExitWorktree,
    Agent,
    TaskCreate,
    TaskUpdate,
    TaskGet,
    TaskStop,
    TaskOutput,
    AskUserQuestion,
    TodoWrite,
}

impl CanonicalTool {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bash => "Bash",
            Self::Read => "Read",
            Self::Edit => "Edit",
            Self::Write => "Write",
            Self::NotebookEdit => "NotebookEdit",
            Self::PowerShell => "PowerShell",
            Self::Monitor => "Monitor",
            Self::PushNotification => "PushNotification",
            Self::Glob => "Glob",
            Self::Grep => "Grep",
            Self::WebFetch => "WebFetch",
            Self::WebSearch => "WebSearch",
            Self::ToolSearch => "ToolSearch",
            Self::Skill => "Skill",
            Self::SendMessage => "SendMessage",
            Self::TeamCreate => "TeamCreate",
            Self::Lsp => "LSP",
            Self::CronCreate => "CronCreate",
            Self::CronDelete => "CronDelete",
            Self::EnterWorktree => "EnterWorktree",
            Self::ExitWorktree => "ExitWorktree",
            Self::Agent => "Agent",
            Self::TaskCreate => "TaskCreate",
            Self::TaskUpdate => "TaskUpdate",
            Self::TaskGet => "TaskGet",
            Self::TaskStop => "TaskStop",
            Self::TaskOutput => "TaskOutput",
            Self::AskUserQuestion => "AskUserQuestion",
            Self::TodoWrite => "TodoWrite",
        }
    }
}

/// Split a model string and separate any trailing reasoning effort suffix
/// (e.g. `gemini-3.7-flash-medium` -> `("gemini-3.7-flash", Some("medium"))`).
pub fn split_model_and_effort(
    raw_model: &str,
    explicit_effort: Option<&str>,
) -> (String, Option<String>) {
    let mut model = raw_model.trim().to_string();
    let mut effort = explicit_effort
        .map(|e| e.trim().to_string())
        .filter(|e| !e.is_empty());

    if effort.is_none() {
        let lower = model.to_lowercase();
        const EFFORT_SUFFIXES: &[(&str, &str)] = &[
            ("-low", "low"),
            ("-medium", "medium"),
            ("-med", "medium"),
            ("-high", "high"),
            ("-max", "high"),
            ("-xhigh", "high"),
            ("/low", "low"),
            ("/medium", "medium"),
            ("/med", "medium"),
            ("/high", "high"),
            ("/max", "high"),
            ("/xhigh", "high"),
        ];

        for &(suffix, eff) in EFFORT_SUFFIXES {
            if lower.ends_with(suffix) {
                let cutoff = model.len() - suffix.len();
                model = model[..cutoff].to_string();
                effort = Some(eff.to_string());
                break;
            }
        }
    } else if let Some(ref eff) = effort {
        let lower = model.to_lowercase();
        let eff_lower = eff.to_lowercase();
        let dash_suffix = format!("-{}", eff_lower);
        let slash_suffix = format!("/{}", eff_lower);
        if lower.ends_with(&dash_suffix) {
            let cutoff = model.len() - dash_suffix.len();
            model = model[..cutoff].to_string();
        } else if lower.ends_with(&slash_suffix) {
            let cutoff = model.len() - slash_suffix.len();
            model = model[..cutoff].to_string();
        }
    }

    (model, effort)
}
