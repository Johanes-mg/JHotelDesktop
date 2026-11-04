use crate::donnees::Donnees;
use crate::email;
use crate::modele::*;
use crate::theme;
use eframe::egui::{self, RichText};
use chrono::Datelike;

#[derive(Default)]
pub struct VueClients {
    onglet: SousOnglet,
    num_chambre: String,
    date_entree: String,
    nombre_jours: String,
    nom_client: String,
    email_client: String,
    telephone: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SousOnglet {
    #[default]
    Reservations,
    Sejours,
}

impl VueClients {
    pub fn afficher(&mut self, ui: &mut egui::Ui, donnees: &mut Donnees) -> Option<String> {
        ui.horizontal(|ui| {
            if sous_onglet(
                ui,
                theme::tr("Réservations"),
                self.onglet == SousOnglet::Reservations,
            ) {
                self.onglet = SousOnglet::Reservations;
            }
            ui.add_space(8.0);
            if sous_onglet(ui, theme::tr("Séjours"), self.onglet == SousOnglet::Sejours) {
                self.onglet = SousOnglet::Sejours;
            }
        });
        ui.add_space(12.0);

        match self.onglet {
            SousOnglet::Reservations => self.afficher_reservations(ui, donnees),
            SousOnglet::Sejours => self.afficher_sejours(ui, donnees),
        }
    }

    fn formulaire(&mut self, ui: &mut egui::Ui, donnees: &Donnees, avec_email: bool) {
        ui.horizontal_wrapped(|ui| {
            let largeur = ((ui.available_width() - 24.0) / 2.0).max(160.0);
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(theme::tr("Chambre"))
                        .size(12.0)
                        .color(theme::palette().texte_secondaire)
                        .strong(),
                );
                ui.add_space(4.0);
                egui::ComboBox::from_id_source(if avec_email {
                    "chambre_res"
                } else {
                    "chambre_sej"
                })
                .selected_text(if self.num_chambre.is_empty() {
                    theme::tr("Choisir").to_string()
                } else {
                    self.num_chambre.clone()
                })
                .width(largeur)
                .show_ui(ui, |ui| {
                    for c in &donnees.chambres {
                        ui.selectable_value(&mut self.num_chambre, c.numero.clone(), &c.numero);
                    }
                });
            });
            ui.add_space(12.0);
            champ_date_fr(
                ui,
                theme::tr("Date d'entrée"),
                &mut self.date_entree,
                if avec_email { "date_res" } else { "date_sej" },
            );
        });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            let largeur = ((ui.available_width() - 24.0) / 2.0).max(160.0);
            champ(
                ui,
                theme::tr("Nombre de jours"),
                &mut self.nombre_jours,
                largeur,
                if avec_email { "jours_res" } else { "jours_sej" },
            );
            ui.add_space(12.0);
            champ(
                ui,
                theme::tr("Nom du client"),
                &mut self.nom_client,
                largeur,
                if avec_email { "nom_res" } else { "nom_sej" },
            );
        });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            let largeur = ((ui.available_width() - 24.0) / 2.0).max(160.0);
            if avec_email {
                champ(
                    ui,
                    theme::tr("Email"),
                    &mut self.email_client,
                    largeur,
                    "mail_res",
                );
            } else {
                champ(
                    ui,
                    theme::tr("Téléphone"),
                    &mut self.telephone,
                    largeur,
                    "tel_sej",
                );
            }
        });
    }

    fn afficher_reservations(
        &mut self,
        ui: &mut egui::Ui,
        donnees: &mut Donnees,
    ) -> Option<String> {
        if self.date_entree.is_empty() {
            self.date_entree = formater_date_fr(aujourdhui());
        }
        let mut message = None;

        ui.label(
            RichText::new(theme::tr("Nouvelle réservation"))
                .size(15.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(10.0);
        self.formulaire(ui, donnees, true);
        ui.add_space(14.0);
        if theme::bouton_avec_icone(
            ui,
            "sendmail.png",
            theme::tr("Réserver et envoyer email"),
            300.0,
            42.0,
            theme::palette().accent,
        ) {
            if let Some(m) = self.reserver(donnees) {
                message = Some(m);
            }
        }

        ui.add_space(24.0);
        theme::separateur(ui);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(theme::tr("Réservations en cours"))
                    .size(15.0)
                    .color(theme::palette().primaire)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                theme::badge(
                    ui,
                    &format!(
                        "{} {}",
                        donnees.reservations.len(),
                        theme::tr("RÉSERVATION(S)")
                    ),
                    theme::palette().fond_bilan,
                    theme::palette().primaire,
                );
            });
        });
        ui.add_space(10.0);

        if donnees.reservations.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(
                    RichText::new(theme::tr("Aucune réservation"))
                        .size(13.0)
                        .color(theme::palette().texte_secondaire),
                );
                ui.add_space(30.0);
            });
            return message;
        }

        let reservations: Vec<Reservation> = donnees.reservations.clone();
        for (i, r) in reservations.iter().enumerate() {
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
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(theme::tr("Annuler"))
                                            .color(theme::palette().danger)
                                            .size(11.5),
                                    )
                                    .fill(theme::palette().fond_carte)
                                    .stroke(egui::Stroke::new(1.0_f32, theme::palette().danger))
                                    .rounding(theme::r_sm()),
                                )
                                .clicked()
                            {
                                donnees.reservations.retain(|x| x.id != r.id);
                                donnees.occupations.retain(|o| o.id_reservation != r.id);
                                message = Some(theme::tr("Réservation annulée").to_string());
                            }
                        });
                    });
                });
            ui.add_space(6.0);
        }

        message
    }

    fn afficher_sejours(&mut self, ui: &mut egui::Ui, donnees: &mut Donnees) -> Option<String> {
        if self.date_entree.is_empty() {
            self.date_entree = formater_date_fr(aujourdhui());
        }
        let mut message = None;

        ui.label(
            RichText::new(theme::tr("Nouveau séjour"))
                .size(15.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(10.0);
        self.formulaire(ui, donnees, false);
        ui.add_space(14.0);
        if theme::bouton_avec_icone(
            ui,
            "savedays.png",
            theme::tr("Enregistrer le séjour"),
            280.0,
            42.0,
            theme::palette().accent,
        ) {
            if let Some(m) = self.enregistrer_sejour(donnees) {
                message = Some(m);
            }
        }

        ui.add_space(24.0);
        theme::separateur(ui);
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(theme::tr("Séjours enregistrés"))
                    .size(15.0)
                    .color(theme::palette().primaire)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                theme::badge(
                    ui,
                    &format!("{} {}", donnees.sejours.len(), theme::tr("SÉJOUR(S)")),
                    theme::palette().fond_bilan,
                    theme::palette().primaire,
                );
            });
        });
        ui.add_space(10.0);

        if donnees.sejours.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(
                    RichText::new(theme::tr("Aucun séjour"))
                        .size(13.0)
                        .color(theme::palette().texte_secondaire),
                );
                ui.add_space(30.0);
            });
            return message;
        }

        let sejours: Vec<Sejour> = donnees.sejours.clone();
        for (i, s) in sejours.iter().enumerate() {
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
                            RichText::new(&s.nom_client)
                                .size(13.0)
                                .color(theme::palette().texte)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        theme::badge(
                            ui,
                            &s.numero_chambre,
                            theme::palette().fond_bilan,
                            theme::palette().accent,
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!(
                                "{} → {}",
                                formater_date_fr(s.date_entree),
                                formater_date_fr(date_sortie(s.date_entree, s.nombre_jours))
                            ))
                            .size(12.0)
                            .color(theme::palette().texte_secondaire),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let mut recu = false;
                            let mut suppr = false;
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(theme::tr("Suppr."))
                                            .color(theme::palette().danger)
                                            .size(11.5),
                                    )
                                    .fill(theme::palette().fond_carte)
                                    .stroke(egui::Stroke::new(1.0_f32, theme::palette().danger))
                                    .rounding(theme::r_sm()),
                                )
                                .clicked()
                            {
                                suppr = true;
                            }
                            ui.add_space(6.0);
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(theme::tr("Reçu"))
                                            .color(theme::palette().accent)
                                            .size(11.5),
                                    )
                                    .fill(theme::palette().fond_carte)
                                    .stroke(egui::Stroke::new(1.0_f32, theme::palette().accent))
                                    .rounding(theme::r_sm()),
                                )
                                .clicked()
                            {
                                recu = true;
                            }
                            if recu {
                                let chemin = self.generer_recu(s, donnees);
                                message =
                                    Some(format!("{} : {}", theme::tr("Reçu généré"), chemin));
                            }
                            if suppr {
                                donnees.sejours.retain(|x| x.id != s.id);
                                message = Some(theme::tr("Séjour supprimé").to_string());
                            }
                        });
                    });
                });
            ui.add_space(6.0);
        }

        message
    }

    fn reserver(&mut self, donnees: &mut Donnees) -> Option<String> {
        let num = self.num_chambre.trim().to_string();
        let nom = self.nom_client.trim().to_string();
        let mail = self.email_client.trim().to_string();
        let jours: i64 = self.nombre_jours.trim().parse().ok()?;
        let date = parser_date_fr(&self.date_entree)?;
        if num.is_empty() || nom.is_empty() || mail.is_empty() {
            return Some(theme::tr("Champs obligatoires manquants").to_string());
        }
        if !donnees.est_disponible(&num, date, jours) {
            return Some(theme::tr("Chambre non disponible pour ces dates").to_string());
        }
        let id = donnees.nouvel_id();
        donnees.reservations.push(Reservation {
            id,
            numero_chambre: num.clone(),
            date_entree: date,
            nombre_jours: jours,
            nom_client: nom.clone(),
            email: mail.clone(),
            creee_le: aujourdhui(),
        });
        let sortie = date_sortie(date, jours);
        let corps = format!(
            "Bonjour {},\n\nVotre reservation est confirmee.\n\nChambre : {}\nEntree : {}\nJours : {}\nSortie : {}\n\nJHotel",
            nom,
            num,
            formater_date_fr(date),
            jours,
            formater_date_fr(sortie)
        );
        email::ouvrir_mailto(&mail, "Confirmation de reservation - JHotel", &corps);
        self.vider();
        Some(theme::tr("Réservation enregistrée, email préparé").to_string())
    }

    fn enregistrer_sejour(&mut self, donnees: &mut Donnees) -> Option<String> {
        let num = self.num_chambre.trim().to_string();
        let nom = self.nom_client.trim().to_string();
        let tel = self.telephone.trim().to_string();
        let jours: i64 = self.nombre_jours.trim().parse().ok()?;
        let date = parser_date_fr(&self.date_entree)?;
        if num.is_empty() || nom.is_empty() || tel.is_empty() {
            return Some(theme::tr("Champs obligatoires manquants").to_string());
        }
        if !donnees.est_disponible(&num, date, jours) {
            return Some(theme::tr("Chambre non disponible").to_string());
        }
        let id = donnees.nouvel_id();
        donnees.sejours.push(Sejour {
            id,
            numero_chambre: num,
            date_entree: date,
            nombre_jours: jours,
            nom_client: nom,
            telephone: tel,
        });
        self.vider();
        Some(theme::tr("Séjour enregistré").to_string())
    }

    fn generer_recu(&self, sejour: &Sejour, donnees: &Donnees) -> String {
        let dossier = std::env::var("USERPROFILE")
            .map(|p| std::path::PathBuf::from(p).join("Documents").join("JHotel"))
            .unwrap_or_else(|_| std::env::temp_dir().join("JHotel"));
        let _ = std::fs::create_dir_all(&dossier);
        let nom_fichier = format!(
            "Recu_{}_{}.txt",
            sejour.nom_client.replace(' ', "_"),
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );
        let chemin = dossier.join(&nom_fichier);
        let prix = donnees
            .trouver_chambre(&sejour.numero_chambre)
            .map(|c| c.prix)
            .unwrap_or(0);
        let contenu = format!(
            "RECU DE SEJOUR\n\nClient : {}\nChambre : {}\nDate entree : {}\nNombre de jours : {}\nDate sortie : {}\nPrix unitaire : {}\nTotal : {}\n\nMerci de votre visite.\nJHotel",
            sejour.nom_client,
            sejour.numero_chambre,
            formater_date_fr(sejour.date_entree),
            sejour.nombre_jours,
            formater_date_fr(date_sortie(sejour.date_entree, sejour.nombre_jours)),
            prix,
            prix * sejour.nombre_jours
        );
        let _ = std::fs::write(&chemin, contenu);
        chemin.display().to_string()
    }

    fn vider(&mut self) {
        self.nom_client.clear();
        self.email_client.clear();
        self.telephone.clear();
        self.nombre_jours.clear();
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

fn sous_onglet(ui: &mut egui::Ui, texte: &str, actif: bool) -> bool {
    let texte_final = if actif {
        RichText::new(texte)
            .color(egui::Color32::WHITE)
            .size(13.0)
            .strong()
    } else {
        RichText::new(texte)
            .color(theme::palette().primaire)
            .size(13.0)
            .strong()
    };
    let fond = if actif {
        theme::palette().accent
    } else {
        theme::palette().fond_carte
    };
    let bordure = if actif {
        theme::palette().accent
    } else {
        theme::palette().bordure
    };
    ui.add(
        egui::Button::new(texte_final)
            .fill(fond)
            .stroke(egui::Stroke::new(1.0_f32, bordure))
            .rounding(theme::r_sm())
            .min_size(egui::vec2(140.0, 34.0)),
    )
    .clicked()
}