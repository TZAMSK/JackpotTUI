use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
    Frame,
};

use crate::{
    application::{Application, SaisieMode, TypeContextuel},
    iu::constants::{TITRE, TITRE_APPLICATION},
};

pub fn afficher_machine(frame: &mut Frame, zone_principal: Rect, application: &mut Application) {
    let layout_principal = Layout::default()
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
    frame.render_widget(titre_application, layout_principal[0]);

    let layout_rouleaux = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(layout_principal[1]);

    for (index, symbole) in application.symboles.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(symbole.dessin().to_string())
                .alignment(Alignment::Center)
                .block(
                    Block::new()
                        .padding(Padding::uniform(8))
                        .borders(Borders::ALL)
                        .border_type(BorderType::QuadrantInside)
                        .style(Style::default().fg(symbole.couleur())),
                ),
            layout_rouleaux[index],
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
        .split(layout_principal[2]);

    for ((index, titre), contenue) in TITRE
        .iter()
        .enumerate()
        .zip(application.montant.affichier().iter())
    {
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

    frame.render_widget(
        Paragraph::new(application.contrôle_indices()),
        layout_principal[3],
    );

    if application.affichage_contextuel {
        match application.type_contextuel {
            TypeContextuel::Mise => {
                afficher_fenêtre_contextuelle(frame, zone_principal, application, "Changer mise")
            }
            TypeContextuel::Totale => {
                afficher_fenêtre_contextuelle(frame, zone_principal, application, "Changer totale")
            }
        }
    }
}

pub fn afficher_fenêtre_contextuelle(
    frame: &mut Frame,
    zone_principal: Rect,
    application: &mut Application,
    titre: &str,
) {
    let fenêtre_zone = centrer_rect(20, 7, zone_principal);

    let défilement_saisie = application
        .saisie
        .visual_scroll(fenêtre_zone.width as usize);

    let fenêtre_block = Block::default()
        .title(titre.to_string())
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::new().fg(Color::Yellow).bg(Color::DarkGray));

    let fenêtre_contenu = Paragraph::new(application.saisie.value()).block(fenêtre_block);

    frame.render_widget(Clear, fenêtre_zone);
    frame.render_widget(fenêtre_contenu, fenêtre_zone);

    if application.saisie_mode == SaisieMode::Édition {
        let x = application.saisie.visual_cursor().max(défilement_saisie) - défilement_saisie + 1;
        frame.set_cursor_position((fenêtre_zone.x + x as u16, fenêtre_zone.y + 1));
    }
}

pub fn centrer_rect(pourcentage_x: u16, pourcentage_y: u16, r: Rect) -> Rect {
    let longueur = r.width * pourcentage_x / 100;
    let hauteur = r.height * pourcentage_y / 100;
    let marge_droit_gauche = (r.width - longueur) / 2;
    let marge_haut_bas = (r.height - hauteur) / 2;
    Rect {
        x: marge_droit_gauche,
        y: marge_haut_bas,
        width: longueur,
        height: hauteur,
    }
}
