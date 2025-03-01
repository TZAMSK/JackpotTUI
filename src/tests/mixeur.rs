#[cfg(test)]
mod tests {
    use crate::symboles::mixeur::Mixeur;

    #[test]
    fn test_étant_donné_un_mixeur_nouvellement_initialisé_lorqsqu_on_passe_une_liste_de_toutes_les_symboles_possibles_on_obtient_trois_symboles(
    ) {
        let cobaye = Mixeur::symboles();
        let symboles_mélangés = cobaye.mélanger(&cobaye.rouleaux);
        assert_eq!(symboles_mélangés.len(), 3);
    }
}
