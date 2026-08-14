#[allow(dead_code, unused_imports)]
mod test_helpers;

use ratatui::style::Color;
use test_helpers::*;
use tmux_agent_sidebar::activity::{ActivityEntry, TaskProgress, TaskStatus};
use tmux_agent_sidebar::state::{BottomTab, Focus};
use tmux_agent_sidebar::tmux::{AgentType, PaneStatus, PermissionMode, SessionInfo, WindowInfo};
use tmux_agent_sidebar::ui::colors::ColorTheme;

// ─── ColorTheme Default Values ──────────────────────────────────────

#[test]
fn test_all_color_theme_defaults() {
    let theme = ColorTheme::default();

    // Core UI colors
    assert_eq!(theme.accent, Color::Indexed(14));
    assert_eq!(theme.border_inactive, Color::Indexed(8));
    assert_eq!(theme.selection_bg, Color::Indexed(8));

    // Status colors
    assert_eq!(theme.status_all, Color::Indexed(12));
    assert_eq!(theme.status_running, Color::Indexed(10));
    assert_eq!(theme.status_waiting, Color::Indexed(11));
    assert_eq!(theme.status_idle, Color::Indexed(14));
    assert_eq!(theme.status_error, Color::Indexed(9));
    assert_eq!(theme.status_unknown, Color::Indexed(8));

    // Agent colors
    assert_eq!(theme.agent_claude, Color::Indexed(13));
    assert_eq!(theme.agent_codex, Color::Indexed(5));
    assert_eq!(theme.agent_opencode, Color::Indexed(6));
    assert_eq!(theme.agent_antigravity, Color::Indexed(12));
    assert_eq!(theme.pet_body, Color::Indexed(11));
    assert_eq!(theme.pet_eye, Color::Indexed(10));

    // Text colors
    assert_eq!(theme.text_active, Color::Indexed(15));
    assert_eq!(theme.text_muted, Color::Indexed(7));
    assert_eq!(theme.text_inactive, Color::Indexed(8));

    // Header/UI element colors
    assert_eq!(theme.session_header, Color::Indexed(12));
    assert_eq!(theme.port, Color::Indexed(8));
    assert_eq!(theme.wait_reason, Color::Indexed(11));
    assert_eq!(theme.branch, Color::Indexed(6));

    // New theme fields
    assert_eq!(theme.badge_danger, Color::Indexed(9));
    assert_eq!(theme.badge_auto, Color::Indexed(11));
    assert_eq!(theme.task_progress, Color::Indexed(11));
    assert_eq!(theme.subagent, Color::Indexed(6));
    assert_eq!(theme.commit_hash, Color::Indexed(11));
    assert_eq!(theme.diff_added, Color::Indexed(10));
    assert_eq!(theme.diff_deleted, Color::Indexed(9));
    assert_eq!(theme.file_change, Color::Indexed(11));
    assert_eq!(theme.pr_link, Color::Indexed(14));
}

// ─── status_color() for all PaneStatus variants ─────────────────────

#[test]
fn test_status_color_all_variants() {
    let theme = ColorTheme::default();

    assert_eq!(
        theme.status_color(&PaneStatus::Running, false),
        Color::Indexed(10)
    );
    assert_eq!(
        theme.status_color(&PaneStatus::Waiting, false),
        Color::Indexed(11)
    );
    assert_eq!(
        theme.status_color(&PaneStatus::Idle, false),
        Color::Indexed(14)
    );
    assert_eq!(
        theme.status_color(&PaneStatus::Error, false),
        Color::Indexed(9)
    );
    assert_eq!(
        theme.status_color(&PaneStatus::Unknown, false),
        Color::Indexed(8)
    );
}

#[test]
fn test_status_color_attention_overrides_all() {
    let theme = ColorTheme::default();

    // attention=true should always return status_waiting regardless of status
    for status in &[
        PaneStatus::Running,
        PaneStatus::Waiting,
        PaneStatus::Idle,
        PaneStatus::Error,
        PaneStatus::Unknown,
    ] {
        assert_eq!(
            theme.status_color(status, true),
            theme.status_waiting,
            "attention=true should override {:?} to waiting color",
            status
        );
    }
}

// ─── agent_color() for all AgentType variants ───────────────────────

#[test]
fn test_agent_color_all_variants() {
    let theme = ColorTheme::default();

    assert_eq!(theme.agent_color(&AgentType::Claude), Color::Indexed(13));
    assert_eq!(theme.agent_color(&AgentType::Codex), Color::Indexed(5));
    assert_eq!(theme.agent_color(&AgentType::OpenCode), Color::Indexed(6));
    assert_eq!(
        theme.agent_color(&AgentType::Antigravity),
        Color::Indexed(12)
    );
    assert_eq!(theme.agent_color(&AgentType::Unknown), theme.status_unknown);
}

// ─── PermissionMode badge colors ────────────────────────────────────

#[test]
fn test_permission_mode_bypass_all_renders_danger_color() {
    let mut pane = make_pane(AgentType::Claude, PaneStatus::Running);
    pane.permission_mode = PermissionMode::BypassPermissions;

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;

    // Styled snapshot locks in the BypassAll `!` badge rendered with
    // badge_danger (fg:167).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 26), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13] [fg:9]![fg:9]


    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

#[test]
fn test_permission_mode_full_auto_renders_auto_color() {
    let mut pane = make_pane(AgentType::Claude, PaneStatus::Running);
    pane.permission_mode = PermissionMode::Auto;

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;

    // Styled snapshot locks in the Auto `auto` badge rendered with
    // badge_auto (fg:221).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 26), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13] [fg:11]a[fg:11]u[fg:11]t[fg:11]o[fg:11]


    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

#[test]
fn test_permission_mode_normal_no_badge() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);
    // permission_mode is Normal by default in make_pane

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;

    // Snapshot locks in that the agent row shows no badge in Normal mode.
    insta::assert_snapshot!(render_to_string(&mut state, 28, 25), @"
     ≡ 1  ● 1  ◎ 0  ◐ 0  ○ 0  ✕
    ⓘ                        — ▾
    project
    ┃ ● claude
    ╭ Activity │ Git ──────────╮
    │      No activity yet     │
    ╰──────────────────────────╯
    ");
}

// ─── Activity tool_color_index all branches ─────────────────────────

#[test]
fn test_tool_color_all_branches() {
    let cases = vec![
        ("Edit", 11),
        ("Write", 11),
        ("Bash", 10),
        ("Read", 12),
        ("Glob", 12),
        ("Grep", 12),
        ("Agent", 13),
        ("UnknownTool", 8),
        ("", 8),
    ];
    for (tool, expected) in cases {
        let entry = ActivityEntry {
            timestamp: "10:00".into(),
            tool: tool.into(),
            label: "test".into(),
        };
        assert_eq!(
            entry.tool_color_index(),
            expected,
            "tool_color_index for '{}'",
            tool
        );
    }
}

// ─── Git status summary colors ──────────────────────────────────────

#[test]
fn test_git_summary_modified_uses_badge_auto_color() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();

    state.bottom_tab = BottomTab::GitStatus;
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.git.branch = "main".into();
    state.git.unstaged_files = vec![tmux_agent_sidebar::git::GitFileEntry {
        status: 'M',
        name: "src/lib.rs".into(),
        additions: 5,
        deletions: 2,
        path: String::new(),
    }];

    // Styled snapshot locks in the Modified file badge color
    // (badge_auto fg:221).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]

    ╭[fg:14] [fg:14]A[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:8]│[fg:8] [fg:8]G[fg:14]i[fg:14]t[fg:14] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]m[fg:15]a[fg:15]i[fg:15]n[fg:15] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]1[fg:7] [fg:7]f[fg:7]i[fg:7]l[fg:7]e[fg:7]s[fg:7]│[fg:14]
    │[fg:14]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]│[fg:14]
    │[fg:14]U[fg:14]n[fg:14]s[fg:14]t[fg:14]a[fg:14]g[fg:14]e[fg:14]d[fg:14] [fg:14]([fg:14]1[fg:14])[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]M[fg:11] [fg:14]s[fg:7]r[fg:7]c[fg:7]/[fg:7]l[fg:7]i[fg:7]b[fg:7].[fg:7]r[fg:7]s[fg:7] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]+[fg:10]5[fg:10]/[fg:7]-[fg:9]2[fg:9]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    ╰[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╯[fg:14]
    ");
}

// ─── Render Tests: verify correct colors in styled output ───────────

#[test]
fn test_task_progress_line_uses_task_progress_color() {
    let mut pane = make_pane(AgentType::Claude, PaneStatus::Running);
    pane.pane_id = "%1".into();

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;

    // Set task progress for pane %1
    let progress = TaskProgress {
        tasks: vec![
            ("Task A".into(), TaskStatus::Completed),
            ("Task B".into(), TaskStatus::InProgress),
            ("Task C".into(), TaskStatus::Pending),
        ],
    };
    state.set_pane_task_progress("%1", Some(progress));

    // Styled snapshot locks in both the task_progress color (fg:223) and
    // the progress glyphs (✔/◼/◻) with the "1/3" count.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 40, 40), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8] 0[fg:8]
    ⓘ[fg:11]                                    —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
       [fg:11] [fg:11]✔[fg:11]◼[fg:11]◻[fg:11] [fg:11]1[fg:11]/[fg:11]3[fg:11]















    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

#[test]
fn test_subagent_line_uses_subagent_color() {
    let mut pane = make_pane(AgentType::Claude, PaneStatus::Running);
    pane.subagents = vec!["Explore #1".into()];

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;

    // Styled snapshot locks in the subagent line color (fg:73) plus the
    // rendered "Explore #1" label.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 40, 27), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8] 0[fg:8]
    ⓘ[fg:11]                                    —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
       [fg:7] [fg:7]└[fg:7] [fg:7]E[fg:6]x[fg:6]p[fg:6]l[fg:6]o[fg:6]r[fg:6]e[fg:6] [fg:6]#[fg:6]1[fg:6]


    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

#[test]
fn test_response_arrow_uses_response_arrow_color() {
    let mut pane = make_pane(AgentType::Claude, PaneStatus::Idle);
    pane.pane_active = false;
    pane.prompt = "Task completed successfully".into();
    pane.prompt_is_response = true;

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;

    // Styled snapshot locks in:
    //   • response_arrow color (fg:81) + bold on the ▷ glyph
    //   • text_active color (fg:255) on the focused response text
    insta::assert_snapshot!(render_to_styled_string(&mut state, 40, 27), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 1[fg:15]  ✕[fg:8] 0[fg:8]
    ⓘ[fg:11]                                    —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ○[fg:14] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
      ▷[fg:14,bold] [fg:14,bold]T[fg:15]a[fg:15]s[fg:15]k[fg:15] [fg:15]c[fg:15]o[fg:15]m[fg:15]p[fg:15]l[fg:15]e[fg:15]t[fg:15]e[fg:15]d[fg:15] [fg:15]s[fg:15]u[fg:15]c[fg:15]c[fg:15]e[fg:15]s[fg:15]s[fg:15]f[fg:15]u[fg:15]l[fg:15]l[fg:15]y[fg:15]


    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

// test_commit_hash_uses_commit_hash_color was removed because
// git_last_commit and commit hash rendering no longer exist.

#[test]
fn test_pr_link_uses_pr_link_color() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();

    state.bottom_tab = BottomTab::GitStatus;
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.git.branch = "feature/test".into();
    state.git.pr_number = Some("99".into());
    state.git.remote_url = "https://github.com/user/repo".into();

    // Styled snapshot locks in the PR link: pr_link color (fg:117) plus
    // the underline modifier on the `#99` glyphs.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 40), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
















    ╭[fg:14] [fg:14]A[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:8]│[fg:8] [fg:8]G[fg:14]i[fg:14]t[fg:14] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]f[fg:15]e[fg:15]a[fg:15]t[fg:15]u[fg:15]r[fg:15]e[fg:15]/[fg:15]t[fg:15]e[fg:15]s[fg:15]t[fg:15] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]#[fg:14,underline]9[fg:14,underline]9[fg:14,underline]│[fg:14]
    │[fg:14]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14]W[fg:7]o[fg:7]r[fg:7]k[fg:7]i[fg:7]n[fg:7]g[fg:7] [fg:7]t[fg:7]r[fg:7]e[fg:7]e[fg:7] [fg:7]c[fg:7]l[fg:7]e[fg:7]a[fg:7]n[fg:7] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    ╰[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╯[fg:14]
    ");
}

#[test]
fn test_diff_stat_added_uses_diff_added_color() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();

    state.bottom_tab = BottomTab::GitStatus;
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.git.branch = "main".into();
    state.git.diff_stat = Some((42, 10));

    // Styled snapshot locks in the `+42` additions rendered with
    // diff_added color (fg:114).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 40), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
















    ╭[fg:14] [fg:14]A[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:8]│[fg:8] [fg:8]G[fg:14]i[fg:14]t[fg:14] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]m[fg:15]a[fg:15]i[fg:15]n[fg:15] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]+[fg:10]4[fg:10]2[fg:10]/[fg:7]-[fg:9]1[fg:9]0[fg:9] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]0[fg:7] [fg:7]f[fg:7]i[fg:7]l[fg:7]e[fg:7]s[fg:7]│[fg:14]
    │[fg:14]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14]W[fg:7]o[fg:7]r[fg:7]k[fg:7]i[fg:7]n[fg:7]g[fg:7] [fg:7]t[fg:7]r[fg:7]e[fg:7]e[fg:7] [fg:7]c[fg:7]l[fg:7]e[fg:7]a[fg:7]n[fg:7] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    ╰[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╯[fg:14]
    ");
}

#[test]
fn test_diff_stat_deleted_uses_diff_deleted_color() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();

    state.bottom_tab = BottomTab::GitStatus;
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.git.branch = "main".into();
    state.git.diff_stat = Some((0, 25));

    // Styled snapshot locks in the `-25` deletions rendered with
    // diff_deleted color (fg:174).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]

    ╭[fg:14] [fg:14]A[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:8]│[fg:8] [fg:8]G[fg:14]i[fg:14]t[fg:14] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]m[fg:15]a[fg:15]i[fg:15]n[fg:15] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]+[fg:10]0[fg:10]/[fg:7]-[fg:9]2[fg:9]5[fg:9] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]0[fg:7] [fg:7]f[fg:7]i[fg:7]l[fg:7]e[fg:7]s[fg:7]│[fg:14]
    │[fg:14]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14]W[fg:7]o[fg:7]r[fg:7]k[fg:7]i[fg:7]n[fg:7]g[fg:7] [fg:7]t[fg:7]r[fg:7]e[fg:7]e[fg:7] [fg:7]c[fg:7]l[fg:7]e[fg:7]a[fg:7]n[fg:7] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    ╰[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╯[fg:14]
    ");
}

#[test]
fn test_file_change_stat_uses_file_change_color() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();

    state.bottom_tab = BottomTab::GitStatus;
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.git.branch = "main".into();
    state.git.unstaged_files = vec![tmux_agent_sidebar::git::GitFileEntry {
        status: 'M',
        name: "lib.rs".into(),
        additions: 40,
        deletions: 10,
        path: String::new(),
    }];

    // Styled snapshot locks in the `M lib.rs` row rendered with
    // badge_auto (fg:221) on the status glyph.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]

    ╭[fg:14] [fg:14]A[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:8]│[fg:8] [fg:8]G[fg:14]i[fg:14]t[fg:14] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]m[fg:15]a[fg:15]i[fg:15]n[fg:15] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]1[fg:7] [fg:7]f[fg:7]i[fg:7]l[fg:7]e[fg:7]s[fg:7]│[fg:14]
    │[fg:14]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]│[fg:14]
    │[fg:14]U[fg:14]n[fg:14]s[fg:14]t[fg:14]a[fg:14]g[fg:14]e[fg:14]d[fg:14] [fg:14]([fg:14]1[fg:14])[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]M[fg:11] [fg:14]l[fg:7]i[fg:7]b[fg:7].[fg:7]r[fg:7]s[fg:7] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]+[fg:10]4[fg:10]0[fg:10]/[fg:7]-[fg:9]1[fg:9]0[fg:9]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    ╰[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╯[fg:14]
    ");
}

// ─── Custom theme overrides for new fields ──────────────────────────

#[test]
fn test_custom_theme_new_fields_override() {
    let theme = ColorTheme {
        badge_danger: Color::Indexed(196),
        badge_auto: Color::Indexed(226),
        task_progress: Color::Indexed(200),
        subagent: Color::Indexed(100),
        commit_hash: Color::Indexed(150),
        diff_added: Color::Indexed(46),
        diff_deleted: Color::Indexed(160),
        file_change: Color::Indexed(208),
        pr_link: Color::Indexed(33),
        port: Color::Indexed(82),
        ..ColorTheme::default()
    };

    assert_eq!(theme.badge_danger, Color::Indexed(196));
    assert_eq!(theme.badge_auto, Color::Indexed(226));
    assert_eq!(theme.task_progress, Color::Indexed(200));
    assert_eq!(theme.subagent, Color::Indexed(100));
    assert_eq!(theme.commit_hash, Color::Indexed(150));
    assert_eq!(theme.diff_added, Color::Indexed(46));
    assert_eq!(theme.diff_deleted, Color::Indexed(160));
    assert_eq!(theme.file_change, Color::Indexed(208));
    assert_eq!(theme.pr_link, Color::Indexed(33));
    assert_eq!(theme.port, Color::Indexed(82));

    // Original fields should still be default
    assert_eq!(theme.accent, Color::Indexed(14));
    assert_eq!(theme.agent_claude, Color::Indexed(13));
    assert_eq!(theme.pet_body, Color::Indexed(11));
    assert_eq!(theme.pet_eye, Color::Indexed(10));
}

// ─── Branch color in styled output ──────────────────────────────────

#[test]
fn test_branch_color_in_agent_panel() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![tmux_agent_sidebar::group::RepoGroup {
        name: "project".into(),
        has_focus: true,
        panes: vec![(
            pane,
            tmux_agent_sidebar::group::PaneGitInfo {
                repo_root: Some("/home/user/project".into()),
                branch: Some("feature/cool-feature".into()),
                is_worktree: false,
                worktree_name: None,
            },
        )],
    }];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;
    state.bottom_panel_height = 0;

    // Styled snapshot locks in the branch name rendered with branch color
    // (fg:109).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 40, 26), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8] 0[fg:8]
    ⓘ[fg:11]                                    —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]                                +[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
    ┃[fg:14]  [fg:6] [fg:6]f[fg:6]e[fg:6]a[fg:6]t[fg:6]u[fg:6]r[fg:6]e[fg:6]/[fg:6]c[fg:6]o[fg:6]o[fg:6]l[fg:6]-[fg:6]f[fg:6]e[fg:6]a[fg:6]t[fg:6]u[fg:6]r[fg:6]e[fg:6]
    ");
}

// ─── Selection background color ─────────────────────────────────────

#[test]
fn test_selection_bg_color_applied() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Idle);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = true;
    state.global.selected_pane_row = 0;

    // Styled snapshot locks in the selected agent row's selection
    // background (bg:239).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 1[fg:15]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    ┃[fg:14,bg:8] [bg:8]○[fg:14,bg:8] [fg:13,bg:8]c[fg:13,bg:8]l[fg:13,bg:8]a[fg:13,bg:8]u[fg:13,bg:8]d[fg:13,bg:8]e[fg:13,bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8]
       [fg:15] [fg:15]W[fg:15]a[fg:15]i[fg:15]t[fg:15]i[fg:15]n[fg:15]g[fg:15] [fg:15]f[fg:15]o[fg:15]r[fg:15] [fg:15]p[fg:15]r[fg:15]o[fg:15]m[fg:15]p[fg:15]t[fg:15]…[fg:15]

    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

// ─── Accent (focused) vs border_inactive colors ─────────────────────

#[test]
fn test_accent_vs_border_inactive_colors() {
    let mut pane1 = make_pane(AgentType::Claude, PaneStatus::Running);
    pane1.pane_id = "%1".into();
    let mut pane2 = make_pane(AgentType::Codex, PaneStatus::Idle);
    pane2.pane_id = "%2".into();

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@0".into(),
            window_name: "fish".into(),
            window_active: true,
            auto_rename: true,
            panes: vec![pane1.clone(), pane2.clone()],
        }],
    }]);
    state.repo_groups = vec![
        tmux_agent_sidebar::group::RepoGroup {
            name: "focused-repo".into(),
            has_focus: true,
            panes: vec![(pane1, tmux_agent_sidebar::group::PaneGitInfo::default())],
        },
        tmux_agent_sidebar::group::RepoGroup {
            name: "unfocused-repo".into(),
            has_focus: false,
            panes: vec![(pane2, tmux_agent_sidebar::group::PaneGitInfo::default())],
        },
    ];
    state.focus_state.focused_pane_id = Some("%1".into());
    state.rebuild_row_targets();

    // Styled snapshot locks in:
    //   • focused group header rendered with accent (fg:153)
    //   • unfocused group header rendered with border_inactive (fg:240)
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 30), @"
     ≡[fg:12] 2[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 1[fg:15]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    f[fg:14]o[fg:14]c[fg:14]u[fg:14]s[fg:14]e[fg:14]d[fg:14]-[fg:14]r[fg:14]e[fg:14]p[fg:14]o[fg:14]
    ┃[fg:14,bg:8] [bg:8]●[fg:82,bg:8] [fg:13,bg:8]c[fg:13,bg:8]l[fg:13,bg:8]a[fg:13,bg:8]u[fg:13,bg:8]d[fg:13,bg:8]e[fg:13,bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8]

    u[fg:15]n[fg:15]f[fg:15]o[fg:15]c[fg:15]u[fg:15]s[fg:15]e[fg:15]d[fg:15]-[fg:15]r[fg:15]e[fg:15]p[fg:15]o[fg:15]
      ○[fg:14] [fg:5]c[fg:5]o[fg:5]d[fg:5]e[fg:5]x[fg:5]
       [fg:8] [fg:8]W[fg:8]a[fg:8]i[fg:8]t[fg:8]i[fg:8]n[fg:8]g[fg:8] [fg:8]f[fg:8]o[fg:8]r[fg:8] [fg:8]p[fg:8]r[fg:8]o[fg:8]m[fg:8]p[fg:8]t[fg:8]…[fg:8]


    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

// ─── Status color rendering for each PaneStatus ─────────────────────

#[test]
fn test_running_status_color_in_output() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Running);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;
    state.bottom_panel_height = 0;

    // Styled snapshot locks in the running spinner using SPINNER_PULSE[0]
    // color (fg:82).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ●[fg:82] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
    ");
}

#[test]
fn test_waiting_status_color_in_output() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Waiting);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;
    state.bottom_panel_height = 0;

    // Styled snapshot locks in the waiting status using status_waiting
    // color (fg:221).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 1[fg:15]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ◐[fg:11] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
    ");
}

#[test]
fn test_error_status_color_in_output() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Error);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;
    state.bottom_panel_height = 0;

    // Styled snapshot locks in the error status using status_error
    // color (fg:167).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ✕[fg:9] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
    ");
}

#[test]
fn test_idle_status_color_in_output() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Idle);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", vec![pane])];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false;

    // Styled snapshot locks in the idle status using status_idle
    // color (fg:110).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 25), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 1[fg:15]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14] ○[fg:14] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]

    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    ╰[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╯[fg:8]
    ");
}

#[test]
fn test_unknown_status_color_in_output() {
    let theme = tmux_agent_sidebar::ui::colors::ColorTheme::default();
    assert_eq!(
        theme.status_color(&PaneStatus::Unknown, false),
        ratatui::style::Color::Indexed(8)
    );
}
