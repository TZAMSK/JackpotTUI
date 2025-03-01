pub mod mixeur;

use ratatui::style::Color;

use crate::iu::constants::{
    BANANE_ART, BIÈRE_ART, CERISE_ART, CITRON_ART, CLOCHE_ART, DIAMANT_ART, ÉTOILE_ART,
};

#[derive(Clone)]
pub enum Type {
    Citron,
    Cloche,
    Cerise,
    Bière,
    Étoile,
    Banane,
    Diamant,
}

#[derive(Clone)]
pub struct Symbole {
    pub type_: Type,
    pub chance: u8,
}

impl Symbole {
    pub fn données(&self) -> (&str, Color) {
        match self.type_ {
            Type::Citron => (CITRON_ART, Color::Yellow),
            Type::Cloche => (CLOCHE_ART, Color::Gray),
            Type::Cerise => (CERISE_ART, Color::Red),
            Type::Bière => (BIÈRE_ART, Color::Indexed(95)),
            Type::Étoile => (ÉTOILE_ART, Color::LightYellow),
            Type::Banane => (BANANE_ART, Color::Indexed(178)),
            Type::Diamant => (DIAMANT_ART, Color::Blue),
        }
    }

    pub fn dessin(&self) -> &str {
        self.données().0
    }

    pub fn couleur(&self) -> Color {
        self.données().1
    }
}
