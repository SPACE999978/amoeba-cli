//! Shared raised-button styling and geometry.

use super::super::*;

pub(in super::super) fn terminal_button_key_style(cli: &Cli, color: Color) -> Style {
    if cli.no_color {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        terminal_button_surface_style(cli, color, true)
    }
}

pub(in super::super) fn terminal_button_label_style(cli: &Cli) -> Style {
    if cli.no_color {
        Style::default()
    } else {
        tui_alt_panel_style(cli).fg(Color::White)
    }
}

pub(in super::super) fn terminal_button_surface_style(
    cli: &Cli,
    color: Color,
    active: bool,
) -> Style {
    if cli.no_color {
        let mut style = Style::default().add_modifier(Modifier::BOLD);
        if active {
            style = style.add_modifier(Modifier::REVERSED);
        }
        return style;
    }
    let bg = if active {
        button_active_color(color)
    } else {
        button_fill_color(color)
    };
    Style::default()
        .fg(Color::White)
        .bg(bg)
        .add_modifier(Modifier::BOLD)
}

pub(in super::super) fn raised_button_lines(
    cli: &Cli,
    label: &str,
    color: Color,
    active: bool,
    width: u16,
    height: u16,
) -> Vec<Line<'static>> {
    if width == 0 || height == 0 {
        return Vec::new();
    }
    let label = centered_text(label, width as usize);
    if height == 1 {
        return vec![Line::from(Span::styled(
            label,
            terminal_button_surface_style(cli, color, active),
        ))];
    }

    if cli.no_color {
        let inner_width = width.saturating_sub(2) as usize;
        let top = if width >= 2 {
            format!("┌{}┐", "─".repeat(inner_width))
        } else {
            " ".repeat(width as usize)
        };
        let face = if width >= 2 {
            format!("│{}│", centered_text(label.trim(), inner_width))
        } else {
            label
        };
        let bottom = if width >= 2 {
            format!("└{}┘", "─".repeat(inner_width))
        } else {
            " ".repeat(width as usize)
        };
        let button_style = terminal_button_surface_style(cli, color, active);
        let mut lines = vec![Line::from(Span::styled(top, button_style))];
        if height >= 2 {
            lines.push(Line::from(Span::styled(face, button_style)));
        }
        if height >= 3 {
            lines.push(Line::from(Span::styled(bottom, button_style)));
        }
        return lines;
    }

    let edge = " ".repeat(width as usize);
    let mut lines = Vec::with_capacity(height.min(3) as usize);
    lines.push(Line::from(Span::styled(
        edge.clone(),
        tui_surface_style(cli, button_top_edge_color(color, active)),
    )));
    lines.push(Line::from(Span::styled(
        label,
        terminal_button_surface_style(cli, color, active),
    )));
    if height >= 3 {
        lines.push(Line::from(Span::styled(
            edge,
            tui_surface_style(cli, button_shadow_color(color)),
        )));
    }
    lines
}

pub(in super::super) fn centered_text(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let label_width = text_width(text);
    if label_width >= width {
        return fit_text_to_width(text, width);
    }
    let left = (width - label_width) / 2;
    let right = width - label_width - left;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

pub(in super::super) fn button_fill_color(color: Color) -> Color {
    match color {
        Color::Rgb(126, 78, 38) => Color::Rgb(111, 67, 31),
        Color::Red | Color::LightRed => Color::Rgb(160, 28, 34),
        Color::Green | Color::LightGreen => Color::Rgb(26, 113, 61),
        Color::Yellow | Color::LightYellow => Color::Rgb(172, 132, 18),
        Color::Blue | Color::LightBlue => Color::Rgb(41, 74, 184),
        Color::Magenta | Color::LightMagenta => Color::Rgb(114, 54, 176),
        Color::Cyan | Color::LightCyan => Color::Rgb(0, 120, 148),
        other => other,
    }
}

pub(in super::super) fn button_active_color(color: Color) -> Color {
    match color {
        Color::Rgb(126, 78, 38) => Color::Rgb(147, 88, 41),
        Color::Red | Color::LightRed => Color::Rgb(192, 37, 44),
        Color::Green | Color::LightGreen => Color::Rgb(33, 139, 75),
        Color::Yellow | Color::LightYellow => Color::Rgb(207, 160, 26),
        Color::Blue | Color::LightBlue => Color::Rgb(55, 96, 220),
        Color::Magenta | Color::LightMagenta => Color::Rgb(140, 69, 214),
        Color::Cyan | Color::LightCyan => Color::Rgb(0, 149, 181),
        other => other,
    }
}

pub(in super::super) fn button_top_edge_color(color: Color, active: bool) -> Color {
    match color {
        Color::Rgb(126, 78, 38) if active => Color::Rgb(196, 128, 68),
        Color::Rgb(126, 78, 38) => Color::Rgb(164, 99, 48),
        Color::Red | Color::LightRed if active => Color::Rgb(238, 61, 68),
        Color::Red | Color::LightRed => Color::Rgb(203, 42, 49),
        Color::Green | Color::LightGreen if active => Color::Rgb(62, 204, 109),
        Color::Green | Color::LightGreen => Color::Rgb(42, 159, 84),
        Color::Yellow | Color::LightYellow if active => Color::Rgb(245, 204, 60),
        Color::Yellow | Color::LightYellow => Color::Rgb(212, 169, 33),
        Color::Blue | Color::LightBlue if active => Color::Rgb(88, 133, 255),
        Color::Blue | Color::LightBlue => Color::Rgb(59, 102, 223),
        Color::Magenta | Color::LightMagenta if active => Color::Rgb(177, 99, 255),
        Color::Magenta | Color::LightMagenta => Color::Rgb(145, 72, 219),
        Color::Cyan | Color::LightCyan if active => Color::Rgb(43, 213, 232),
        Color::Cyan | Color::LightCyan => Color::Rgb(0, 164, 191),
        other => other,
    }
}

pub(in super::super) fn button_shadow_color(color: Color) -> Color {
    match color {
        Color::Rgb(126, 78, 38) => Color::Rgb(67, 39, 22),
        Color::Red | Color::LightRed => Color::Rgb(92, 18, 25),
        Color::Green | Color::LightGreen => Color::Rgb(13, 71, 40),
        Color::Yellow | Color::LightYellow => Color::Rgb(111, 83, 13),
        Color::Blue | Color::LightBlue => Color::Rgb(19, 42, 112),
        Color::Magenta | Color::LightMagenta => Color::Rgb(66, 31, 111),
        Color::Cyan | Color::LightCyan => Color::Rgb(0, 73, 94),
        other => other,
    }
}
