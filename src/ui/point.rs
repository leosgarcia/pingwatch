use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Paragraph, Wrap};
use crate::ip_data::IpData;
use crate::ui::utils::{calculate_avg_rtt, calculate_jitter, calculate_loss_pkg, draw_errors_section};
use crate::i18n;

pub fn get_loss_color_and_emoji(loss_rate: f64) -> Color {
    if loss_rate > 50.0 {
        Color::Red
    } else if loss_rate > 0.0 {
        Color::Yellow
    } else {
        Color::Green
    }
}

pub fn draw_point_view<B: Backend>(
    f: &mut Frame,
    ip_data: &[IpData],
    errs: &[String],
    area: Rect,
    lang: &str,
) {
    let data = ip_data.to_vec();

    // Calculate how much vertical space each IP will take (increased to account for multiple lines if needed)
    let ip_height = 5; // Increased to accommodate potential multi-line display
    let total_height = (data.len() * ip_height) + 2; // +2 for margins

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(total_height as u16),
            Constraint::Min(6),        // For errors section
        ].as_ref())
        .split(area);

    // draw legend
    let legend = Line::from(vec![
        Span::styled(" 🏎  PingWatch Point View ", Style::default().fg(Color::Cyan)),
        Span::raw("("),
        Span::styled("•", Style::default().fg(Color::Green)),
        Span::raw(format!(" {}, ", i18n::t(lang, "point-view-legend"))),
        Span::styled("↑", Style::default().fg(Color::Yellow)),
        Span::raw(format!(" {} (over 80% of max), ", i18n::t(lang, "point-view-high-latency"))),
        Span::styled("✗", Style::default().fg(Color::Red)),
        Span::raw(format!(" {})", i18n::t(lang, "point-view-timeout"))),
    ]);

    let legend_paragraph = Paragraph::new(legend);
    f.render_widget(legend_paragraph, chunks[0]);

    let ip_area = chunks[1];

    let ip_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(ip_height as u16); data.len()])
        .margin(1)
        .split(ip_area);


    for (i, ip) in data.iter().enumerate() {
        let avg_rtt = calculate_avg_rtt(&ip.rtts);
        let jitter = calculate_jitter(&ip.rtts);
        let loss_pkg = calculate_loss_pkg(ip.timeout, ip.received);
        let loss_pkg_color = get_loss_color_and_emoji(loss_pkg);


        // Create the info line (row 1) with all metrics from table view
        let info_line = Line::from(vec![
            Span::raw(format!("{}: ", i18n::t(lang, "label-target"))),
            Span::styled(format!("{} ", ip.addr), Style::default().fg(Color::Green)),
            Span::raw(format!("{}: ", i18n::t(lang, "label-ip"))),
            Span::styled(format!("{} ", ip.ip), Style::default().fg(Color::Green)),
            Span::raw(format!("{}: ", i18n::t(lang, "label-last-rtt"))),
            Span::styled(
                if ip.last_attr == 0.0 {
                    i18n::t(lang, "metric-less-than")
                } else if ip.last_attr == -1.0 {
                    i18n::t(lang, "metric-zero")
                } else {
                    format!("{:.2}{}", ip.last_attr, i18n::t(lang, "unit-ms"))
                },
                Style::default().fg(Color::Green)
            ),
            Span::raw(format!(" {}: ", i18n::t(lang, "label-avg-rtt"))),
            Span::styled(format!("{:.2}{}", avg_rtt, i18n::t(lang, "unit-ms")), Style::default().fg(Color::Green)),
            Span::raw(format!(" {}: ", i18n::t(lang, "label-max"))),
            Span::styled(format!("{:.2}{}", ip.max_rtt, i18n::t(lang, "unit-ms")), Style::default().fg(Color::Green)),
            Span::raw(format!(" {}: ", i18n::t(lang, "label-min"))),
            Span::styled(format!("{:.2}{}", ip.min_rtt, i18n::t(lang, "unit-ms")), Style::default().fg(Color::Green)),
            Span::raw(format!(" {}: ", i18n::t(lang, "label-jitter"))),
            Span::styled(format!("{:.2}{}", jitter, i18n::t(lang, "unit-ms")), Style::default().fg(Color::Green)),
            Span::raw(format!(" {}: ", i18n::t(lang, "label-loss"))),
            Span::styled(format!("{:.2}{}", loss_pkg, i18n::t(lang, "unit-percent")), Style::default().fg(loss_pkg_color)), 
        ]);
        for &rtt in &ip.rtts {
            if rtt < 0.0 {
                // Timeout/packet loss - red X
                points_spans.push(Span::styled("✗", Style::default().fg(Color::Red)));
            } else if rtt > ip.max_rtt * 0.8 {
                // High latency () - yellow dot
                points_spans.push(Span::styled("↑", Style::default().fg(Color::Yellow)));
            } else {
                // Normal latency - green dot
                points_spans.push(Span::styled("•", Style::default().fg(Color::Green)));
            }
            points_spans.push(Span::raw(" ")); // Space between points
        }

        // Create the block for this IP's data
        let ip_block = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Info line
                Constraint::Min(4), // Points area with enough space for multiple lines
            ].as_ref())
            .split(ip_chunks[i]);

        // Render the info line
        let info_paragraph = Paragraph::new(info_line).wrap(Wrap { trim: true });
        f.render_widget(info_paragraph, ip_block[0]);


        let points_line = Line::from(points_spans);
        let points_paragraph = Paragraph::new(points_line)
            .wrap(Wrap { trim: true })     // Use false to avoid trimming spaces
            .block(Block::default());       // Adding a block can help with wrapping behavior

        f.render_widget(points_paragraph, ip_block[1]);
    }

    // Draw errors section at the bottom
    let errors_chunk = chunks[1];
    draw_errors_section::<B>(f, errs, errors_chunk);
}