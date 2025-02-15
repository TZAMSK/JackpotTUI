use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn afficher_machine(frame: &mut Frame, zone_principal: Rect) {
    let disposition_principale = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(3, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
            ]
            .as_ref(),
        )
        .split(zone_principal);

    let disposition_machine = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(disposition_principale[0]);

    let rouleaux = ["🍒", "🍋", "🔔"];
    for (i, rouleau) in disposition_machine.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(rouleaux[i])
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Yellow)),
            *rouleau,
        );
    }

    let disposition_info = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(disposition_principale[1]);

    frame.render_widget(
        Paragraph::new("🍒🍒🍒 = 10%\n🍋🍋🍋 = 5%\n🔔🔔🔔 = 2%")
            .block(Block::default().borders(Borders::ALL).title("Possibilités"))
            .alignment(Alignment::Left),
        disposition_info[0],
    );

    frame.render_widget(
        Paragraph::new("🍒🍒🍒 = $100\n🍋🍋🍋 = $200\n🔔🔔🔔 = $500")
            .block(Block::default().borders(Borders::ALL).title("Paiements"))
            .alignment(Alignment::Left),
        disposition_info[1],
    );

    let disposition_montants_action = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(4, 10),
                Constraint::Ratio(3, 10),
                Constraint::Ratio(3, 10),
            ]
            .as_ref(),
        )
        .split(disposition_principale[2]);

    frame.render_widget(
        Paragraph::new("Wagered: $10")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        disposition_montants_action[0],
    );

    frame.render_widget(
        Paragraph::new("Total: $1000")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        disposition_montants_action[1],
    );

    frame.render_widget(
        Paragraph::new("[ESPACE] TOURNER")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Green)),
            )
            .alignment(Alignment::Center),
        disposition_montants_action[2],
    );
}
