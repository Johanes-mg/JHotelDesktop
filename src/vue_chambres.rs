use crate::donnees::Donnees;
use crate::modele::*;
use crate::theme;
use eframe::egui::{self, RichText};

#[derive(Default)]
pub struct VueChambres {
    numero: String,
    designation: String,
    type_chambre: String,
    prix: String,
    selection: Option<String>,
}

const TYPES_CHAMBRE: [&str; 3] = ["Economique", "Standard", "Luxe"];

fn type_libelle(t: &str) -> &'static str {
    match t {
        "Economique" => theme::tr("Economique"),
        "Standard" => theme::tr("Standard"),
        "Luxe" => theme::tr("Luxe"),
        _ => "?",
    }
}

impl VueChambres {
    pub fn afficher(&mut self, ui: &mut egui::Ui, donnees: &mut Donnees) -> bool {
        let mut modifie = false;

        ui.label(
            RichText::new(theme::tr("Nouvelle chambre"))
                .size(15.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(10.0);

        ui.horizontal_wrapped(|ui| {
            let largeur = ((ui.available_width() - 24.0) / 2.0).max(160.0);
            champ(ui, theme::tr("Numéro"), &mut self.numero, largeur, "ch_num");
            ui.add_space(12.0);
            champ(
                ui,
                theme::tr("Désignation"),
                &mut self.designation,
                largeur,
                "ch_design",
            );
        });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            let largeur = ((ui.available_width() - 24.0) / 2.0).max(160.0);
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(theme::tr("Type"))
                        .size(12.0)
                        .color(theme::palette().texte_secondaire)
                        .strong(),
                );
                ui.add_space(4.0);
                if self.type_chambre.is_empty() {
                    self.type_chambre = "Standard".to_string();
                }
                egui::ComboBox::from_id_source("ch_type")
                    .selected_text(type_libelle(&self.type_chambre))
                    .width(largeur)
                    .show_ui(ui, |ui| {
                        for t in TYPES_CHAMBRE {
                            ui.selectable_value(
                                &mut self.type_chambre,
                                t.to_string(),
                                type_libelle(t),
                            );
                        }
                    });
            });
            ui.add_space(12.0);
            champ(ui, theme::tr("Prix"), &mut self.prix, largeur, "ch_prix");
        });

        ui.add_space(14.0);
        ui.horizontal(|ui| {
            let largeur_bouton = ((ui.available_width() - 16.0) / 3.0).max(80.0);
            if theme::bouton_couleur(
                ui,
                theme::tr("Ajouter"),
                largeur_bouton,
                40.0,
                theme::palette().accent,
            ) {
                if self.ajouter(donnees) {
                    modifie = true;
                }
            }
            ui.add_space(8.0);
            let peut_modifier = self
                .selection
                .as_deref()
                .map(|n| {
                    !donnees.chambre_occupee_a(n, aujourdhui())
                        && !donnees.chambre_reservee_a(n, aujourdhui())
                })
                .unwrap_or(false);
            if peut_modifier {
                if theme::bouton_couleur(
                    ui,
                    theme::tr("Modifier"),
                    largeur_bouton,
                    40.0,
                    theme::palette().warning,
                ) {
                    if self.modifier(donnees) {
                        modifie = true;
                    }
                }
                ui.add_space(8.0);
                if theme::bouton_couleur(
                    ui,
                    theme::tr("Supprimer"),
                    largeur_bouton,
                    40.0,
                    theme::palette().danger,
                ) {
                    if self.supprimer(donnees) {
                        modifie = true;
                    }
                }
            } else {
                theme::bouton_desactive_couleur(
                    ui,
                    theme::tr("Modifier"),
                    largeur_bouton,
                    40.0,
                );
                ui.add_space(8.0);
                theme::bouton_desactive_couleur(
                    ui,
                    theme::tr("Supprimer"),
                    largeur_bouton,
                    40.0,
                );
            }
        });

        ui.add_space(24.0);
        theme::separateur(ui);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(theme::tr("Liste des chambres"))
                    .size(15.0)
                    .color(theme::palette().primaire)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                theme::badge(
                    ui,
                    &format!("{} {}", donnees.chambres.len(), theme::tr("CHAMBRE(S)")),
                    theme::palette().fond_bilan,
                    theme::palette().primaire,
                );
            });
        });
        ui.add_space(10.0);

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
            return modifie;
        }

        let today = aujourdhui();
        let chambres: Vec<Chambre> = donnees.chambres.clone();

        for (i, c) in chambres.iter().enumerate() {
            let statut = donnees.statut_chambre(&c.numero, today);
            let fond_ligne = if self.selection.as_deref() == Some(c.numero.as_str()) {
                theme::palette().fond_bilan
            } else if i % 2 == 0 {
                theme::palette().fond_carte
            } else {
                theme::palette().fond
            };

            let (rect, resp) = ui
                .allocate_exact_size(egui::vec2(ui.available_width(), 44.0), egui::Sense::click());

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                painter.rect_filled(rect, theme::r_sm(), fond_ligne);

                let largeur = rect.width();
                let pad = 12.0;
                let largeur_statut = 110.0;
                let largeur_num = 80.0;
                let largeur_type = 120.0;
                let largeur_prix = 120.0;
                let largeur_design = (largeur
                    - pad * 2.0
                    - largeur_statut
                    - largeur_num
                    - largeur_type
                    - largeur_prix
                    - 32.0)
                    .max(60.0);

                let mut x = rect.left() + pad;

                let (bg_statut, fg_statut) = theme::badge_statut(statut.libelle());
                let badge_rect = egui::Rect::from_min_size(
                    egui::pos2(x, rect.center().y - 10.0),
                    egui::vec2(88.0, 20.0),
                );
                painter.rect_filled(badge_rect, theme::r_sm(), bg_statut);
                painter.text(
                    badge_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    theme::tr(statut.libelle()),
                    egui::FontId::proportional(10.5),
                    fg_statut,
                );
                x += largeur_statut;

                painter.text(
                    egui::pos2(x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    &c.numero,
                    egui::FontId::proportional(12.5),
                    theme::palette().texte,
                );
                x += largeur_num;

                painter.text(
                    egui::pos2(x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    &c.designation,
                    egui::FontId::proportional(12.0),
                    theme::palette().texte,
                );
                x += largeur_design + 8.0;

                painter.text(
                    egui::pos2(x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    type_libelle(&c.type_chambre),
                    egui::FontId::proportional(12.0),
                    theme::palette().texte_secondaire,
                );
                x += largeur_type;

                painter.text(
                    egui::pos2(x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    format!("{}", c.prix),
                    egui::FontId::proportional(12.0),
                    theme::palette().texte,
                );
            }

            if resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if resp.clicked() {
                self.selection = Some(c.numero.clone());
                self.numero = c.numero.clone();
                self.designation = c.designation.clone();
                self.type_chambre = c.type_chambre.clone();
                self.prix = c.prix.to_string();
            }

            ui.add_space(2.0);
        }

        modifie
    }

    fn ajouter(&mut self, donnees: &mut Donnees) -> bool {
        let num = self.numero.trim().to_string();
        let design = self.designation.trim().to_string();
        let prix: i64 = match self.prix.trim().parse() {
            Ok(v) => v,
            Err(_) => return false,
        };
        if num.is_empty() || design.is_empty() || donnees.chambre_existe(&num) {
            return false;
        }
        donnees.chambres.push(Chambre {
            numero: num,
            designation: design,
            type_chambre: self.type_chambre.clone(),
            prix,
        });
        self.vider();
        true
    }

    fn modifier(&mut self, donnees: &mut Donnees) -> bool {
        let ancien = match &self.selection {
            Some(n) => n.clone(),
            None => return false,
        };
        let num = self.numero.trim().to_string();
        let design = self.designation.trim().to_string();
        let prix: i64 = match self.prix.trim().parse() {
            Ok(v) => v,
            Err(_) => return false,
        };
        if num.is_empty() || design.is_empty() || (num != ancien && donnees.chambre_existe(&num)) {
            return false;
        }
        if let Some(c) = donnees.chambres.iter_mut().find(|c| c.numero == ancien) {
            c.numero = num;
            c.designation = design;
            c.type_chambre = self.type_chambre.clone();
            c.prix = prix;
        }
        self.selection = None;
        self.vider();
        true
    }

    fn supprimer(&mut self, donnees: &mut Donnees) -> bool {
        let num = match &self.selection {
            Some(n) => n.clone(),
            None => return false,
        };
        if donnees.chambre_occupee_a(&num, aujourdhui())
            || donnees.chambre_reservee_a(&num, aujourdhui())
        {
            return false;
        }
        donnees.chambres.retain(|c| c.numero != num);
        donnees.reservations.retain(|r| r.numero_chambre != num);
        donnees.sejours.retain(|s| s.numero_chambre != num);
        self.selection = None;
        self.vider();
        true
    }

    fn vider(&mut self) {
        self.numero.clear();
        self.designation.clear();
        self.type_chambre = "Standard".to_string();
        self.prix.clear();
        self.selection = None;
    }
}

fn champ(ui: &mut egui::Ui, label: &str, valeur: &mut String, largeur: f32, id: &str) {
    ui.vertical(|ui| {
        ui.label(
            RichText::new(label)
                .size(12.0)
                .color(theme::palette().texte_secondaire)
                .strong(),
        );
        ui.add_space(4.0);
        ui.add(
            egui::TextEdit::singleline(valeur)
                .id(egui::Id::new(id))
                .desired_width(largeur)
                .margin(egui::vec2(12.0, 9.0)),
        );
    });
}