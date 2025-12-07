use crate::app::{App, Mode};
use chrono::{Datelike, Local};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    match app.mode {
        Mode::Calendar => draw_calendar(f, app, chunks[0]),
        Mode::Editor => draw_editor(f, app, chunks[0]),
    }

    draw_status_bar(f, app, chunks[1]);
}

fn draw_calendar(f: &mut Frame, app: &App, area: Rect) {
    let months = app.calendar.get_display_months();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    for (row_idx, row) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(*row);

        for (col_idx, col) in cols.iter().enumerate() {
            let month_idx = row_idx * 3 + col_idx;
            if let Some((year, month)) = months.get(month_idx) {
                draw_month(f, app, *col, *year, *month);
            }
        }
    }
}

fn draw_month(f: &mut Frame, app: &App, area: Rect, year: i32, month: u32) {
    let month_name = match month {
        1 => "1月",
        2 => "2月",
        3 => "3月",
        4 => "4月",
        5 => "5月",
        6 => "6月",
        7 => "7月",
        8 => "8月",
        9 => "9月",
        10 => "10月",
        11 => "11月",
        12 => "12月",
        _ => "?月",
    };

    let title = format!(" {}/{} ", year, month_name);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let days = app.calendar.get_month_days(year, month);
    let today = Local::now().naive_local().date();

    let mut lines = vec![];

    let header = Line::from(vec![
        Span::styled("日 ", Style::default().fg(Color::Red)),
        Span::styled("月 ", Style::default().fg(Color::White)),
        Span::styled("火 ", Style::default().fg(Color::White)),
        Span::styled("水 ", Style::default().fg(Color::White)),
        Span::styled("木 ", Style::default().fg(Color::White)),
        Span::styled("金 ", Style::default().fg(Color::White)),
        Span::styled("土 ", Style::default().fg(Color::Blue)),
    ]);
    lines.push(header);

    let mut week_spans = vec![];
    for (i, day_opt) in days.iter().enumerate() {
        let day_str = if let Some(date) = day_opt {
            format!("{:2} ", date.day())
        } else {
            "   ".to_string()
        };

        let is_selected = day_opt
            .as_ref()
            .map(|d| *d == app.calendar.selected_date)
            .unwrap_or(false);

        let is_today = day_opt.as_ref().map(|d| *d == today).unwrap_or(false);

        let has_entry = day_opt
            .as_ref()
            .map(|d| app.has_entry(d))
            .unwrap_or(false);

        let weekday = i % 7;
        let base_color = match weekday {
            0 => Color::Red,
            6 => Color::Blue,
            _ => Color::White,
        };

        let mut style = Style::default().fg(base_color);

        if is_selected {
            style = style.bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD);
        } else if is_today {
            style = style.add_modifier(Modifier::BOLD).fg(Color::Yellow);
        }

        if has_entry && !is_selected {
            style = style.add_modifier(Modifier::UNDERLINED);
        }

        week_spans.push(Span::styled(day_str, style));

        if (i + 1) % 7 == 0 || i == days.len() - 1 {
            lines.push(Line::from(week_spans.clone()));
            week_spans.clear();
        }
    }

    let calendar_widget = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    f.render_widget(calendar_widget, inner);
}

fn draw_editor(f: &mut Frame, app: &App, area: Rect) {
    let date_str = app.calendar.selected_date.format("%Y-%m-%d (%A)").to_string();
    let title = format!("Diary - {}", date_str);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let display_lines = app.editor.get_display_lines(inner.height as usize);

    let text: Vec<Line> = display_lines
        .iter()
        .map(|line| Line::from(line.clone()))
        .collect();

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, inner);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_text = Line::from(vec![
        Span::styled(&app.status_message, Style::default().fg(Color::Yellow)),
    ]);

    let status = Paragraph::new(status_text)
        .style(Style::default().bg(Color::DarkGray));

    f.render_widget(status, area);
}
