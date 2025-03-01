use crate::symboles::{mixeur::Mixeur, Symbole};

pub struct Application {
    pub mixeur: Mixeur,
    pub symboles: Vec<Symbole>,
}

impl Application {
    pub fn initialiser() -> Self {
        let mixeur = Mixeur::symboles();
        let symboles = mixeur.mélanger(&mixeur.rouleaux);
        Self { mixeur, symboles }
    }

    pub fn mélanger_symboles(&mut self) {
        self.symboles = self.mixeur.mélanger(&self.mixeur.rouleaux);
    }
}
