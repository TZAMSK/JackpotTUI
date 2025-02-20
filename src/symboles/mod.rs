use crate::iu::constants::CITRON_ART;
use crossterm::style::{Color, Stylize};

pub enum Type {
    Citron,
    Cloche,
    Cerise,
    Bière,
    Étoile,
    Banane,
    Diamand,
}

pub struct Symbole {
    pub type_: Type,
}

impl Symbole {
    pub fn dessin(&self) -> String {
        match self.type_ {
            Type::Citron => CITRON_ART.with(Color::Yellow).to_string(),
            _ => "awd".to_string(),
        }
    }
}
