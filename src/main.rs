
use eframe::egui;
use std::collections::HashMap;
use egui::TextEdit;

#[derive(Default)]
struct GestionTempsApp {
    donnees_hors_clientele: String,
    donnees_clientele: String,
    resultat: Option<ResultatCalcul>,
    show_result: bool,
}

#[derive(Clone)]
struct ResultatCalcul {
    hors_clientele: f64,
    travail_clientele: f64,
    deplacement: f64,
}

impl eframe::App for GestionTempsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📊 Gestion du Temps de Travail");
            ui.separator();

            // Zone de saisie 1
            ui.group(|ui| {
            ui.label("🏢 Données Hors Clientèle:");
            ui.add(
                TextEdit::multiline(&mut self.donnees_hors_clientele)
                    .desired_width(f32::INFINITY)
                    .desired_rows(6),
                );
                ui.small("Format: Code\\tDescription\\tDébut\\tFin");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("👥 Données En Clientèle:");
                ui.add(
                    TextEdit::multiline(&mut self.donnees_clientele)
                        .desired_width(f32::INFINITY)
                        .desired_rows(6),
                );
                ui.small("Format: WO\\tClient\\t...\\tDuréeTravail\\tDuréeTrajet");
            });
            ui.add_space(20.0);

            // Boutons
            ui.horizontal(|ui| {
                if ui.button("🔄 Calculer").clicked() {
                    self.calculer_resultats();
                }
                
                if ui.button("🗑️ Effacer").clicked() {
                    self.donnees_hors_clientele.clear();
                    self.donnees_clientele.clear();
                    self.resultat = None;
                    self.show_result = false;
                }
            });

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

                if ui.button("💾 Générer Graphique").clicked() {
                    self.generer_graphique();
                }
            }
        });
    }
}

impl GestionTempsApp {
    fn calculer_resultats(&mut self) {
        // Ici vous implémenteriez la logique de calcul
        // Similaire à votre script Python mais en Rust
        
        let hors_clientele = self.parser_donnees_hors_clientele(&self.donnees_hors_clientele);
        let (travail, deplacement) = self.parser_donnees_clientele(&self.donnees_clientele);
        
        self.resultat = Some(ResultatCalcul {
            hors_clientele,
            travail_clientele: travail,
            deplacement,
        });
    }
    
    fn parser_donnees_hors_clientele(&self, donnees: &str) -> f64 {
        // Parser et calculer les heures hors clientèle
        // Exclure les "Pause déjeuner"
        0.0 // Placeholder
    }
    
    fn parser_donnees_clientele(&self, donnees: &str) -> (f64, f64) {
        // Parser et extraire durée travail et déplacement
        (0.0, 0.0) // Placeholder
    }
    
    fn generer_graphique(&self) {
        // Utiliser plotters pour générer le camembert
        println!("Génération du graphique...");
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_title("Gestion du Temps"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Gestion du Temps",
        options,
        Box::new(|_cc| Box::new(GestionTempsApp::default())),
    )
}
