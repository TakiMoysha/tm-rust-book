use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};

use crate::app::{App, InputMode};

/// Main render function
pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // Header with input
    draw_header(frame, app, chunks[0]);

    // Main content area
    if app.parse_error.is_some() {
        draw_error(frame, app, chunks[1]);
    } else if !app.definitions.is_empty() {
        draw_content(frame, app, chunks[1]);
    } else {
        draw_empty(frame, app, chunks[1]);
    }

    // Draw suggestions popup if active
    if app.show_suggestions {
        draw_suggestions_popup(frame, app);
    }

    // Footer with help
    draw_footer(frame, app, chunks[2]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let mode_style = match app.input_mode {
        InputMode::Normal => Style::default().fg(Color::Green),
        InputMode::Insert => Style::default().fg(Color::Yellow),
    };

    let mode_text = match app.input_mode {
        InputMode::Normal => " NORMAL ",
        InputMode::Insert => " INSERT ",
    };

    let input_text = app.file_path.clone();

    let input = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(mode_style)
                .title(Line::from(vec![
                    Span::styled(mode_text, mode_style.add_modifier(Modifier::BOLD)),
                    Span::raw(" File path "),
                ]))
                .title_bottom(Line::from(vec![Span::raw("i:edit  Enter:load  r:reload")])),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(input, area);

    // Render cursor in insert mode
    if app.input_mode == InputMode::Insert {
        let x = area.x + 1 + app.cursor_position as u16;
        let y = area.y + 1;
        frame.set_cursor_position((x, y));
    }
}

fn draw_content(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    // Left panel - list of definitions
    draw_definition_list(frame, app, chunks[0]);

    // Right panel - details
    draw_definition_details(frame, app, chunks[1]);
}

fn draw_definition_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .definitions
        .iter()
        .map(|def| {
            let text = format!("{} [{}]", def.id.subtype_id, def.id.type_id);
            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Blocks ({})", app.definitions.len())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn draw_definition_details(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Details ");

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if let Some(def) = app.selected_definition() {
        let text = format_definition_details(def);
        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .scroll((0, 0));

        frame.render_widget(paragraph, inner_area);
    } else {
        let placeholder = Paragraph::new("Select a block to view details")
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(placeholder, inner_area);
    }
}

fn format_definition_details(def: &playground_se::types::Definition) -> String {
    let mut lines = vec![
        format!("Display Name: {}", def.display_name),
        format!("Type ID: {}", def.id.type_id),
        format!("Subtype ID: {}", def.id.subtype_id),
        format!("Cube Size: {}", def.cube_size),
        format!("Block Topology: {}", def.block_topology),
        format!("Size: {}x{}x{}", def.size.x, def.size.y, def.size.z),
        format!("Block Pair Name: {}", def.block_pair_name),
        format!("Build Time: {}s", def.build_time_seconds),
        format!("Edge Type: {}", def.edge_type),
    ];

    if let Some(model) = &def.model {
        lines.push(format!("Model: {}", model));
    }

    if let Some(critical) = &def.critical_component {
        lines.push(format!(
            "\nCritical Component: {} (index: {})",
            critical.subtype, critical.index
        ));
    }
    if let Some(components) = &def.components {
        lines.push(String::from("\nComponents:"));
        for comp in &components.components {
            lines.push(format!("  - {} x{}", comp.subtype, comp.count));
        }
    }

    lines.join("\n")
}

fn draw_error(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(error) = &app.parse_error {
        let error_text = Paragraph::new(error.clone())
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Error ")
                    .border_style(Style::default().fg(Color::Red)),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(error_text, area);
    }
}

fn draw_empty(frame: &mut Frame, _app: &App, area: Rect) {
    let text = Paragraph::new("Enter a file path and press Enter to load")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(text, area);
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let help_text = if app.show_suggestions {
        "Tab:complete | ↑↓:navigate | Enter:accept | Right:enter dir | Esc:cancel"
    } else {
        match app.input_mode {
            InputMode::Normal => "i:insert | j/↓:down | k/↑:up | g:first | G:last | r:reload | q:quit",
            InputMode::Insert => "Tab:suggest | Enter:load | ←/→:move | Home:start | End:end | Esc:normal",
        }
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(footer, area);
}

/// Draw suggestions float popup
fn draw_suggestions_popup(frame: &mut Frame, app: &App) {
    let area = frame.area();
    // Fixed size: 70% width, 70% height
    let popup_area = centered_rect(70, 70, area);

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Split popup into input and list areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(popup_area);

    // Input area at top of popup
    let input_text = app.suggestion_input.clone();
    let input = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Path ")
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(input, chunks[0]);

    // Suggestions list
    if app.suggestions.is_empty() {
        let empty_text = Paragraph::new("No matches")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(empty_text, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .suggestions
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let icon = if item.is_dir { "📁" } else { "📄" };
                let style = if idx == app.selected_suggestion {
                    Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
                } else if item.is_sbc {
                    Style::default().fg(Color::Green)
                } else if item.is_dir {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };
                
                let text = format!("{} {}", icon, item.name);
                ListItem::new(text).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} items ", app.suggestions.len())),
            )
            .highlight_symbol("> ");

        // Create mutable state for the selected item
        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(Some(app.selected_suggestion));
        
        frame.render_stateful_widget(list, chunks[1], &mut list_state);
    }

    // Set cursor in the input field
    let cursor_x = popup_area.x + 1 + app.suggestion_input.len() as u16;
    let cursor_y = popup_area.y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));
}

/// Draw a popup for detailed view (optional)
pub fn draw_popup(frame: &mut Frame, content: &str, title: &str) {
    let area = frame.area();
    let popup_area = centered_rect(60, 60, area);

    frame.render_widget(Clear, popup_area);

    let paragraph = Paragraph::new(content.to_string())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
