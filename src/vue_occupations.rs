use crate::donnees::Donnees;
use crate::modele::*;
use crate::theme;
use eframe::egui::{self, RichText};

#[derive(Default)]
pub struct VueOccupations {
    selection: Option<u64>,
}

impl VueOccupations {
    pub fn afficher(&mut self, ui: &mut egui::Ui, donnees: &mut Donnees) -> Option<String> {
        let mut message = None;
        let today = aujourdhui();
        let disponibles: Vec<(u64, String)> = donnees
            .reservations
            .iter()
            .filter(|r| !donnees.reservation_occupee(r.id))
            .filter(|r| {
                let fin = date_sortie(r.date_entree, r.nombre_jours);
                (today >= r.date_entree && today < fin) || today < r.date_entree
            })
            .map(|r| {
                (
                    r.id,
                    format!(
                        "{}, {}, {} - {}",
                        r.nom_client,
                        r.numero_chambre,
                        formater_date_fr(r.date_entree),
                        formater_date_fr(date_sortie(r.date_entree, r.nombre_jours))
                    ),
                )
            })
            .collect();

        ui.label(
            RichText::new(theme::tr("Transformer une réservation en occupation"))
                .size(15.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(10.0);

        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(theme::tr("Réservation"))
                    .size(12.0)
                    .color(theme::palette().texte_secondaire)
                    .strong(),
            );
            ui.add_space(8.0);
            let texte = self
                .selection
                .and_then(|id| disponibles.iter().find(|(i, _)| *i == id))
                .map(|(_, t)| t.clone())
                .unwrap_or_else(|| theme::tr("Choisir").to_string());
            egui::ComboBox::from_id_source("combo_occupation")
                .selected_text(texte)
                .width(460.0)
                .show_ui(ui, |ui| {
                    for (id, texte) in &disponibles {
                        ui.selectable_value(&mut self.selection, Some(*id), texte);
                    }
                });
        });

        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
            if theme::bouton_primaire(ui, theme::tr("Occuper"), 160.0) {
                if let Some(id_reserv) = self.selection {
                    if !donnees.reservation_occupee(id_reserv) {
                        let id = donnees.nouvel_id();
                        donnees.occupations.push(Occupation {
                            id,
                            id_reservation: id_reserv,
                        });
                        message = Some(theme::tr("Occupation enregistrée").to_string());
                    }
                }
            }
            ui.add_space(8.0);
            if theme::bouton_secondaire(ui, theme::tr("Supprimer la sélection"), 200.0) {
                if let Some(id_reserv) = self.selection {
                    donnees
                        .occupations
                        .retain(|o| o.id_reservation != id_reserv);
                    message = Some(theme::tr("Occupation supprimée").to_string());
                }
            }
        });

        ui.add_space(24.0);
        theme::separateur(ui);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(theme::tr("Occupations en cours"))
                    .size(15.0)
                    .color(theme::palette().primaire)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                theme::badge(
                    ui,
                    &format!(
                        "{} {}",
                        donnees.occupations.len(),
                        theme::tr("OCCUPATION(S)")
                    ),
                    theme::palette().fond_bilan,
                    theme::palette().primaire,
                );
            });
        });
        ui.add_space(10.0);

        if donnees.occupations.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(
                    RichText::new(theme::tr("Aucune occupation en cours"))
                        .size(13.0)
                        .color(theme::palette().texte_secondaire),
                );
                ui.add_space(30.0);
            });
            return message;
        }

        let occupations: Vec<Occupation> = donnees.occupations.clone();
        for (i, o) in occupations.iter().enumerate() {
            if let Some(r) = donnees
                .reservations
                .iter()
                .find(|r| r.id == o.id_reservation)
            {
                let (statut, fond_statut) = if r.couvre(today) {
                    (theme::tr("EN COURS"), theme::palette().warning)
                } else if today < r.date_entree {
                    (theme::tr("RÉSERVÉ"), theme::palette().accent)
                } else {
                    (theme::tr("TERMINÉ"), theme::palette().succes)
                };
                let fond = if i % 2 == 0 {
                    theme::palette().fond_carte
                } else {
                    theme::palette().fond
                };
                egui::Frame::none()
                    .fill(fond)
                    .rounding(theme::r_sm())
                    .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(&r.nom_client)
                                    .size(13.0)
                                    .color(theme::palette().texte)
                                    .strong(),
                            );
                            ui.add_space(8.0);
                            theme::badge(
                                ui,
                                &r.numero_chambre,
                                theme::palette().fond_bilan,
                                theme::palette().accent,
                            );
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new(format!(
                                    "{} → {}",
                                    formater_date_fr(r.date_entree),
                                    formater_date_fr(date_sortie(r.date_entree, r.nombre_jours))
                                ))
                                .size(12.0)
                                .color(theme::palette().texte_secondaire),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    theme::badge(ui, statut, fond_statut, egui::Color32::WHITE);
                                },
                            );
                        });
                    });
                ui.add_space(6.0);
            }
        }

        message
    }
}