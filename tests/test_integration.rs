
use gestion_temps::GestionTempsApp;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_workflow_complet() {
        let mut app = GestionTempsApp::default();
        
        // Simuler la saisie utilisateur
        app.donnees_hors_clientele = "ABS--313129\tTest\t10/06/2025 08:00\t10/06/2025 10:00".to_string();
        app.donnees_clientele = "WO-123\tClient Test\tTest\tClôturé\t10/06/2025 08:00\t10/06/2025 09:00\t-\t-\t10/06/2025 09:00\t10/06/2025 10:00\t-\t-\t-\t-\t1,0\t0,5".to_string();
        
        // Exécuter le calcul
        app.calculer_resultats();
        
        // Vérifier que les résultats sont présents
        assert!(app.resultat.is_some());
        assert!(!app.status_message.is_empty());
        assert!(!app.temps_par_jour.is_empty());
    }
}