use rand::distr::{weighted::WeightedIndex, Distribution};

use crate::symboles::{Symbole, Type};

pub struct Mixeur {
    pub rouleaux: Vec<Symbole>,
}

impl Mixeur {
    pub fn symboles() -> Self {
        Self {
            rouleaux: vec![
                Symbole {
                    type_: Type::Citron,
                    chance: 20,
                },
                Symbole {
                    type_: Type::Cloche,
                    chance: 19,
                },
                Symbole {
                    type_: Type::Cerise,
                    chance: 15,
                },
                Symbole {
                    type_: Type::Bière,
                    chance: 14,
                },
                Symbole {
                    type_: Type::Étoile,
                    chance: 14,
                },
                Symbole {
                    type_: Type::Banane,
                    chance: 21,
                },
                Symbole {
                    type_: Type::Diamant,
                    chance: 7,
                },
            ],
        }
    }

    pub fn mélanger(&self, liste: &Vec<Symbole>) -> Vec<Symbole> {
        let symboles_pondérés =
            match WeightedIndex::new(self.rouleaux.iter().map(|symbole| symbole.chance)) {
                Ok(index) => index,
                Err(_) => return Vec::new(),
            };

        let mut symboles = Vec::new();

        for _ in 0..3 {
            symboles.push(liste[symboles_pondérés.sample(&mut rand::rng())].clone());
        }

        symboles
    }
}

impl Default for Mixeur {
    fn default() -> Self {
        Self { rouleaux: vec![] }
    }
}
