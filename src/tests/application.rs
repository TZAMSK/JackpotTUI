#[cfg(test)]
mod tests {
    use crate::{
        application::Application,
        iu::constants::SYMBOLES,
        symboles::{Symbole, Type},
    };

    #[test]
    fn test_étant_donné_une_liste_de_touts_les_symboles_lorqsqu_on_calcule_la_pondération_totale_on_obtient_trois_76(
    ) {
        let cobaye = Application::poids_total(&SYMBOLES);
        assert_eq!(cobaye, 76.0);
    }

    #[test]
    fn test_étant_donné_une_liste_de_symboles_contenant_un_citron_diamant_et_bière_lorsqu_on_extrait_leur_pondération_obtient_leur_pondération(
    ) {
        let combinaison = vec![Type::Citron, Type::Diamant, Type::Bière];
        let cobaye = Application::pondérations(&SYMBOLES, combinaison);

        assert_eq!(cobaye, vec![40.0, 1.0, 4.0]);
    }

    #[test]
    fn test_étant_donné_après_avoir_tourner_la_machine_lorsqu_on_a_une_combinaison_de_3_citrons_on_obtient_son_paiement(
    ) {
        let combinaison = vec![Type::Citron; 3];
        let cobaye = Application::calculer_paiement_combinaison(&SYMBOLES, combinaison);

        assert_eq!(cobaye, 3.0);
    }

    #[test]
    fn test_étant_donné_après_avoir_tourner_la_machine_lorsqu_on_a_une_combinaison_de_3_diamants_on_obtient_son_paiement(
    ) {
        let combinaison = vec![Type::Diamant; 3];
        let cobaye = Application::calculer_paiement_combinaison(&SYMBOLES, combinaison);

        assert_eq!(cobaye, 197539.0);
    }

    #[test]
    fn test_étant_donné_après_avoir_tourner_la_machine_lorsqu_on_a_une_combinaison_d_un_citron_diamant_et_bière_on_obtient_son_paiement(
    ) {
        let combinaison = vec![Type::Citron, Type::Diamant, Type::Bière];
        let cobaye = Application::calculer_paiement_combinaison(&SYMBOLES, combinaison);

        assert_eq!(cobaye, 1235.0);
    }

    #[test]
    fn test_étant_donné_une_combinaison_de_symboles_lorsqu_on_a_une_combinaison_de_symboles_similaires_on_obtient_true(
    ) {
        let cobaye = Application::initialiser();
        let combinaison = vec![
            Symbole {
                type_: Type::Diamant,
                pondération: 0,
            },
            Symbole {
                type_: Type::Diamant,
                pondération: 0,
            },
            Symbole {
                type_: Type::Diamant,
                pondération: 0,
            },
        ];

        assert_eq!(cobaye.similaire(combinaison), true);
    }

    #[test]
    fn test_étant_donné_une_combinaison_de_symboles_lorsqu_on_a_une_combinaison_de_symboles_uniques_on_obtient_false(
    ) {
        let cobaye = Application::initialiser();
        let combinaison = vec![
            Symbole {
                type_: Type::Diamant,
                pondération: 0,
            },
            Symbole {
                type_: Type::Citron,
                pondération: 0,
            },
            Symbole {
                type_: Type::Bière,
                pondération: 0,
            },
        ];

        assert_eq!(cobaye.similaire(combinaison), false);
    }
}
