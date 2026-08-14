use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::ctx::RowCtx;
use crate::tmux::PaneStatus;
use crate::ui::icons::StatusIcons;
use crate::ui::text::{display_width, elapsed_label, truncate_to_width};

pub(super) fn status_row(
    pane: &crate::tmux::PaneInfo,
    ctx: &RowCtx,
    icons: &StatusIcons,
    spinner_frame: usize,
    now: u64,
) -> Line<'static> {
    use crate::tmux::PermissionMode;
    let theme = ctx.theme;

    let (icon, pulse_color) = running_icon_for(&pane.status, spinner_frame, icons, theme);
    let icon_color =
        pulse_color.unwrap_or_else(|| theme.status_color(&pane.status, pane.attention));
    let title_raw: &str = if pane.session_name.is_empty() {
        pane.agent.label()
    } else {
        &pane.session_name
    };
    let badge = pane.permission_mode.badge();
    let elapsed = elapsed_label(pane.started_at, now);

    let title_fg = theme.agent_color(&pane.agent);
    let elapsed_fg = if pane.status.is_active() {
        theme.text_active
    } else {
        theme.text_muted
    };

    let (model_extracted, effort_extracted) = match pane.model.as_deref() {
        Some(m) if !m.is_empty() => {
            crate::tool_name::split_model_and_effort(m, pane.effort.as_deref())
        }
        _ => (String::new(), pane.effort.clone()),
    };

    let formatted_model = if model_extracted.is_empty() {
        None
    } else {
        Some(format_model_name(&model_extracted))
    };
    let formatted_effort = effort_extracted
        .map(|e| e.to_lowercase())
        .filter(|e| !e.is_empty());

    let mut model_part_width = 0;
    if let Some(ref m) = formatted_model {
        model_part_width += 1 + display_width(m); // '/' + model
        if let Some(ref e) = formatted_effort {
            model_part_width += 1 + display_width(e); // '/' + effort
        }
    }

    let badge_extra = if badge.is_empty() { 0 } else { 1 };
    let fixed_width =
        display_width(icon) + 1 + model_part_width + badge_extra + display_width(badge);
    // User-supplied session names (set via `/rename`) can be arbitrarily
    // long; cap the title to the space left after reserving room for the
    // icon, model/effort, badge, and elapsed label so they stay visible
    // instead of being pushed off-screen.
    let elapsed_width = display_width(&elapsed);
    let elapsed_gap = usize::from(elapsed_width > 0);
    let title_budget = ctx
        .inner_width
        .saturating_sub(fixed_width + elapsed_gap + elapsed_width);
    let title = truncate_to_width(title_raw, title_budget);

    let left_width = fixed_width + display_width(&title);
    let available_for_elapsed = ctx.inner_width.saturating_sub(left_width);
    let elapsed = truncate_to_width(&elapsed, available_for_elapsed);
    let elapsed_width = display_width(&elapsed);

    let mut left_spans: Vec<Span<'static>> = Vec::with_capacity(8);
    left_spans.push(Span::styled(
        icon.to_string(),
        ctx.apply_bg(Style::default().fg(icon_color)),
    ));
    left_spans.push(Span::styled(
        format!(" {}", title),
        ctx.apply_bg(Style::default().fg(title_fg)),
    ));

    if let Some(m) = formatted_model {
        left_spans.push(Span::styled(
            "/".to_string(),
            ctx.apply_bg(Style::default().fg(Color::Indexed(7))),
        ));
        left_spans.push(Span::styled(
            m,
            ctx.apply_bg(Style::default().fg(Color::Indexed(15))),
        ));
        if let Some(e) = formatted_effort {
            let color = effort_color(&e);
            left_spans.push(Span::styled(
                "/".to_string(),
                ctx.apply_bg(Style::default().fg(Color::Indexed(7))),
            ));
            left_spans.push(Span::styled(e, ctx.apply_bg(Style::default().fg(color))));
        }
    }

    if !badge.is_empty() {
        let badge_color = match pane.permission_mode {
            PermissionMode::BypassPermissions => theme.badge_danger,
            PermissionMode::Auto => theme.badge_auto,
            PermissionMode::DontAsk => theme.badge_auto,
            PermissionMode::Plan => theme.badge_plan,
            PermissionMode::AcceptEdits => theme.badge_auto,
            PermissionMode::Defer => theme.badge_auto,
            PermissionMode::Default => theme.text_muted,
        };
        left_spans.push(Span::styled(
            format!(" {}", badge),
            ctx.apply_bg(Style::default().fg(badge_color)),
        ));
    }

    let right_spans = vec![Span::styled(
        elapsed,
        ctx.apply_bg(Style::default().fg(elapsed_fg)),
    )];

    ctx.row_line_split(left_spans, left_width, right_spans, elapsed_width)
}

fn format_model_name(raw: &str) -> String {
    raw.to_lowercase().replace(' ', "")
}

fn effort_color(effort: &str) -> Color {
    match effort.trim().to_lowercase().as_str() {
        "low" => Color::Indexed(2),            // Green
        "medium" | "med" => Color::Indexed(3), // Yellow
        _ => Color::Indexed(1),                // Red for high / max / xhigh etc.
    }
}

pub(super) fn running_icon_for<'a>(
    status: &PaneStatus,
    spinner_frame: usize,
    icons: &'a StatusIcons,
    theme: &crate::ui::colors::ColorTheme,
) -> (&'a str, Option<Color>) {
    use crate::SPINNER_PULSE;

    match status {
        PaneStatus::Running => {
            if let Some(custom) = theme.running_spinner {
                let color = if spinner_frame.is_multiple_of(2) {
                    custom
                } else {
                    theme.status_running
                };
                (icons.status_icon(status), Some(color))
            } else {
                let color_idx = SPINNER_PULSE[spinner_frame % SPINNER_PULSE.len()];
                (icons.status_icon(status), Some(Color::Indexed(color_idx)))
            }
        }
        _ => (icons.status_icon(status), None),
    }
}
