use eframe::egui;
use chrono::{NaiveDateTime, Timelike, Datelike,NaiveDate};
use std::collections::HashMap;
use plotters::prelude::*;

#[derive(Default)]
struct GestionTempsApp {
    donnees_hors_clientele: String,
    donnees_clientele: String,
    resultat: Option<ResultatCalcul>,
    show_result: bool,
    status_message: String,
    temps_par_jour: HashMap<NaiveDate, f64>,
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

            egui::ScrollArea::vertical()
                .auto_shrink([false;2])
                .show(ui, |ui| {
                        ui.heading("📊 Gestion du Temps de Travail");
                        ui.separator();

                        // Zone de saisie 1 - Données Hors Clientèle
                        ui.group(|ui| {
                            ui.label("🏢 Données Hors Clientèle:");
                            ui.add(
                                egui::TextEdit::multiline(&mut self.donnees_hors_clientele)
                                    .id_source("donnees_hors_clientele")
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
                                    .id_source("donnees_clientele")
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
                                self.temps_par_jour.clear();
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

                        // Ajoutez une section pour afficher les résultats par jour
                        if !self.temps_par_jour.is_empty() {
                            ui.separator();
                            ui.heading("Temps de travail par jour :");

                            for (date, &total) in &self.temps_par_jour {
                                ui.label(format!("Date: {} - Temps total: {} ( {:.2}h )", date, self.format_duree_en_heures(total), total));
                            }
                        }
                });
        });
    }
}

impl GestionTempsApp {
    fn calculer_resultats(&mut self) {
        self.status_message.clear();
        self.temps_par_jour.clear(); // Reset des données par jour
        
        // Cloner les données pour éviter les conflits d'emprunt
        let donnees_hors_clientele = self.donnees_hors_clientele.clone();
        let donnees_clientele = self.donnees_clientele.clone();
        
        let hors_clientele_data = self.parser_donnees_hors_clientele(&donnees_hors_clientele);
        let hors_clientele_total = hors_clientele_data.iter().map(|e| e.duree).sum::<f64>();
        
        let (travail, deplacement) = self.parser_donnees_clientele(&donnees_clientele);
        
        self.resultat = Some(ResultatCalcul {
            hors_clientele: hors_clientele_total,
            travail_clientele: travail,
            deplacement,
            details_hors_clientele: hors_clientele_data,
        });
        
        self.status_message = "Calculs terminés avec succès !".to_string();
    }
    
    fn parser_donnees_hors_clientele(&mut self, donnees: &str) -> Vec<EntreeHorsClientele> {
        let mut resultats = Vec::new();

        for (numero_ligne, ligne) in donnees.lines().enumerate() {
            let ligne = ligne.trim();
            if ligne.is_empty() {
                continue;
            }

            let parties: Vec<&str> = ligne.split('\t').collect();
            if parties.len() < 4 {
                println!("Ligne {} ignorée (pas assez de colonnes): {}", numero_ligne + 1, ligne);
                continue;
            }

            let description = parties[1];
            if description.to_lowercase().contains("pause déjeuner") {
                continue;
            }

            let debut = parties[2].trim();
            let fin = parties[3].trim();
            let duree = self.calculer_duree(debut, fin);

            // Parsez la date de début pour obtenir la date
            match NaiveDateTime::parse_from_str(debut, "%d/%m/%Y %H:%M") {
                Ok(date_debut) => {
                    let date = date_debut.date();
                    *self.temps_par_jour.entry(date).or_insert(0.0) += duree;
                }
                Err(e) => {
                    println!("Erreur parsing date hors clientèle ligne {}: '{}' - {}", numero_ligne + 1, debut, e);
                    // Essayer d'autres formats possibles
                    if let Ok(date_debut) = NaiveDateTime::parse_from_str(debut, "%d/%m/%Y %H:%M:%S") {
                        let date = date_debut.date();
                        *self.temps_par_jour.entry(date).or_insert(0.0) += duree;
                    } else if let Ok(date_debut) = NaiveDateTime::parse_from_str(debut, "%Y-%m-%d %H:%M") {
                        let date = date_debut.date();
                        *self.temps_par_jour.entry(date).or_insert(0.0) += duree;
                    }
                }
            }

            resultats.push(EntreeHorsClientele {
                description: description.to_string(),
                debut: Some(debut.to_string()),
                fin: Some(fin.to_string()),
                duree,
            });
        }

        resultats
    }

    
    fn parser_donnees_clientele(&mut self, donnees: &str) -> (f64, f64) {
        let mut total_travail = 0.0;
        let mut total_deplacement = 0.0;

        for (numero_ligne, ligne) in donnees.lines().enumerate() {
            let ligne = ligne.trim();
            if ligne.is_empty() || ligne.starts_with("ABS") || ligne.starts_with("Description") {
                continue;
            }

            let parties: Vec<&str> = ligne.split('\t').collect();
            if parties.len() >= 6 {
                // Parsez la date pour obtenir la date
                let date_str = parties[8].trim();
                println!("{}", date_str);
                match NaiveDate::parse_from_str(date_str, "%d/%m/%Y %H:%M") {
                    Ok(date) => {
                        // Durée du travail (avant-dernière colonne)
                        if let Ok(travail) = parties[parties.len() - 2].replace(',', ".").parse::<f64>() {
                            total_travail += travail;
                            *self.temps_par_jour.entry(date).or_insert(0.0) += travail;
                        }

                        // Durée du trajet (dernière colonne)
                        if let Ok(deplacement) = parties[parties.len() - 1].replace(',', ".").parse::<f64>() {
                            total_deplacement += deplacement;
                            *self.temps_par_jour.entry(date).or_insert(0.0) += deplacement;
                        }
                    }
                    Err(e) => {
                        println!("Erreur parsing date clientèle ligne {}: '{}' - {}", numero_ligne + 1, date_str, e);
                        
                        // Essayer d'autres formats possibles
                        let date_parsed = if let Ok(d) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                            Some(d)
                        } else if let Ok(d) = NaiveDate::parse_from_str(date_str, "%d-%m-%Y") {
                            Some(d)
                        } else if let Ok(d) = NaiveDate::parse_from_str(date_str, "%d.%m.%Y") {
                            Some(d)
                        } else {
                            println!("Impossible de parser la date: '{}'", date_str);
                            None
                        };
                        
                        if let Some(date) = date_parsed {
                            // Durée du travail (avant-dernière colonne)
                            if let Ok(travail) = parties[parties.len() - 2].replace(',', ".").parse::<f64>() {
                                total_travail += travail;
                                *self.temps_par_jour.entry(date).or_insert(0.0) += travail;
                            }

                            // Durée du trajet (dernière colonne)
                            if let Ok(deplacement) = parties[parties.len() - 1].replace(',', ".").parse::<f64>() {
                                total_deplacement += deplacement;
                                *self.temps_par_jour.entry(date).or_insert(0.0) += deplacement;
                            }
                        }
                    }
                }
            } else {
                println!("Ligne {} ignorée (pas assez de colonnes): {}", numero_ligne + 1, ligne);
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
                0.0
            }
        }
    }

    fn format_duree_en_heures(&self,duree_heures: f64) -> String {
        let heures = duree_heures.floor() as u32;
        let minutes = ((duree_heures - heures as f64) * 60.0).floor() as u32;
        let secondes = ((((duree_heures - heures as f64) * 60.0) - minutes as f64) * 60.0).round() as u32;

       // format!("{:02}h{:02}m{:02}s", heures, minutes, secondes)
        format!("{:02}h{:02}", heures, minutes)
    }
    
    fn generer_graphique(&self) {
        if let Some(ref resultat) = self.resultat {
            match self.creer_camembert(resultat) {
                Ok(_) => {
                    println!("Graphique généré avec succès : camembert_temps_travail.png");
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