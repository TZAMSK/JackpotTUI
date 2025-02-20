use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::iu::constants::{CONTENUE, CONTROLES, SYMBOLES, TITRE, TITRE_APPLICATION};

pub fn afficher_machine(frame: &mut Frame, zone_principal: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(2, 10),
                Constraint::Ratio(6, 10),
                Constraint::Ratio(1, 10),
                Constraint::Ratio(1, 10),
            ]
            .as_ref(),
        )
        .split(zone_principal);

    let titre_application = Paragraph::new(TITRE_APPLICATION)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(titre_application, main_layout[0]);

    let reel_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(main_layout[1]);

    for (index, symbole) in SYMBOLES.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(symbole.to_string())
                .alignment(Alignment::Center)
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::QuadrantInside)
                        .style(Style::default().fg(Color::Magenta)),
                ),
            reel_layout[index],
        );
    }

    let info_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(2, 7),
                Constraint::Ratio(2, 7),
                Constraint::Ratio(3, 7),
            ]
            .as_ref(),
        )
        .split(main_layout[2]);

    for ((index, titre), contenue) in TITRE.iter().enumerate().zip(CONTENUE.iter()) {
        let couleur = match index {
            0 | 1 => Color::Yellow,
            _ => Color::Green,
        };

        frame.render_widget(
            Paragraph::new(contenue.to_string()).block(
                Block::new()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(couleur))
                    .title(titre.to_string())
                    .title_alignment(Alignment::Center),
            ),
            info_layout[index],
        );
    }

    frame.render_widget(Paragraph::new(CONTROLES), main_layout[3])
}
