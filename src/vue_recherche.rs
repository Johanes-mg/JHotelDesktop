use crate::donnees::Donnees;
use crate::modele::*;
use crate::theme;
use eframe::egui::{self, RichText};
use chrono::Datelike;

#[derive(Default)]
pub struct VueRecherche {
    date_recherche: String,
    num_chambre: String,
}

fn type_libelle(t: &str) -> &'static str {
    match t {
        "Economique" => theme::tr("Economique"),
        "Standard" => theme::tr("Standard"),
        "Luxe" => theme::tr("Luxe"),
        _ => "?",
    }
}

impl VueRecherche {
    pub fn afficher(&mut self, ui: &mut egui::Ui, donnees: &Donnees) {
        if self.date_recherche.is_empty() {
            self.date_recherche = formater_date_fr(aujourdhui());
        }

        ui.horizontal_wrapped(|ui| {
            champ_date_fr(ui, theme::tr("Date d'entrée"), &mut self.date_recherche, "rech_date");
            ui.add_space(16.0);
            ui.label(
                RichText::new(theme::tr("Chambre"))
                    .size(12.0)
                    .color(theme::palette().texte_secondaire)
                    .strong(),
            );
            ui.add_space(6.0);
            egui::ComboBox::from_id_source("recherche_chambre")
                .selected_text(if self.num_chambre.is_empty() {
                    theme::tr("Toutes").to_string()
                } else {
                    self.num_chambre.clone()
                })
                .width(160.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.num_chambre, String::new(), theme::tr("Toutes"));
                    for c in &donnees.chambres {
                        ui.selectable_value(&mut self.num_chambre, c.numero.clone(), &c.numero);
                    }
                });
        });

        ui.add_space(24.0);
        theme::separateur(ui);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(theme::tr("Disponibilités"))
                    .size(15.0)
                    .color(theme::palette().primaire)
                    .strong(),
            );
        });
        ui.add_space(10.0);

        let date = parser_date_fr(&self.date_recherche).unwrap_or_else(aujourdhui);
        if donnees.chambres.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(
                    RichText::new(theme::tr("Aucune chambre enregistrée"))
                        .size(13.0)
                        .color(theme::palette().texte_secondaire),
                );
                ui.add_space(30.0);
            });
            return;
        }

        let chambres: Vec<Chambre> = donnees
            .chambres
            .iter()
            .filter(|c| self.num_chambre.is_empty() || c.numero == self.num_chambre)
            .cloned()
            .collect();

        if chambres.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(
                    RichText::new(theme::tr("Aucun résultat"))
                        .size(13.0)
                        .color(theme::palette().texte_secondaire),
                );
                ui.add_space(20.0);
            });
            return;
        }

        for (i, c) in chambres.iter().enumerate() {
            let statut = donnees.statut_chambre(&c.numero, date);
            let (bg_statut, fg_statut) = theme::badge_statut(statut.libelle());
            let occupant = donnees
                .details_occupant(&c.numero, date)
                .map(|(nom, d, f)| {
                    if theme::anglais() {
                        format!(
                            "{} from {} to {}",
                            nom,
                            formater_date_fr(d),
                            formater_date_fr(f)
                        )
                    } else {
                        format!(
                            "{} du {} au {}",
                            nom,
                            formater_date_fr(d),
                            formater_date_fr(f)
                        )
                    }
                })
                .unwrap_or_default();
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
                        theme::badge(ui, theme::tr(statut.libelle()), bg_statut, fg_statut);
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(&c.numero)
                                .size(13.0)
                                .color(theme::palette().texte)
                                .strong(),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(&c.designation)
                                .size(12.0)
                                .color(theme::palette().texte_secondaire),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(type_libelle(&c.type_chambre))
                                .size(12.0)
                                .color(theme::palette().texte_secondaire),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(format!("{}", c.prix))
                                .size(12.0)
                                .color(theme::palette().texte)
                                .strong(),
                        );
                        if !occupant.is_empty() {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(occupant)
                                            .size(11.5)
                                            .color(theme::palette().texte_secondaire),
                                    );
                                },
                            );
                        }
                    });
                });
            ui.add_space(6.0);
        }
    }
}

fn champ_date_fr(ui: &mut egui::Ui, label: &str, valeur: &mut String, id: &str) {
    let jour_lu = parser_date_fr(valeur).unwrap_or_else(aujourdhui);
    let mut jour = jour_lu.day();
    let mut mois = jour_lu.month();
    let mut annee = jour_lu.year();
    let mut change = false;

    let (lbl_j, lbl_m, lbl_a) = if theme::anglais() {
        ("DD", "MM", "YYYY")
    } else {
        ("JJ", "MM", "AAAA")
    };

    ui.vertical(|ui| {
        ui.label(
            RichText::new(label)
                .size(12.0)
                .color(theme::palette().texte_secondaire)
                .strong(),
        );
        ui.add_space(4.0);
        ui.push_id(id, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(lbl_j)
                        .size(10.0)
                        .color(theme::palette().texte_secondaire)
                        .strong(),
                );
                if ui
                    .add(
                        egui::DragValue::new(&mut jour)
                            .range(1..=31)
                            .speed(1)
                            .custom_formatter(|n, _| format!("{:02}", n as i64)),
                    )
                    .changed()
                {
                    change = true;
                }
                ui.add_space(4.0);
                ui.label(
                    RichText::new(lbl_m)
                        .size(10.0)
                        .color(theme::palette().texte_secondaire)
                        .strong(),
                );
                if ui
                    .add(
                        egui::DragValue::new(&mut mois)
                            .range(1..=12)
                            .speed(1)
                            .custom_formatter(|n, _| format!("{:02}", n as i64)),
                    )
                    .changed()
                {
                    change = true;
                }
                ui.add_space(4.0);
                ui.label(
                    RichText::new(lbl_a)
                        .size(10.0)
                        .color(theme::palette().texte_secondaire)
                        .strong(),
                );
                if ui
                    .add(
                        egui::DragValue::new(&mut annee)
                            .range(1900..=2100)
                            .speed(1),
                    )
                    .changed()
                {
                    change = true;
                }
            });
        });
    });

    if change {
        if let Some(d) = chrono::NaiveDate::from_ymd_opt(annee, mois, jour) {
            *valeur = formater_date_fr(d);
        }
    }
}