use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block,
        Borders, 
        BorderType, 
        Cell, 
        List, 
        ListItem, 
        ListState, 
        Paragraph, 
        Table, 
        Row
    },
};

use crate::database;

pub fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Interplanetary Database")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("in")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Planets-CLI",
            Style::default()
                .fg(Color::Magenta),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("
            Press 'p' to access the Planets, 'a' to add new Planet and 'd' to delete
        ")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            borders(Borders::ALL)
            .style(Style::default().fb(Color::Cyan))
            .title("Hercules-Corona Borealis Great Wall - Supercluster")
            .border_type(BorderType::Doubled),
    );
    home
}
