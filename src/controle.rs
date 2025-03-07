use std::error::Error;

use crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};

use crate::application::Application;

pub type AppResultat<T> = std::result::Result<T, Box<dyn Error>>;

pub fn traiter_événement_clavier(application: &mut Application) -> AppResultat<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                event::KeyCode::Char('q') => return Err("erreur".to_string().into()),
                event::KeyCode::Char(' ') => application.mélanger_symboles(),
                _ => {}
            }
        }
    }

    Ok(())
}
