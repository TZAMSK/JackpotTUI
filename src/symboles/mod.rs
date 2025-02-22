use ratatui::style::Color;

use crate::iu::constants::{CITRON_ART, DIAMAND_ART};

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
    pub couleur_iu: Color,
}

impl Symbole {
    pub fn dessin(&self) -> &str {
        match self.type_ {
            Type::Citron => CITRON_ART,
            Type::Diamand => DIAMAND_ART,
            _ => "AWD",
        }
    }
}
