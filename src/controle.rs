use std::error::Error;

use crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};
use tui_input::backend::crossterm::EventHandler;

use crate::application::{Application, SaisieMode, TypeContextuel};

pub type AppResultat<T> = std::result::Result<T, Box<dyn Error>>;

pub fn traiter_événement_clavier(application: &mut Application) -> AppResultat<()> {
    let event = event::read()?;

    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match application.saisie_mode {
                SaisieMode::Normale => match key.code {
                    event::KeyCode::Char('q') => return Err("erreur".to_string().into()),
                    event::KeyCode::Char(' ') => application.mélanger_symboles(),
                    event::KeyCode::Char('e') => application.éditer(TypeContextuel::Totale),
                    event::KeyCode::Char('w') => application.éditer(TypeContextuel::Mise),
                    _ => {}
                },
                SaisieMode::Édition => match key.code {
                    event::KeyCode::Esc => application.arrêter_édition(),
                    event::KeyCode::Enter => application.soumettre(),
                    _ => {
                        application.saisie.handle_event(&event);
                    }
                },
            }
        }
    }

    Ok(())
}
