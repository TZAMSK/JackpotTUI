use std::error::Error;

use crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};

use crate::application::{Application, SaisieMode};

pub type AppResultat<T> = std::result::Result<T, Box<dyn Error>>;

pub fn traiter_événement_clavier(application: &mut Application) -> AppResultat<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match application.saisie_mode {
                SaisieMode::Normale => match key.code {
                    event::KeyCode::Char('q') => return Err("erreur".to_string().into()),
                    event::KeyCode::Char(' ') => application.mélanger_symboles(),
                    event::KeyCode::Char('e') => application.éditer(),
                    _ => {}
                },
                SaisieMode::Édition => match key.code {
                    event::KeyCode::Esc => application.arrêter_édition(),
                    event::KeyCode::Enter => application.soumettre(),
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
