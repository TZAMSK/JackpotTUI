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
            WeightedIndex::new(liste.iter().map(|symbole| symbole.chance)).unwrap();

        let mut symboles = Vec::new();

        for _ in 0..3 {
            symboles.push(liste[symboles_pondérés.sample(&mut rand::rng())].clone());
        }

        symboles
    }

    pub fn mélanger_symboles() -> Vec<Symbole> {
        let mixeur = Self::symboles();
        return mixeur.mélanger(&mixeur.rouleaux);
    }
}

impl Default for Mixeur {
    fn default() -> Self {
        Self { rouleaux: vec![] }
    }
}
