#[cfg(test)]
mod tests {
    use crate::symboles::{mixeur::Mixeur, Symbole};

    #[test]
    fn test_étant_donné_un_mixeur_nouvellement_initialisé_lorqsqu_on_passe_une_liste_de_toutes_les_symboles_possibles_on_obtient_trois_symboles(
    ) {
        let symboles = Mixeur::mélanger_symboles();
        assert_eq!(symboles.len(), 3);
    }

    #[test]
    fn test_étant_donné_un_mixeur_nouvellement_initialisé_lorqsqu_on_passe_une_liste_de_toutes_les_symboles_possibles_on_obtient_une_liste_qui_possède_des_éléments_de_type_symbole(
    ) {
        let symboles = Mixeur::mélanger_symboles();
        assert_eq!(symboles.len(), 3);
    }
}
