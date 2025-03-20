use tui_input::Input;

use crate::{
    iu::constants::SYMBOLES,
    montant::Montant,
    symboles::{mixeur::Mixeur, Symbole, Type},
};

pub struct Application {
    pub mixeur: Mixeur,
    pub symboles: Vec<Symbole>,
    pub montant: Montant,
    pub saisie: Input,
    pub saisie_mode: SaisieMode,
    pub affichage_contextuel: bool,
    pub type_contextuel: TypeContextuel,
}

#[derive(PartialEq)]
pub enum SaisieMode {
    Normale,
    Édition,
}

pub enum TypeContextuel {
    Mise,
    Totale,
}

impl Application {
    pub fn initialiser() -> Self {
        let mixeur = Mixeur::symboles();
        let symboles = mixeur.mélanger(&mixeur.rouleaux);
        let montant = Montant {
            mise: 2.0,
            total: 90.0,
            gains: vec![
                (
                    "💠💠💠".to_string(),
                    Self::calculer_payement_combinaison(&SYMBOLES, vec![Type::Diamant; 3]),
                ),
                (
                    "🍺🍺🍺".to_string(),
                    Self::calculer_payement_combinaison(&SYMBOLES, vec![Type::Bière; 3]),
                ),
                (
                    "⭐⭐⭐".to_string(),
                    Self::calculer_payement_combinaison(&SYMBOLES, vec![Type::Étoile; 3]),
                ),
                (
                    "🍒🍒🍒".to_string(),
                    Self::calculer_payement_combinaison(&SYMBOLES, vec![Type::Cerise; 3]),
                ),
                (
                    "🔔🔔🔔".to_string(),
                    Self::calculer_payement_combinaison(&SYMBOLES, vec![Type::Cloche; 3]),
                ),
                (
                    "🍋🍋🍋".to_string(),
                    Self::calculer_payement_combinaison(&SYMBOLES, vec![Type::Citron; 3]),
                ),
                (
                    "🍌🍌🍌".to_string(),
                    Self::calculer_payement_combinaison(&SYMBOLES, vec![Type::Banane; 3]),
                ),
            ],
        };

        Self {
            mixeur,
            symboles,
            montant,
            saisie: Input::default(),
            saisie_mode: SaisieMode::Normale,
            affichage_contextuel: false,
            type_contextuel: TypeContextuel::Mise,
        }
    }

    pub fn mélanger_symboles(&mut self) {
        self.symboles = self.mixeur.mélanger(&self.mixeur.rouleaux);
    }

    pub fn éditer(&mut self, type_contextuel: TypeContextuel) {
        self.saisie_mode = SaisieMode::Édition;
        self.type_contextuel = type_contextuel;
        self.affichage_contextuel = true;
    }

    pub fn arrêter_édition(&mut self) {
        self.saisie_mode = SaisieMode::Normale;
        self.affichage_contextuel = false;
        self.saisie.reset();
    }

    pub fn contrôle_indices(&self) -> &'static str {
        match self.saisie_mode {
            SaisieMode::Normale => {
                "<q> Quitter\n<w>Changer mise\n<e> Changer totale\n<espace> Tourner"
            }

            SaisieMode::Édition => "<esc> Quitter mode édition\n<enter> Enregistrer",
        }
    }

    pub fn soumettre(&mut self) {
        match self.type_contextuel {
            TypeContextuel::Mise => self
                .montant
                .changer_mise(self.saisie.value().parse::<f32>().unwrap()),
            TypeContextuel::Totale => self
                .montant
                .ajouter_total(self.saisie.value().parse::<f32>().unwrap()),
        }
    }

    pub fn calculer_payement_combinaison(symboles: &[Symbole], combinaison: Vec<Type>) -> f32 {
        let mut probabilité = 1.0;
        let taux_de_retour_au_joueur = 0.95;

        for pondération in Self::pondérations(symboles, combinaison) {
            probabilité *= pondération / Self::poids_total(symboles)
        }

        let payement = taux_de_retour_au_joueur / probabilité;
        payement.round()
    }

    pub fn poids_total(symboles: &[Symbole]) -> f32 {
        symboles
            .iter()
            .map(|symbole| symbole.pondération as f32)
            .sum()
    }

    pub fn pondérations(symboles: &[Symbole], combinaison: Vec<Type>) -> Vec<f32> {
        combinaison
            .iter()
            .filter_map(|type_cherché| {
                symboles
                    .iter()
                    .find(|&symbole| symbole.type_ == *type_cherché)
                    .map(|symbole| symbole.pondération.clone() as f32)
            })
            .collect::<Vec<_>>()
    }
}
