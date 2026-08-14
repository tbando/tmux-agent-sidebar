#[allow(dead_code, unused_imports)]
mod test_helpers;

use test_helpers::*;
use tmux_agent_sidebar::activity::ActivityEntry;
use tmux_agent_sidebar::state::{BottomTab, Focus};
use tmux_agent_sidebar::tmux::{AgentType, PaneStatus, SessionInfo, WindowInfo};

// ─── Styled Snapshot Tests for Selection and Focus ─────────────────

#[test]
fn snapshot_selected_focused_styled() {
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
    state.bottom_panel_height = 0;

    // Styled snapshot locks in the selected row's ┃[fg:153,bg:239] marker
    // and the selection background spanning its content cells.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 10), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 1[fg:15]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]
    ┃[fg:14,bg:8] [bg:8]○[fg:14,bg:8] [fg:13,bg:8]c[fg:13,bg:8]l[fg:13,bg:8]a[fg:13,bg:8]u[fg:13,bg:8]d[fg:13,bg:8]e[fg:13,bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8]
       [fg:15] [fg:15]W[fg:15]a[fg:15]i[fg:15]t[fg:15]i[fg:15]n[fg:15]g[fg:15] [fg:15]f[fg:15]o[fg:15]r[fg:15] [fg:15]p[fg:15]r[fg:15]o[fg:15]m[fg:15]p[fg:15]t[fg:15]…[fg:15]
    ");
}

#[test]
fn snapshot_activity_focused_styled() {
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
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.activity.entries = vec![ActivityEntry {
        timestamp: "10:32".into(),
        tool: "Edit".into(),
        label: "src/main.rs".into(),
    }];

    // Styled snapshot locks in the focused group header accent (fg:153) and
    // the active-panel border color.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 14), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]

    ╭[fg:14] [fg:14]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14]1[fg:15]0[fg:15]:[fg:15]3[fg:15]2[fg:15] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]E[fg:11]d[fg:11]i[fg:11]t[fg:11]│[fg:14]
    │[fg:14] [fg:7] [fg:7]s[fg:7]r[fg:7]c[fg:7]/[fg:7]m[fg:7]a[fg:7]i[fg:7]n[fg:7].[fg:7]r[fg:7]s[fg:7] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
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
fn snapshot_activity_unfocused_styled() {
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
    state.focus_state.focus = Focus::Panes; // not activity
    state.focus_state.sidebar_focused = true;
    state.activity.entries = vec![ActivityEntry {
        timestamp: "10:32".into(),
        tool: "Edit".into(),
        label: "src/main.rs".into(),
    }];

    // Styled snapshot locks in the unfocused bottom-panel border
    // (border_inactive fg:240).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 14), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]

    ╭[fg:8] [fg:8]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]─[fg:8]╮[fg:8]
    │[fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
    │[fg:8]1[fg:15]0[fg:15]:[fg:15]3[fg:15]2[fg:15] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]E[fg:11]d[fg:11]i[fg:11]t[fg:11]│[fg:8]
    │[fg:8] [fg:7] [fg:7]s[fg:7]r[fg:7]c[fg:7]/[fg:7]m[fg:7]a[fg:7]i[fg:7]n[fg:7].[fg:7]r[fg:7]s[fg:7] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8] [fg:8]│[fg:8]
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
fn bottom_tab_activity_uses_accent_when_selected() {
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
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.bottom_tab = BottomTab::Activity;

    // Styled snapshot locks in `A` using accent (fg:153) and `G` remaining
    // muted (fg:252) on the bottom-panel tab title row.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 14), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]

    ╭[fg:14] [fg:14]A[fg:14]c[fg:14]t[fg:14]i[fg:14]v[fg:14]i[fg:14]t[fg:14]y[fg:14] [fg:8]│[fg:8] [fg:8]G[fg:7]i[fg:7]t[fg:7] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]N[fg:7]o[fg:7] [fg:7]a[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:7]y[fg:7]e[fg:7]t[fg:7] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    │[fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14] [fg:14]│[fg:14]
    ╰[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╯[fg:14]
    ");
}

#[test]
fn bottom_tab_git_uses_accent_when_selected() {
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
    state.focus_state.focus = Focus::ActivityLog;
    state.focus_state.sidebar_focused = true;
    state.bottom_tab = BottomTab::GitStatus;

    // Styled snapshot locks in `G` using accent (fg:153) and `A` remaining
    // muted (fg:252) on the bottom-panel tab title row.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 14), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]

    ╭[fg:14] [fg:14]A[fg:7]c[fg:7]t[fg:7]i[fg:7]v[fg:7]i[fg:7]t[fg:7]y[fg:7] [fg:8]│[fg:8] [fg:8]G[fg:14]i[fg:14]t[fg:14] [fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╮[fg:14]
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
    ╰[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]╯[fg:14]
    ");
}

// ─── Selection Background Border Tests ───────────────────────────────

#[test]
fn selection_marker_uses_accent_color_with_selection_bg() {
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
    state.focus_state.sidebar_focused = true;
    state.focus_state.focus = Focus::Panes;
    state.global.selected_pane_row = 0;

    // Styled snapshot locks in:
    //   1. the selected row begins with `┃[fg:153,bg:239]` (accent + selection bg)
    //   2. the selected row never contains the old frame `│`
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 24), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    ┃[fg:14,bg:8] [bg:8]●[fg:10,bg:8] [fg:13,bg:8]c[fg:13,bg:8]l[fg:13,bg:8]a[fg:13,bg:8]u[fg:13,bg:8]d[fg:13,bg:8]e[fg:13,bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8] [bg:8]

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
fn selection_bg_covers_inner_padding() {
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
    state.focus_state.focus = Focus::Panes;
    state.global.selected_pane_row = 0;

    // Styled snapshot locks in the selection background extending across the
    // inner padding immediately after the `┃` marker (` [bg:239]`).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 24), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 1[fg:15]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
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

#[test]
fn no_selection_bg_when_not_selected() {
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
    state.focus_state.sidebar_focused = false; // not focused → no selection

    // Styled snapshot locks in the absence of any selection background
    // (bg:239) while the sidebar is not focused.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 24), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 1[fg:15]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 0[fg:8]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:14]r[fg:14]o[fg:14]j[fg:14]e[fg:14]c[fg:14]t[fg:14]

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

// ─── Custom Theme Tests ─────────────────────────────────────────────

#[test]
fn snapshot_custom_theme_colors() {
    use ratatui::style::Color;
    use tmux_agent_sidebar::ui::colors::ColorTheme;

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

    // Override theme with custom colors
    state.theme = ColorTheme {
        accent: Color::Indexed(196),       // red accent
        agent_claude: Color::Indexed(226), // yellow agent
        status_idle: Color::Indexed(46),   // green idle
        port: Color::Indexed(39),          // cyan port
        ..ColorTheme::default()
    };
    // Unfocus sidebar so selected row doesn't use REVERSED (which hides colors)
    state.focus_state.sidebar_focused = false;
    state.bottom_panel_height = 0;

    // Styled snapshot locks in the custom theme colors (accent fg:196,
    // agent_claude fg:226, status_idle fg:46).
    insta::assert_snapshot!(render_to_styled_string(&mut state, 28, 10), @"
     ≡[fg:12] 1[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 1[fg:15]  ✕[fg:8]
    ⓘ[fg:11]                        —[fg:7] ▾[fg:7]
    p[fg:196]r[fg:196]o[fg:196]j[fg:196]e[fg:196]c[fg:196]t[fg:196]
    ┃[fg:196] ○[fg:46] [fg:226]c[fg:226]l[fg:226]a[fg:226]u[fg:226]d[fg:226]e[fg:226]
       [fg:15] [fg:15]W[fg:15]a[fg:15]i[fg:15]t[fg:15]i[fg:15]n[fg:15]g[fg:15] [fg:15]f[fg:15]o[fg:15]r[fg:15] [fg:15]p[fg:15]r[fg:15]o[fg:15]m[fg:15]p[fg:15]t[fg:15]…[fg:15]
    ");
}

#[test]
fn test_theme_default_matches_shell_colors() {
    use ratatui::style::Color;
    use tmux_agent_sidebar::ui::colors::ColorTheme;

    let theme = ColorTheme::default();

    // Verify defaults match ANSI 16 palette
    assert_eq!(theme.accent, Color::Indexed(14));
    assert_eq!(theme.border_inactive, Color::Indexed(8));
    assert_eq!(theme.status_running, Color::Indexed(10));
    assert_eq!(theme.status_waiting, Color::Indexed(11));
    assert_eq!(theme.status_idle, Color::Indexed(14));
    assert_eq!(theme.status_error, Color::Indexed(9));
    assert_eq!(theme.agent_claude, Color::Indexed(13));
    assert_eq!(theme.agent_codex, Color::Indexed(5));
    assert_eq!(theme.text_active, Color::Indexed(15));
    assert_eq!(theme.text_muted, Color::Indexed(7));
    assert_eq!(theme.session_header, Color::Indexed(12));
    assert_eq!(theme.wait_reason, Color::Indexed(11));
}
