use eframe::egui;
use gestion_temps::GestionTempsApp;


fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_title("Gestion du Temps de Travail")
            .with_resizable(true)
            .with_icon(
                // NOE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/sablier.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    
    eframe::run_native(
        "Gestion du Temps de Travail",
        options,
        Box::new(|_cc| Box::new(GestionTempsApp::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    // Test helper pour créer une instance de GestionTempsApp
    fn create_test_app() -> GestionTempsApp {
        GestionTempsApp::default()
    }

    #[test]
    fn test_calculer_duree_normale() {
        let app = create_test_app();
        let duree = app.calculer_duree("10/06/2025 08:00", "10/06/2025 10:30");
        assert_eq!(duree, 2.5);
    }

    #[test]
    fn test_calculer_duree_vide() {
        let app = create_test_app();
        let duree = app.calculer_duree("", "10/06/2025 10:30");
        assert_eq!(duree, 0.0);
        
        let duree = app.calculer_duree("10/06/2025 08:00", "");
        assert_eq!(duree, 0.0);
    }

    #[test]
    fn test_format_duree_en_heures() {
        let app = create_test_app();
        
        assert_eq!(app.format_duree_en_heures(&2.5), "02h30");
        assert_eq!(app.format_duree_en_heures(&1.25), "01h15");
        assert_eq!(app.format_duree_en_heures(&0.0), "00h00");
        assert_eq!(app.format_duree_en_heures(&8.75), "08h45");
    }

    #[test]
    fn test_parser_donnees_hors_clientele() {
        let mut app = create_test_app();
        let donnees = "ABS--313129\tRévision véhicule\t10/06/2025 08:00\t10/06/2025 10:00\nABS--313130\tPause déjeuner\t10/06/2025 12:00\t10/06/2025 13:00";
        
        let resultats = app.parser_donnees_hors_clientele(donnees);
        
        // Devrait avoir 1 seul résultat (pause déjeuner exclue)
        assert_eq!(resultats.len(), 1);
        assert_eq!(resultats[0].description, "Révision véhicule");
        assert_eq!(resultats[0].duree, 2.0);
    }

    #[test]
    fn test_parser_donnees_clientele() {
        let mut app = create_test_app();
        let donnees = "WO-02562974\tClient Alpha\tUpgrade Technique\tClôturé\t10/06/2025 08:48\t10/06/2025 09:24\t-\t-\t10/06/2025 09:24\t10/06/2025 10:41\t-\t-\t-\t-\t1,3\t0,6";
        
        let (travail, deplacement) = app.parser_donnees_clientele(donnees);
        
        assert_eq!(travail, 1.3);
        assert_eq!(deplacement, 0.6);
    }

    #[test]
    fn test_calcul_complet_jeu1() {
        let mut app = create_test_app();
        
        // Données hors clientèle du jeu 1
        app.donnees_hors_clientele = "ABS--313129\tRévision véhicule\t10/06/2025 07:30\t10/06/2025 11:15\nABS--313130\tFormation sécurité\t10/06/2025 13:00\t10/06/2025 17:00\nABS--313131\tRéunion équipe\t11/06/2025 08:00\t11/06/2025 09:30\nABS--313132\tPréparation matériel\t11/06/2025 14:00\t11/06/2025 15:30\nABS--313133\tPause déjeuner\t11/06/2025 12:00\t11/06/2025 13:00\nABS--313134\tMaintenance outillage\t12/06/2025 16:00\t12/06/2025 17:45".to_string();
        
        // Données clientèle du jeu 1
        app.donnees_clientele = "WO-02562974\tClient Alpha\tUpgrade Technique\tClôturé\t10/06/2025 08:48\t10/06/2025 09:24\t-\t-\t10/06/2025 09:24\t10/06/2025 10:41\t-\t-\t-\t-\t1,3\t0,6\nWO-02562975\tClient Beta\tMaintenance\tClôturé\t11/06/2025 09:30\t11/06/2025 10:15\t-\t-\t11/06/2025 10:15\t11/06/2025 12:00\t-\t-\t-\t-\t1,75\t0,8\nWO-02562976\tClient Gamma\tInstallation\tClôturé\t12/06/2025 08:00\t12/06/2025 09:00\t-\t-\t12/06/2025 09:00\t12/06/2025 14:00\t-\t-\t-\t-\t5,0\t1,2".to_string();
        
        app.calculer_resultats();
        
        if let Some(resultat) = &app.resultat {
            // Vérifications avec une tolérance pour les calculs flottants
            assert!((resultat.hors_clientele - 12.5).abs() < 0.01, 
                   "Hors clientèle attendu: 12.5, trouvé: {}", resultat.hors_clientele);
            assert!((resultat.travail_clientele - 8.05).abs() < 0.01,
                   "Travail clientèle attendu: 8.05, trouvé: {}", resultat.travail_clientele);
            assert!((resultat.deplacement - 2.6).abs() < 0.01,
                   "Déplacement attendu: 2.6, trouvé: {}", resultat.deplacement);
        } else {
            panic!("Aucun résultat calculé");
        }
    }

    #[test]
    fn test_temps_par_jour() {
        let mut app = create_test_app();
        
        app.donnees_hors_clientele = "ABS--313170\tPréparation\t17/06/2025 07:30\t17/06/2025 08:30".to_string();
        app.donnees_clientele = "WO-02563010\tClient Kappa\tInstallation\tClôturé\t17/06/2025 08:30\t17/06/2025 09:00\t-\t-\t17/06/2025 09:00\t17/06/2025 12:30\t-\t-\t-\t-\t3,5\t0,5".to_string();
        
        app.calculer_resultats();
        
        let date_17_juin = NaiveDate::from_ymd_opt(2025, 6, 17).unwrap();
        let temps_total = app.temps_par_jour.get(&date_17_juin);
        
        assert!(temps_total.is_some());
        // 1h (hors clientèle) + 3.5h (travail) + 0.5h (déplacement) = 5h
        assert!((temps_total.unwrap() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_donnees_manquantes() {
        let mut app = create_test_app();
        
        // Test avec données incomplètes
        let resultats = app.parser_donnees_hors_clientele("ABS--313160\tTâche incomplète\t16/06/2025 08:00\t");
        assert_eq!(resultats.len(), 1);
        assert_eq!(resultats[0].duree, 0.0); // Durée nulle pour données incomplètes
    }

    #[test]
    fn test_conversion_virgule_vers_point() {
        let mut app = create_test_app();
        
        // Test avec virgules dans les durées
        let donnees = "WO-02562990\tClient Test\tTest\tClôturé\t14/06/2025 10:45\t14/06/2025 11:15\t-\t-\t14/06/2025 11:15\t14/06/2025 14:30\t-\t-\t-\t-\t3,25\t0,75";
        let (travail, deplacement) = app.parser_donnees_clientele(donnees);
        
        assert_eq!(travail, 3.25);
        assert_eq!(deplacement, 0.75);
    }

    #[test]
    fn test_ligne_vide_ignoree() {
        let mut app = create_test_app();
        
        let donnees_avec_lignes_vides = "ABS--313129\tRévision véhicule\t10/06/2025 08:00\t10/06/2025 10:00\n\n\nABS--313130\tFormation\t10/06/2025 14:00\t10/06/2025 16:00";
        let resultats = app.parser_donnees_hors_clientele(donnees_avec_lignes_vides);
        
        assert_eq!(resultats.len(), 2);
        assert_eq!(resultats[0].duree, 2.0);
        assert_eq!(resultats[1].duree, 2.0);
    }

    #[test]
    fn test_format_date_alternatif() {
        let app = create_test_app();
        
        // Test avec différents formats de date si votre parser les supporte
        // (Vous devrez peut-être adapter selon votre implémentation)
        let duree = app.calculer_duree("10/06/2025 08:00", "10/06/2025 10:00");
        assert_eq!(duree, 2.0);
    }
}
