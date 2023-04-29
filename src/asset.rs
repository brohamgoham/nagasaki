use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
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
            Style::default().fg(Color::Magenta),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(
            "
            Press 'p' to access the Planets, 'a' to add new Planet and 'd' to delete
        ",
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan))
            .title("Hercules-Corona Borealis Great Wall - Supercluster")
            .border_type(BorderType::Double),
    );
    home
}

pub fn render_planets<'a>(planet_list_state: &ListState) -> (Option<List<'a>>, Option<Table<'a>>) {
    let planets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::LightCyan))
        .title("Planets")
        .border_type(BorderType::Rounded);

    let planet_list = database::read_data().expect("Could not find Planet in DB");
    let items: Vec<_> = planet_list
        .iter()
        .map(|planet| {
            ListItem::new(Spans::from(vec![Span::styled(
                planet.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    if planet_list.len() == 0 {
        return (None, None);
    }
    let selected_planet = planet_list
        .get(
            planet_list_state
                .selected()
                .expect("Could not get selected Planet"),
        )
        .expect("Planet already Exists")
        .clone();

    let list = List::new(items).block(planets).highlight_style(
        Style::default()
            .bg(Color::Green)
            .fg(Color::LightMagenta)
            .add_modifier(Modifier::BOLD),
    );

    let planet_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_planet.id.to_string())),
        Cell::from(Span::raw(selected_planet.name)),
        Cell::from(Span::raw(selected_planet.category)),
        Cell::from(Span::raw(selected_planet.age.to_string())),
        Cell::from(Span::raw(selected_planet.created_at.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "NASA No.",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "International Space Station Number",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Planet",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age(YEARS)",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow))
            .title("InterPlanetary Details")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(20),
    ]);

    (Some(list), Some(planet_detail))
}
