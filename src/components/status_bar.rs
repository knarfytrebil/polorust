use structs::app::AppState;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};

use tui::widgets::{Paragraph};
use tui::text::{Spans, Span};
use tui::Frame;

pub fn render<B>(frame: &mut Frame<B>, app: &AppState, area: Rect)
where
    B: Backend,
{
    let text = Spans::from(vec![
        Span::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(app.mode.symbol.clone(), Style::default().bg(Color::Black)),
        Span::styled(" ", Style::default().fg(Color::White).bg(Color::Black)),
        Span::styled(
            "\u{E0B0} ",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Span::styled(
            "Runing Tasks [0]",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Span::styled(" ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::styled(
            "\u{E0B0}",
            Style::default().fg(Color::White).bg(Color::Black),
        ),
    ]);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, area);
}
