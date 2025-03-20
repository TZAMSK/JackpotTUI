use rand::distr::{weighted::WeightedIndex, Distribution};

use crate::{iu::constants::SYMBOLES, symboles::Symbole};

pub struct Mixeur {
    pub rouleaux: Vec<Symbole>,
}

impl Mixeur {
    pub fn symboles() -> Self {
        Self {
            rouleaux: SYMBOLES.to_vec(),
        }
    }

    pub fn mélanger(&self, liste: &Vec<Symbole>) -> Vec<Symbole> {
        let symboles_pondérés =
            WeightedIndex::new(self.rouleaux.iter().map(|symbole| symbole.pondération))
                .expect("Invalide");

        let mut symboles = Vec::new();

        for _ in 0..3 {
            symboles.push(liste[symboles_pondérés.sample(&mut rand::rng())].clone());
        }

        symboles
    }
}
