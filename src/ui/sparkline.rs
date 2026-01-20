use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Span, Line};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline, Wrap};
use crate::ip_data::IpData;
use crate::ui::point::get_loss_color_and_emoji;
use crate::ui::utils::{calculate_avg_rtt, calculate_jitter, calculate_loss_pkg, draw_errors_section};
use crate::i18n;

pub fn draw_sparkline_view<B: Backend>(
    f: &mut Frame,
    ip_data: &[IpData],
    errs: &[String],
    area: Rect,
    lang: &str,
) {
    let data = ip_data.to_vec();
    let n = data.len().max(1);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            std::iter::once(Constraint::Length(0))
                .chain(std::iter::once(Constraint::Length(2)))
                .chain(std::iter::repeat(Constraint::Length(5)).take(n))
                .chain([Constraint::Min(6)])
                .collect::<Vec<_>>()
        )
        .split(area);

    let legend = Line::from(vec![
        Span::styled(" 🏎  PingWatch SparkLine View ", Style::default().fg(Color::Cyan)),
        Span::raw("("),
        Span::raw(i18n::t(lang, "sparkline-view-note")),
        Span::raw(")"),
    ]);

    let desc_para = Paragraph::new(legend);
    f.render_widget(desc_para, chunks[1]);

    for (i, ip) in data.iter().enumerate() {
        let avg_rtt = calculate_avg_rtt(&ip.rtts);
        let jitter = calculate_jitter(&ip.rtts);
        let loss_pkg = calculate_loss_pkg(ip.timeout, ip.received);
        let loss_pkg_color = get_loss_color_and_emoji(loss_pkg);

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

        let info_para = Paragraph::new(info_line).wrap(Wrap { trim: true });
        f.render_widget(info_para, chunks[i + 2]);

        let spark_rect = Rect {
            x: chunks[i + 2].x,
            y: chunks[i + 2].y + 1,
            width: chunks[i + 2].width,
            height: chunks[i + 2].height.saturating_sub(1),
        };

        let rtts_len = ip.rtts.len();
        let width = spark_rect.width as usize;
        let spark_data: Vec<u64> = ip.rtts
            .iter()
            .skip(rtts_len.saturating_sub(width))
            .map(|&rtt| if rtt < 0.0 { 0 } else { rtt as u64 })
            .collect();

        let spark = Sparkline::default()
            .block(Block::default().borders(Borders::ALL).title("RTT Sparkline"))
            .data(&spark_data)
            .style(Style::default().fg(Color::LightBlue));
        f.render_widget(spark, spark_rect);
    }

    let errors_chunk = chunks.last().unwrap();
    draw_errors_section::<B>(f, errs, *errors_chunk);
}
