
use gestion_temps::GestionTempsApp;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_workflow_complet_client_1() {
        let mut app = GestionTempsApp::default();
        
        app.donnees_hors_clientele = include_str!("data/client_1/hors_client.txt").to_string();
        app.donnees_clientele = include_str!("data/client_1/client.txt").to_string();

        // Exécuter le calcul
        app.calculer_resultats();
        
        // Vérifier que les résultats sont présents
        assert!(app.resultat.is_some());
        assert!(!app.status_message.is_empty());
        assert!(!app.temps_par_jour.is_empty());
       
       let resultat = app.resultat.as_ref().unwrap();
       // Vérifier les durées (tolérance possible sur les flottants)
        assert!((resultat.hors_clientele - 12.5).abs() < 0.01, "Durée hors clientèle incorrecte");
        assert!((resultat.travail_clientele - 8.05).abs() < 0.01, "Durée travail clientèle incorrecte");
        assert!((resultat.deplacement - 2.6).abs() < 0.01, "Durée déplacement incorrecte");

        // Vérifier le total
        let total = resultat.hors_clientele + resultat.travail_clientele + resultat.deplacement;
        assert!((total - 23.15).abs() < 0.01, "Durée totale incorrecte");
    }

    #[test]
    fn test_workflow_complet_client_2() {

        let mut app = GestionTempsApp::default();
        
        app.donnees_hors_clientele = include_str!("data/client_2/hors_client.txt").to_string();
        app.donnees_clientele = include_str!("data/client_2/client.txt").to_string();

        // Exécuter le calcul
        app.calculer_resultats();
        
        // Vérifier que les résultats sont présents
        assert!(app.resultat.is_some());
        assert!(!app.status_message.is_empty());
        assert!(!app.temps_par_jour.is_empty());
       
       let resultat = app.resultat.as_ref().unwrap();
       // Vérifier les durées (tolérance possible sur les flottants)
        assert!((resultat.hors_clientele - 5.5).abs() < 0.01, "Durée hors clientèle incorrecte");
        assert!((resultat.travail_clientele - 3.0).abs() < 0.01, "Durée travail clientèle incorrecte");
        assert!((resultat.deplacement - 1.0).abs() < 0.01, "Durée déplacement incorrecte");

        // Vérifier le total
        let total = resultat.hors_clientele + resultat.travail_clientele + resultat.deplacement;
        assert!((total - 9.5).abs() < 0.01, "Durée totale incorrecte");

    }

      #[test]
    fn test_workflow_complet_client_3() {

        let mut app = GestionTempsApp::default();
        
        app.donnees_hors_clientele = include_str!("data/client_3/hors_client.txt").to_string();
        app.donnees_clientele = include_str!("data/client_3/client.txt").to_string();

        // Exécuter le calcul
        app.calculer_resultats();
        
        // Vérifier que les résultats sont présents
        assert!(app.resultat.is_some());
        assert!(!app.status_message.is_empty());
        assert!(!app.temps_par_jour.is_empty());
       
       let resultat = app.resultat.as_ref().unwrap();
       // Vérifier les durées (tolérance possible sur les flottants)
        assert!((resultat.hors_clientele - 7.0).abs() < 0.01, "Durée hors clientèle incorrecte");
        assert!((resultat.travail_clientele - 7.25).abs() < 0.01, "Durée travail clientèle incorrecte");
        assert!((resultat.deplacement - 1.75).abs() < 0.01, "Durée déplacement incorrecte");

        // Vérifier le total
        let total = resultat.hors_clientele + resultat.travail_clientele + resultat.deplacement;
        assert!((total - 16.0).abs() < 0.01, "Durée totale incorrecte");

    }
}