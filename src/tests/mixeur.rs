#[cfg(test)]
mod tests {
    use crate::symboles::{mixeur::Mixeur, Symbole, Type};

    #[test]
    fn test_étant_donné_un_mixeur_nouvellement_initialisé_lorqsqu_on_passe_une_liste_de_toutes_les_symboles_possibles_on_obtient_trois_symboles(
    ) {
        let cobaye = Mixeur::symboles();
        let symboles_mélangés = cobaye.mélanger(&cobaye.rouleaux);
        assert_eq!(symboles_mélangés.len(), 3);
    }

    #[test]
    fn test_étant_donné_un_mixeur_nouvellement_initialisé_lorqsqu_on_passe_une_liste_de_toutes_les_symboles_possibles_on_obtient_une_liste_contenant_des_éléments_de_type_symbole(
    ) {
        let cobaye = Mixeur::symboles();
        let symboles_mélangés = cobaye.mélanger(&cobaye.rouleaux);

        for symbole in symboles_mélangés {
            assert!(matches!(symbole, Symbole { .. }));
        }
    }

    #[test]
    fn test_étant_donné_un_mixeur_nouvellement_initialisé_lorqsqu_on_passe_une_liste_ou_le_citron_à_une_plus_haute_pondération_que_les_autres_on_obtient_au_moins_un_citron_à_chaque_fois(
    ) {
        let cobaye = Mixeur::symboles();

        let symboles = vec![
            Symbole {
                type_: Type::Citron,
                pondération: 255,
            },
            Symbole {
                type_: Type::Cloche,
                pondération: 0,
            },
            Symbole {
                type_: Type::Cerise,
                pondération: 0,
            },
            Symbole {
                type_: Type::Bière,
                pondération: 0,
            },
            Symbole {
                type_: Type::Étoile,
                pondération: 0,
            },
            Symbole {
                type_: Type::Banane,
                pondération: 0,
            },
            Symbole {
                type_: Type::Diamant,
                pondération: 0,
            },
        ];

        let symboles_mélangés = cobaye.mélanger(&symboles);

        assert!(symboles.iter().any(|s| s.type_ == Type::Citron));
    }
}
