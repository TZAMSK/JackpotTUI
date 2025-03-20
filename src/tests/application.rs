#[cfg(test)]
mod tests {
    use crate::{application::Application, iu::constants::SYMBOLES, symboles::Type};

    #[test]
    fn test_étant_donné_une_liste_de_touts_les_symboles_lorqsqu_on_calcule_la_pondération_totale_on_obtient_trois_110(
    ) {
        let cobaye = Application::poids_total(&SYMBOLES);
        assert_eq!(cobaye, 110.0);
    }

    #[test]
    fn test_étant_donné_une_liste_de_symboles_contenant_un_citron_diamant_et_bière_lorsqu_on_extrait_leur_pondération_obtient_leur_pondération(
    ) {
        let combinaison = vec![Type::Citron, Type::Diamant, Type::Bière];
        let cobaye = Application::pondérations(&SYMBOLES, combinaison);

        assert_eq!(cobaye, vec![20.0, 7.0, 14.0]);
    }

    #[test]
    fn test_étant_donné_une_combinaison_de_symboles_contenant_3_citrons_lorsqu_on_calcul_la_probablité_de_cette_combinaison_on_obtient_leur_pondération(
    ) {
        let combinaison = vec![Type::Citron; 3];
        let cobaye = Application::calculer_payement_combinaison(&SYMBOLES, combinaison);

        assert_eq!(cobaye, 158.0);
    }

    #[test]
    fn test_étant_donné_une_combinaison_de_symboles_contenant_3_diamants_lorsqu_on_calcul_la_probablité_de_cette_combinaison_on_obtient_leur_pondération(
    ) {
        let combinaison = vec![Type::Diamant; 3];
        let cobaye = Application::calculer_payement_combinaison(&SYMBOLES, combinaison);

        assert_eq!(cobaye, 3686.0);
    }

    #[test]
    fn test_étant_donné_une_combinaison_de_symboles_contenant_un_citron_diamant_et_bière_lorsqu_on_calcul_la_probablité_de_cette_combinaison_on_obtient_leur_pondération(
    ) {
        let combinaison = vec![Type::Citron, Type::Diamant, Type::Bière];
        let cobaye = Application::calculer_payement_combinaison(&SYMBOLES, combinaison);

        assert_eq!(cobaye, 645.0);
    }
}
