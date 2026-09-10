//! AI agent connection state, controls, and user-facing help.

use super::super::*;

#[derive(Clone, Copy, Debug)]
pub(in super::super) struct AgentConnectionLayout {
    pub(in super::super) help_area: Rect,
    pub(in super::super) button_area: Option<Rect>,
    pub(in super::super) links_area: Option<Rect>,
}

pub(in super::super) fn agent_connection_layout(area: Rect) -> AgentConnectionLayout {
    let links_height = if area.height >= 20 { 4 } else { 0 };
    let button_height = if area.height >= 8 {
        3
    } else if area.height >= 4 {
        1
    } else {
        0
    };
    if button_height == 0 {
        return AgentConnectionLayout {
            help_area: area,
            button_area: None,
            links_area: None,
        };
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(button_height),
            Constraint::Length(links_height),
        ])
        .split(area);
    let button_row = rows[1];
    let button_width = button_row.width.min(34);
    let button_x = button_row
        .x
        .saturating_add(button_row.width.saturating_sub(button_width) / 2);
    let button_area = (button_width > 0).then_some(Rect {
        x: button_x,
        y: button_row.y,
        width: button_width,
        height: button_row.height,
    });
    AgentConnectionLayout {
        help_area: rows[0],
        button_area,
        links_area: (links_height > 0).then_some(rows[2]),
    }
}

pub(in super::super) fn agent_connection_button_rect(area: Rect) -> Option<Rect> {
    agent_connection_layout(area).button_area
}

pub(in super::super) fn draw_agent_connection_screen(
    frame: &mut Frame<'_>,
    cli: &Cli,
    area: Rect,
    app: &LabApp,
) {
    let focused = app.focus == LabFocus::Help;
    let layout = agent_connection_layout(area);
    let help = Paragraph::new(scroll_lines_to_panel(
        home_help_agent_lines(cli, app),
        layout.help_area,
        cli,
        app.focused_panel_scroll(LabFocus::Help),
        focused,
    ))
    .block(panel_block(
        cli,
        "connect your AI agent",
        Color::Magenta,
        focused,
    ))
    .wrap(Wrap { trim: true });
    frame.render_widget(help, layout.help_area);

    if let Some(button_area) = layout.button_area {
        let (label, mut color) = match app.mcp_connection_action() {
            McpConnectionAction::Enable => ("ENABLE PETRI MCP", Color::Green),
            McpConnectionAction::Disable => ("DISABLE PETRI MCP", Color::Red),
            McpConnectionAction::Repair => ("REPAIR PETRI MCP", Color::Yellow),
            McpConnectionAction::Blocked => ("EXISTING PETRI MCP FOUND", Color::Yellow),
        };
        if app.guide.focused_control.as_deref() == Some("control:mcp:connection") {
            color = Color::LightYellow;
        }
        frame.render_widget(
            Paragraph::new(raised_button_lines(
                cli,
                label,
                color,
                app.mcp_connection_button_active(),
                button_area.width,
                button_area.height,
            )),
            button_area,
        );
    }

    if let Some(links_area) = layout.links_area {
        let links = Paragraph::new(scroll_lines_to_panel(
            home_help_link_lines(cli, app),
            links_area,
            cli,
            0,
            false,
        ))
        .block(panel_block(cli, "learn more", Color::Cyan, false))
        .wrap(Wrap { trim: true });
        frame.render_widget(links, links_area);
    }
}

pub(in super::super) fn home_help_link_lines(cli: &Cli, app: &LabApp) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled(
                "GitBook: ",
                style(cli, Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                docs_url(),
                style(cli, Color::Cyan).add_modifier(Modifier::UNDERLINED),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Terms of Service: ",
                style(cli, Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                app.wallet_terms.terms_url.clone(),
                style(cli, Color::Yellow).add_modifier(Modifier::UNDERLINED),
            ),
        ]),
    ]
}

pub(in super::super) fn home_help_lines(cli: &Cli, app: &LabApp) -> Vec<Line<'static>> {
    match app.home_help_topic {
        HomeHelpTopic::Overview => home_help_overview_lines(cli),
        HomeHelpTopic::Agents => home_help_agent_lines(cli, app),
    }
}

pub(in super::super) fn home_help_overview_lines(cli: &Cli) -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled(
            "What Amoeba Farm is",
            style(cli, Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Amoeba Farm is a market and oracle system for synthetic exposure to hardware and industrial input markets.",
            style(cli, Color::White),
        )),
        Line::from(Span::styled(
            "It turns scattered hardware price information into defined monthly markets, transparent oracle recipes, and bounded-risk contracts.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "In v1, users trade fixed-risk monthly exposure to a published settlement result, not physical delivery.",
            style(cli, Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "What each home lane does",
            style(cli, Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Trade options: browse capped call and put spreads, then review max loss and max payout before trading.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "View chart: inspect fair price, recent movement, volume, and liquidity for the selected market.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Oracle evidence: see the source trail and settlement rules that decide the monthly result.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Wallet ledger: review recent wallet activity, Amoeba trades, and indexed account history.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Staking: queue AMBA for seven days, activate it into transferable sAMBA, track embedded rewards, or cancel the queue.",
            style(cli, Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Use the links panel for the GitBook and Terms of Service. Open Connect your AI agent from Home for MCP setup.",
            style(cli, Color::DarkGray),
        )),
    ]
}

pub(in super::super) fn home_help_agent_lines(cli: &Cli, app: &LabApp) -> Vec<Line<'static>> {
    let (status_text, status_color) = match app.mcp_connection_state {
        McpConnectionState::Disabled => ("Not enabled on this computer", Color::Gray),
        McpConnectionState::Enabled => ("Enabled on this computer", Color::Green),
        McpConnectionState::NeedsRepair => ("Repair available", Color::Yellow),
        McpConnectionState::Conflict => ("An existing Petri connection was found", Color::Yellow),
    };
    let action_hint = match app.mcp_connection_action() {
        McpConnectionAction::Blocked => "This existing connection was left unchanged.",
        McpConnectionAction::Repair => {
            "Select Repair to diagnose and rebuild Petri's connection in place."
        }
        _ => "Click the button or press Enter. Restart an AI agent that is already open.",
    };
    let mut lines = vec![
        Line::from(Span::styled(
            "Connect your AI agent: Claude Code, Codex, Gemini, and other supported AI agents can use Petri for markets, oracle evidence, and drafts.",
            style(cli, Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("Status: {status_text}"),
            style(cli, status_color).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Enable once. It stays ready after restarts and starts with your assistant.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Nothing needs to keep running while your assistant is closed.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Transactions: unavailable through Petri MCP. Agents can inspect, draft, and validate only.",
            style(cli, Color::Yellow),
        )),
        Line::from(Span::styled(
            "Your key is not copied. Hosted managed signing uses canonical SDK/Spread plans directly, not this CLI subprocess.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(action_hint, style(cli, Color::DarkGray))),
    ];
    if let Some(issue) = app.mcp_connection_issue.as_deref() {
        lines.push(Line::from(Span::styled(
            issue.to_string(),
            style(cli, Color::LightRed).add_modifier(Modifier::BOLD),
        )));
        if app.mcp_repair_failed {
            lines.push(Line::from(Span::styled(
                "If repair still cannot complete, disconnect only Petri's managed connection, then connect it again.",
                style(cli, Color::Yellow).add_modifier(Modifier::BOLD),
            )));
        }
    } else if app.mcp_connection_state == McpConnectionState::NeedsRepair {
        lines.push(Line::from(Span::styled(
            "Select Repair. Petri will check the runtime and owned agent settings, then rebuild only what it manages.",
            style(cli, Color::Yellow).add_modifier(Modifier::BOLD),
        )));
    }
    lines
}

pub(in super::super) fn oracle_help_lines(cli: &Cli, app: &LabApp) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(Span::styled(
            "How to read the oracle",
            style(cli, Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "The main page is for navigation. This page explains the rules behind it.",
            style(cli, Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Settlement rule",
            style(cli, Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Each source is compared with its own opening value. The oracle combines those source-local moves through frozen monthly weights and fixed basket weights.",
            style(cli, Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Phase timeline",
            style(cli, Color::Yellow).add_modifier(Modifier::BOLD),
        )),
    ];

    let active = app.oracle_phase();
    for phase in OraclePhase::ALL {
        let marker = if phase == active { "ACTIVE" } else { "      " };
        lines.push(Line::from(vec![
            Span::styled(format!("{marker} "), style(cli, phase.active_color())),
            Span::styled(
                format!("{}: ", phase.label()),
                style(cli, phase.color()).add_modifier(Modifier::BOLD),
            ),
            Span::styled(phase.detail(), style(cli, Color::Gray)),
        ]));
    }

    lines.extend([
        Line::from(""),
        Line::from(Span::styled(
            "Context actions",
            style(cli, Color::Magenta).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Propose source: source category, canonical locator, source definition, product row, stake/support.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Opening: value, source time, exact canonical URL, matching 14-digit UTC Wayback capture, stake. Updates keep their existing archive-evidence fields.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Challenge: bad source definition, wrong row, stale opening print, or bad live update evidence.",
            style(cli, Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Evidence rules",
            style(cli, Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Allowed V1 sources: public retailer, distributor, manufacturer/store, benchmark/assessment, or public API.",
            style(cli, Color::Gray),
        )),
        Line::from(Span::styled(
            "Not allowed in V1: OTC trades, private invoices, broker DMs, login-only prices, search snippets, auctions, unstable seller listings.",
            style(cli, Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Controls",
            style(cli, Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Enter on the intro opens the tree. Backspace/Left returns from help to the intro. Home/Alt+h returns home.",
            style(cli, Color::Gray),
        )),
    ]);

    lines
}
