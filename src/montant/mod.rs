pub struct Montant {
    pub mise: f32,
    pub total: f32,
    pub gains: Vec<(String, f32)>,
}

impl Montant {
    pub fn changer_mise(&mut self, mise: f32) {
        self.mise = mise;
    }

    pub fn ajouter_total(&mut self, total: f32) {
        self.total += total;
    }

    pub fn dépenser(&mut self) {
        self.total -= self.mise;
        self.perdu();
    }

    fn perdu(&mut self) {
        if self.total <= 0.0 {
            self.total = 0.0;
            self.mise = 0.0;
        }
    }

    pub fn afficher(&self) -> Vec<String> {
        let mut affichage = Vec::new();

        affichage.push(format!(
            "Mise: ${:.2}\nTotale: ${:.2}",
            self.mise, self.total
        ));

        let combinaison = self
            .gains
            .iter()
            .map(|(symboles, retour)| format!("{} = ${:.2}", symboles, retour * self.mise))
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|groupe| groupe.join(" | "))
            .collect::<Vec<_>>()
            .join("\n");
        affichage.push(combinaison);

        affichage.push("TOURNER".to_string());

        affichage
    }
}
