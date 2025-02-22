use ratatui::style::Color;

use crate::iu::constants::{CITRON_ART, DIAMANT_ART};

pub enum Type {
    Citron,
    Cloche,
    Cerise,
    Bière,
    Étoile,
    Banane,
    Diamant,
}

pub struct Symbole {
    pub type_: Type,
}

impl Symbole {
    pub fn données(&self) -> (&str, Color) {
        match self.type_ {
            Type::Citron => (CITRON_ART, Color::Yellow),
            Type::Diamant => (DIAMANT_ART, Color::Blue),
            _ => ("AAA", Color::Red),
        }
    }

    pub fn dessin(&self) -> &str {
        self.données().0
    }

    pub fn couleur(&self) -> Color {
        self.données().1
    }
}
