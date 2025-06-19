use eframe::egui;
use chrono::{NaiveDateTime, Timelike, Datelike};
use std::collections::HashMap;
use plotters::prelude::*;

#[derive(Default)]
struct GestionTempsApp {
    donnees_hors_clientele: String,
    donnees_clientele: String,
    resultat: Option<ResultatCalcul>,
    show_result: bool,
    status_message: String,
}

#[derive(Clone, Debug)]
struct ResultatCalcul {
    hors_clientele: f64,
    travail_clientele: f64,
    deplacement: f64,
    details_hors_clientele: Vec<EntreeHorsClientele>,
}

#[derive(Clone, Debug)]
struct EntreeHorsClientele {
    description: String,
    debut: Option<String>,
    fin: Option<String>,
    duree: f64,
}

impl eframe::App for GestionTempsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📊 Gestion du Temps de Travail");
            ui.separator();

            // Zone de saisie 1 - Données Hors Clientèle
            ui.group(|ui| {
                ui.label("🏢 Données Hors Clientèle:");
                ui.add(
                    egui::TextEdit::multiline(&mut self.donnees_hors_clientele)
                        .desired_width(f32::INFINITY)
                        .desired_rows(8)
                        .hint_text("Collez vos données ici...\nFormat: Code\tDescription\tDébut\tFin"),
                );
                ui.small("Format attendu: Code\\tDescription\\tDébut (DD/MM/YYYY HH:MM)\\tFin (DD/MM/YYYY HH:MM)");
            });

            ui.add_space(10.0);

            // Zone de saisie 2 - Données En Clientèle
            ui.group(|ui| {
                ui.label("👥 Données En Clientèle:");
                ui.add(
                    egui::TextEdit::multiline(&mut self.donnees_clientele)
                        .desired_width(f32::INFINITY)
                        .desired_rows(8)
                        .hint_text("Collez vos données ici..."),
                );
                ui.small("Format attendu: WO\\tClient\\t...\\tDuréeTravail\\tDuréeTrajet");
            });

            ui.add_space(20.0);

            // Boutons d'action
            ui.horizontal(|ui| {
                if ui.button("🔄 Calculer").clicked() {
                    self.calculer_resultats();
                }
                
                if ui.button("🗑️ Effacer").clicked() {
                    self.donnees_hors_clientele.clear();
                    self.donnees_clientele.clear();
                    self.resultat = None;
                    self.show_result = false;
                    self.status_message.clear();
                }
            });

            // Message de statut
            if !self.status_message.is_empty() {
                ui.add_space(10.0);
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), &self.status_message);
            }

            // Affichage des résultats
            if let Some(ref resultat) = self.resultat {
                ui.separator();
                ui.heading("📈 Résultats:");
                
                ui.group(|ui| {
                    ui.label(format!("🏢 Hors clientèle: {:.2} heures", resultat.hors_clientele));
                    ui.label(format!("👥 Travail clientèle: {:.2} heures", resultat.travail_clientele));
                    ui.label(format!("🚗 Déplacement: {:.2} heures", resultat.deplacement));
                    
                    let total = resultat.hors_clientele + resultat.travail_clientele + resultat.deplacement;
                    ui.strong(&format!("⏱️ Total: {:.2} heures", total));
                });

                // Détails des activités hors clientèle
                if !resultat.details_hors_clientele.is_empty() {
                    ui.add_space(10.0);
                    ui.collapsing("📋 Détails Hors Clientèle", |ui| {
                        for detail in &resultat.details_hors_clientele {
                            ui.label(format!("• {}: {:.2}h", detail.description, detail.duree));
                        }
                    });
                }

                ui.add_space(10.0);
                if ui.button("💾 Générer Graphique").clicked() {
                    self.generer_graphique();
                }
            }
        });
    }
}

impl GestionTempsApp {
    fn calculer_resultats(&mut self) {
        self.status_message.clear();
        
        let hors_clientele_data = self.parser_donnees_hors_clientele(&self.donnees_hors_clientele);
        let hors_clientele_total = hors_clientele_data.iter().map(|e| e.duree).sum::<f64>();
        
        let (travail, deplacement) = self.parser_donnees_clientele(&self.donnees_clientele);
        
        self.resultat = Some(ResultatCalcul {
            hors_clientele: hors_clientele_total,
            travail_clientele: travail,
            deplacement,
            details_hors_clientele: hors_clientele_data,
        });
        
        self.status_message = "Calculs terminés avec succès !".to_string();
    }
    
    fn parser_donnees_hors_clientele(&self, donnees: &str) -> Vec<EntreeHorsClientele> {
        let mut resultats = Vec::new();
        
        for ligne in donnees.lines() {
            let ligne = ligne.trim();
            if ligne.is_empty() {
                continue;
            }
            
            let parties: Vec<&str> = ligne.split('\t').collect();
            if parties.len() < 4 {
                continue;
            }
            
            // Ignorer les pauses déjeuner
            let description = parties[1];
            if description.to_lowercase().contains("pause déjeuner") {
                println!("⏭️ Pause déjeuner ignorée: {}", description);
                continue;
            }
            
            let debut = parties[2];
            let fin = parties[3];
            let duree = self.calculer_duree(debut, fin);
            
            resultats.push(EntreeHorsClientele {
                description: description.to_string(),
                debut: Some(debut.to_string()),
                fin: Some(fin.to_string()),
                duree,
            });
        }
        
        resultats
    }
    
    fn parser_donnees_clientele(&self, donnees: &str) -> (f64, f64) {
        let mut total_travail = 0.0;
        let mut total_deplacement = 0.0;
        
        for ligne in donnees.lines() {
            let ligne = ligne.trim();
            if ligne.is_empty() || ligne.starts_with("ABS") || ligne.starts_with("Description") {
                continue;
            }
            
            let parties: Vec<&str> = ligne.split('\t').collect();
            if parties.len() >= 6 {
                // Durée du travail (avant-dernière colonne)
                if let Ok(travail) = parties[parties.len() - 2].replace(',', ".").parse::<f64>() {
                    total_travail += travail;
                }
                
                // Durée du trajet (dernière colonne)
                if let Ok(deplacement) = parties[parties.len() - 1].replace(',', ".").parse::<f64>() {
                    total_deplacement += deplacement;
                }
            }
        }
        
        (total_travail, total_deplacement)
    }
    
    fn calculer_duree(&self, debut: &str, fin: &str) -> f64 {
        if debut.is_empty() || fin.is_empty() {
            return 0.0;
        }
        
        let format = "%d/%m/%Y %H:%M";
        
        match (
            NaiveDateTime::parse_from_str(debut, format),
            NaiveDateTime::parse_from_str(fin, format)
        ) {
            (Ok(debut_dt), Ok(fin_dt)) => {
                let duree = fin_dt.signed_duration_since(debut_dt);
                duree.num_seconds() as f64 / 3600.0
            }
            _ => {
                println!("Erreur de parsing des dates: {} -> {}", debut, fin);
                0.0
            }
        }
    }
    
    fn generer_graphique(&self) {
        if let Some(ref resultat) = self.resultat {
            match self.creer_camembert(resultat) {
                Ok(_) => {
                    println!("Graphique généré avec succès !");
                    // Ici vous pourriez afficher un message dans l'interface
                }
                Err(e) => {
                    println!("Erreur lors de la génération du graphique: {}", e);
                }
            }
        }
    }
    
    fn creer_camembert(&self, resultat: &ResultatCalcul) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("camembert_temps_travail.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption("Répartition du temps de travail", ("sans-serif", 30))
            .margin(20)
            .build_cartesian_2d(-1.2f32..1.2f32, -1.2f32..1.2f32)?;
        
        let total = resultat.hors_clientele + resultat.travail_clientele + resultat.deplacement;
        if total == 0.0 {
            return Ok(());
        }
        
        let donnees = vec![
            ("Hors clientèle", resultat.hors_clientele, &RED),
            ("Travail clientèle", resultat.travail_clientele, &BLUE),
            ("Déplacement", resultat.deplacement, &GREEN),
        ];
        
        let mut angle_debut = 0.0f32;
        
        for (label, valeur, couleur) in donnees {
            if valeur > 0.0 {
                let angle_fin = angle_debut + (valeur / total * 360.0) as f32;
                
                // Dessiner la section du camembert
                let points: Vec<(f32, f32)> = (0..=((angle_fin - angle_debut) as i32))
                    .map(|i| {
                        let angle = (angle_debut + i as f32) * std::f32::consts::PI / 180.0;
                        (angle.cos(), angle.sin())
                    })
                    .collect();
                
                let mut path = vec![(0.0, 0.0)];
                path.extend(points);
                path.push((0.0, 0.0));
                
                chart.draw_series(std::iter::once(Polygon::new(path, couleur.filled())))?;
                
                // Ajouter le texte
                let angle_milieu = (angle_debut + angle_fin) / 2.0 * std::f32::consts::PI / 180.0;
                let x = angle_milieu.cos() * 0.7;
                let y = angle_milieu.sin() * 0.7;
                
                chart.draw_series(std::iter::once(Text::new(
                    format!("{}\n{:.1}h ({:.1}%)", label, valeur, valeur / total * 100.0),
                    (x, y),
                    ("sans-serif", 12),
                )))?;
                
                angle_debut = angle_fin;
            }
        }
        
        root.present()?;
        println!("Graphique sauvegardé sous 'camembert_temps_travail.png'");
        
        Ok(())
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_title("Gestion du Temps de Travail")
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Gestion du Temps de Travail",
        options,
        Box::new(|_cc| Box::new(GestionTempsApp::default())),
    )
}