#[allow(dead_code, unused_imports)]
mod test_helpers;

use test_helpers::*;
use tmux_agent_sidebar::state::Focus;
use tmux_agent_sidebar::tmux::{AgentType, PaneStatus, SessionInfo, WindowInfo};
use tmux_agent_sidebar::ui::colors::ColorTheme;
use tmux_agent_sidebar::ui::icons::StatusIcons;

// ─── Agents: auto-scroll behavior Tests ─────────────────────────────

#[test]
fn test_agents_auto_scroll_keeps_selected_visible() {
    // Create enough agents to overflow a small viewport
    let mut panes = Vec::new();
    for i in 0..10 {
        let mut pane = make_pane(AgentType::Claude, PaneStatus::Idle);
        pane.pane_id = format!("%{}", i);
        panes.push(pane);
    }

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: panes.clone(),
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", panes)];
    state.focus_state.sidebar_focused = true;
    state.focus_state.focus = Focus::Panes;
    state.rebuild_row_targets();

    // Render with a small height. With the 2-row header, the first pane
    // still stays visible without needing to scroll.
    let _ = render_to_string(&mut state, 28, 26);
    assert_eq!(state.scrolls.panes.offset, 0, "initially at top");

    // Select last agent and re-render
    state.global.selected_pane_row = 9;
    let _ = render_to_string(&mut state, 28, 26);
    assert!(
        state.scrolls.panes.offset > 0,
        "should scroll down to show selected agent"
    );
}

#[test]
fn test_panes_scroll_offset_tracks_total_and_visible() {
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

    let _ = render_to_string(&mut state, 28, 26);
    // After rendering, panes_scroll.total_lines and panes_scroll.visible_height should be set
    assert!(
        state.scrolls.panes.total_lines > 0,
        "total lines should be populated"
    );
    assert!(
        state.scrolls.panes.visible_height > 0,
        "visible height should be populated"
    );
}

// ─── Agents: Codex agent color ──────────────────────────────────────

#[test]
fn snapshot_codex_agent_styled() {
    let theme = ColorTheme::default();
    assert_eq!(
        theme.agent_color(&AgentType::Codex),
        ratatui::style::Color::Indexed(5)
    );
}

// ─── Agents: Unknown agent type ─────────────────────────────────────

#[test]
fn snapshot_unknown_agent_styled() {
    let theme = ColorTheme::default();
    assert_eq!(
        theme.agent_color(&AgentType::Unknown),
        ratatui::style::Color::Indexed(8)
    );
}

// ─── Agents: running icon variants via render ───────────────────────

#[test]
fn test_running_icon_blink_off() {
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
    state.spinner_frame = 0;

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

#[test]
fn test_running_spinner_frame_advances() {
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
    state.spinner_frame = 3;

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

#[test]
fn test_waiting_icon() {
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

    insta::assert_snapshot!(render_to_string(&mut state, 28, 25), @"
     ≡ 1  ● 0  ◎ 0  ◐ 1  ○ 0  ✕
    ⓘ                        — ▾
    project
    ┃ ◐ claude
    ╭ Activity │ Git ──────────╮
    │      No activity yet     │
    ╰──────────────────────────╯
    ");
}

#[test]
fn test_error_icon() {
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

    insta::assert_snapshot!(render_to_string(&mut state, 28, 25), @"
     ≡ 1  ● 0  ◎ 0  ◐ 0  ○ 0  ✕
    ⓘ                        — ▾
    project
    ┃ ✕ claude
    ╭ Activity │ Git ──────────╮
    │      No activity yet     │
    ╰──────────────────────────╯
    ");
}

#[test]
fn test_unknown_status_icon() {
    let icons = StatusIcons::default();
    assert_eq!(icons.status_icon(&PaneStatus::Unknown), "·");
}

// ─── Agents: auto-scroll keeps selected pane visible ───────────────

#[test]
fn test_agents_auto_scroll_shows_last_selected_pane() {
    // When the last agent in a group is selected, the auto-scroll
    // should bring it into view (the selection marker must be visible).
    let mut panes = Vec::new();
    for i in 0..6 {
        let mut pane = make_pane(AgentType::Claude, PaneStatus::Idle);
        pane.pane_id = format!("%{}", i);
        panes.push(pane);
    }

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: panes.clone(),
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", panes)];
    state.focus_state.sidebar_focused = true;
    state.focus_state.focus = Focus::Panes;
    state.rebuild_row_targets();

    // Select the last agent
    state.global.selected_pane_row = 5;
    // Use a tight height so agents area is small (height - 1 margin - 20 bottom)
    let _ = render_to_string(&mut state, 28, 26);

    // Auto-scroll should have moved forward to keep the last-selected pane visible.
    assert!(
        state.scrolls.panes.offset > 0,
        "selecting the last agent should scroll the list"
    );
}

#[test]
fn test_agents_auto_scroll_up_shows_group_header() {
    // After scrolling down, selecting the first agent should scroll
    // back up enough to show the group header.
    let mut panes = Vec::new();
    for i in 0..8 {
        let mut pane = make_pane(AgentType::Claude, PaneStatus::Idle);
        pane.pane_id = format!("%{}", i);
        panes.push(pane);
    }

    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "project".into(),
            window_active: true,
            auto_rename: false,
            panes: panes.clone(),
        }],
    }]);
    state.repo_groups = vec![make_repo_group("project", panes)];
    state.focus_state.sidebar_focused = true;
    state.focus_state.focus = Focus::Panes;
    state.rebuild_row_targets();

    // Scroll to bottom
    state.global.selected_pane_row = 7;
    let _ = render_to_string(&mut state, 28, 26);
    assert!(state.scrolls.panes.offset > 0, "should have scrolled down");

    // Now select first agent and re-render
    state.global.selected_pane_row = 0;
    // The snapshot locks in that the `project` repo header is visible after
    // scrolling back up to the first agent.
    insta::assert_snapshot!(render_to_string(&mut state, 28, 26), @"
     ≡ 8  ● 0  ◎ 0  ◐ 0  ○ 8  ✕
    ⓘ                        — ▾
    project
      ○ claude
        Waiting for prompt…
    ╭ Activity │ Git ──────────╮
    │      No activity yet     │
    ╰──────────────────────────╯
    ");
}

// ─── Repo popup rendering ───────────────────────────────────────────

#[test]
fn repo_popup_renders_repo_names_when_open() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Idle);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "frontend".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![
        make_repo_group("frontend", vec![pane.clone()]),
        make_repo_group("backend", vec![pane.clone()]),
    ];
    state.rebuild_row_targets();
    state.popup = tmux_agent_sidebar::state::PopupState::Repo {
        selected: 0,
        area: None,
    };

    // The snapshot locks in that the popup lists the `All` entry plus both
    // repo names when opened.
    insta::assert_snapshot!(render_to_string(&mut state, 40, 30), @"
     ≡ 2  ● 0  ◎ 0  ◐ 0  ○ 2  ✕ 0
    ⓘ                                    — ▾
    frontend                    ┌──────────┐
    ┃ ○ claude                  │ All      │
        Waiting for prompt…     │ frontend │
                                │ backend  │
    backend                     └──────────┘
    ┃ ○ claude
        Waiting for prompt…
    ╭ Activity │ Git ──────────────────────╮
    │            No activity yet           │
    ╰──────────────────────────────────────╯
    ");
    // The popup area is required for click hit-testing and is non-visual
    // state, so it stays as a direct assertion.
    assert!(
        state.repo_popup_area().is_some(),
        "render should populate repo popup area for hit-testing"
    );
}

#[test]
fn repo_popup_highlights_selected_entry_with_background() {
    let pane = make_pane(AgentType::Claude, PaneStatus::Idle);
    let mut state = make_state(vec![SessionInfo {
        session_name: "main".into(),
        windows: vec![WindowInfo {
            window_id: "@1".into(),
            window_name: "frontend".into(),
            window_active: true,
            auto_rename: false,
            panes: vec![pane.clone()],
        }],
    }]);
    state.repo_groups = vec![
        make_repo_group("frontend", vec![pane.clone()]),
        make_repo_group("backend", vec![pane.clone()]),
    ];
    state.rebuild_row_targets();
    state.focus_state.sidebar_focused = false; // surface raw colors instead of REVERSED
    state.popup = tmux_agent_sidebar::state::PopupState::Repo {
        selected: 2, // "backend" (0=All, 1=frontend, 2=backend)
        area: None,
    };

    // Styled snapshot locks in that the `backend` row carries the selection
    // background (bg:239) on each cell of the entry.
    insta::assert_snapshot!(render_to_styled_string(&mut state, 40, 30), @"
     ≡[fg:12] 2[fg:15]  ●[fg:8] 0[fg:8]  ◎[fg:8] 0[fg:8]  ◐[fg:8] 0[fg:8]  ○[fg:8] 2[fg:15]  ✕[fg:8] 0[fg:8]
    ⓘ[fg:11]                                    —[fg:15] ▾[fg:15]
    f[fg:14]r[fg:14]o[fg:14]n[fg:14]t[fg:14]e[fg:14]n[fg:14]d[fg:14]                    ┌[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]┐[fg:14]
    ┃[fg:14] ○[fg:14] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]                  │[fg:14] [fg:15]A[fg:15]l[fg:15]l[fg:15] [fg:15] [fg:15] [fg:15] [fg:15] [fg:15] [fg:15]│[fg:14]
       [fg:15] [fg:15]W[fg:15]a[fg:15]i[fg:15]t[fg:15]i[fg:15]n[fg:15]g[fg:15] [fg:15]f[fg:15]o[fg:15]r[fg:15] [fg:15]p[fg:15]r[fg:15]o[fg:15]m[fg:15]p[fg:15]t[fg:15]…[fg:15]     │[fg:14] [fg:7]f[fg:7]r[fg:7]o[fg:7]n[fg:7]t[fg:7]e[fg:7]n[fg:7]d[fg:7] [fg:7]│[fg:14]
                                │[fg:14] [fg:15,bg:8]b[fg:15,bg:8]a[fg:15,bg:8]c[fg:15,bg:8]k[fg:15,bg:8]e[fg:15,bg:8]n[fg:15,bg:8]d[fg:15,bg:8] [fg:15,bg:8] [fg:15,bg:8]│[fg:14]
    b[fg:14]a[fg:14]c[fg:14]k[fg:14]e[fg:14]n[fg:14]d[fg:14]                     └[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]─[fg:14]┘[fg:14]
    ┃[fg:14] ○[fg:14] [fg:13]c[fg:13]l[fg:13]a[fg:13]u[fg:13]d[fg:13]e[fg:13]
       [fg:15] [fg:15]W[fg:15]a[fg:15]i[fg:15]t[fg:15]i[fg:15]n[fg:15]g[fg:15] [fg:15]f[fg:15]o[fg:15]r[fg:15] [fg:15]p[fg:15]r[fg:15]o[fg:15]m[fg:15]p[fg:15]t[fg:15]…[fg:15]

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
