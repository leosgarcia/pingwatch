use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Paragraph, Row, Table};
use crate::ip_data::IpData;
use crate::ui::utils::{calculate_avg_rtt, calculate_jitter, calculate_loss_pkg, draw_errors_section};
use crate::i18n;


pub fn draw_table_view<B: Backend>(
    f: &mut Frame,
    ip_data: &[IpData],
    errs: &[String],
    area: Rect,
    lang: &str,
) {
    let mut data = ip_data.to_vec();

    data.sort_by(|a, b| {
        let loss_a = calculate_loss_pkg(a.timeout, a.received);
        let loss_b = calculate_loss_pkg(b.timeout, b.received);

        // sort by loss rate first, then by latency
        match loss_a.partial_cmp(&loss_b) {
            Some(std::cmp::Ordering::Equal) => {
                let avg_a = calculate_avg_rtt(&a.rtts);
                let avg_b = calculate_avg_rtt(&b.rtts);
                avg_a.partial_cmp(&avg_b).unwrap_or(std::cmp::Ordering::Equal)
            }
            Some(ordering) => ordering,
            None => std::cmp::Ordering::Equal
        }
    });


    let header_style = Style::default()
        .add_modifier(Modifier::BOLD);

    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED);

    // create header
    let header = Row::new(vec![
        i18n::t(lang, "label-rank"),
        i18n::t(lang, "label-target"),
        i18n::t(lang, "label-ip"),
        i18n::t(lang, "label-last-rtt"),
        i18n::t(lang, "label-avg-rtt"),
        i18n::t(lang, "label-max"),
        i18n::t(lang, "label-min"),
        i18n::t(lang, "label-jitter"),
        i18n::t(lang, "label-loss"),
    ])
        .style(header_style)
        .height(1);


    // create rows
    let rows = data.iter().enumerate().map(|(index, data)| {
        let avg_rtt = calculate_avg_rtt(&data.rtts);
        let jitter = calculate_jitter(&data.rtts);
        let loss_pkg = calculate_loss_pkg(data.timeout, data.received);

        let rank = match index {
            0 => i18n::t(lang, "rank-first"),
            1 => i18n::t(lang, "rank-second"),
            2 => i18n::t(lang, "rank-third"),
            n if n < 10 && n != ip_data.len() - 1 => i18n::t(lang, "rank-top-10"),
            _ => i18n::t(lang, "rank-slow"),
        };

        let row = Row::new(vec![
            rank,
            data.addr.clone(),
            data.ip.clone(),
            if data.last_attr == 0.0 {
                i18n::t(lang, "metric-less-than")
            } else if data.last_attr == -1.0 {
                i18n::t(lang, "metric-zero")
            } else {
                format!("{:.2}{}", data.last_attr, i18n::t(lang, "unit-ms"))
            },
            format!("{:.2}{}", avg_rtt, i18n::t(lang, "unit-ms")),
            format!("{:.2}{}", data.max_rtt, i18n::t(lang, "unit-ms")),
            format!("{:.2}{}", data.min_rtt, i18n::t(lang, "unit-ms")),
            format!("{:.2}{}", jitter, i18n::t(lang, "unit-ms")),
            format!("{:.2}{}", loss_pkg, i18n::t(lang, "unit-percent")),
        ]).height(1);

        // highlight the row with different colors
        if loss_pkg > 50.0 {
            row.style(Style::default().bg(Color::Red).fg(Color::White)) // Light red color
        } else if loss_pkg > 0.0 {
            row.style(Style::default().bg(Color::Yellow).fg(Color::White)) // Light yellow color
        } else {
            row
        }
    });


    let table = Table::new(
        rows,
        [
            Constraint::Percentage(3),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ],
    )
        .header(header)
        .block(Block::default()
            .title("🏎  PingWatch Table (Sort by: Loss Rate ↑ then Latency ↑)"))
        .row_highlight_style(selected_style)
        .highlight_symbol(">> ");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(5),
            Constraint::Length(6),
        ].as_ref())
        .split(area);

    // black line
    let blank = Paragraph::new("");
    f.render_widget(blank, chunks[0]);
    f.render_widget(table, chunks[1]);

    let errors_chunk = chunks.last().unwrap();
    draw_errors_section::<B>(f, errs, *errors_chunk);
}