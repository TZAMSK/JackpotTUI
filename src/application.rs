use tui_input::Input;

use crate::symboles::{mixeur::Mixeur, Symbole};

pub struct Application {
    pub mixeur: Mixeur,
    pub symboles: Vec<Symbole>,
    pub saisie: Input,
    pub saisie_mode: SaisieMode,
    pub affichage_contextuel: bool,
}

#[derive(PartialEq)]
pub enum SaisieMode {
    Normale,
    Édition,
}

impl Application {
    pub fn initialiser() -> Self {
        let mixeur = Mixeur::symboles();
        let symboles = mixeur.mélanger(&mixeur.rouleaux);
        Self {
            mixeur,
            symboles,
            saisie: Input::default(),
            saisie_mode: SaisieMode::Normale,
            affichage_contextuel: false,
        }
    }

    pub fn mélanger_symboles(&mut self) {
        self.symboles = self.mixeur.mélanger(&self.mixeur.rouleaux);
    }

    pub fn éditer(&mut self) {
        self.saisie_mode = SaisieMode::Édition;
        self.affichage_contextuel = true;
    }

    pub fn arrêter_édition(&mut self) {
        self.saisie_mode = SaisieMode::Normale;
        self.affichage_contextuel = false;
    }

    pub fn contrôle_indices(&self) -> &'static str {
        match self.saisie_mode {
            SaisieMode::Normale => {
                "<q> Quitter\n<w>Changer mise\n<e> Changer totale\n<espace> Tourner"
            }

            SaisieMode::Édition => "<esc> Quitter mode édition\n<enter> Enregistrer",
        }
    }

    pub fn soumettre(&self) {}
}
