use crate::donnees::Donnees;
use crate::modele::*;
use crate::persistance;
use crate::theme;
use crate::vue_chambres::VueChambres;
use crate::vue_clients::VueClients;
use crate::vue_occupations::VueOccupations;
use crate::vue_recherche::VueRecherche;
use chrono::Datelike;
use eframe::egui::{self, Color32, RichText};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Onglet {
    Accueil,
    Chambres,
    Clients,
    Occupations,
    Recherche,
    Parametres,
}

impl Onglet {
    fn image(self) -> &'static str {
        match self {
            Onglet::Accueil => "accueil.png",
            Onglet::Chambres => "chambres.png",
            Onglet::Clients => "clients.png",
            Onglet::Occupations => "occupations.png",
            Onglet::Recherche => "recherche.png",
            Onglet::Parametres => "parametres.png",
        }
    }
    fn libelle(self) -> &'static str {
        match self {
            Onglet::Accueil => "Accueil",
            Onglet::Chambres => "Chambres",
            Onglet::Clients => "Clients",
            Onglet::Occupations => "Occupations",
            Onglet::Recherche => "Recherche",
            Onglet::Parametres => "Paramètres",
        }
    }
    fn libelle_en(self) -> &'static str {
        match self {
            Onglet::Accueil => "Home",
            Onglet::Chambres => "Rooms",
            Onglet::Clients => "Guests",
            Onglet::Occupations => "Occupancy",
            Onglet::Recherche => "Search",
            Onglet::Parametres => "Settings",
        }
    }
}

pub struct JHotelApp {
    pub donnees: Donnees,
    pub onglet: Onglet,
    pub vue_chambres: VueChambres,
    pub vue_clients: VueClients,
    pub vue_occupations: VueOccupations,
    pub vue_recherche: VueRecherche,
    pub message: Option<(String, Color32)>,
    theme_applique: ThemeApp,
    donnees_modifiees: bool,
    message_parametres: Option<(String, Color32)>,
}

impl JHotelApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (donnees, erreur_chargement) = persistance::charger();
        let theme_applique = donnees.parametres.theme;
        theme::configurer_langue(donnees.parametres.langue == LangueApp::English);
        theme::configurer_style(&cc.egui_ctx, theme_applique == ThemeApp::Sombre);
        Self {
            donnees,
            onglet: Onglet::Accueil,
            vue_chambres: VueChambres::default(),
            vue_clients: VueClients::default(),
            vue_occupations: VueOccupations::default(),
            vue_recherche: VueRecherche::default(),
            message: erreur_chargement.map(|m| (m, theme::palette().danger)),
            theme_applique,
            donnees_modifiees: false,
            message_parametres: None,
        }
    }

    pub fn notifier(&mut self, texte: impl Into<String>, couleur: Color32) {
        self.message = Some((texte.into(), couleur));
    }

    fn logo_affiche(&self) -> String {
        if let Some(chemin) = &self.donnees.parametres.logo_personnalise {
            if std::path::Path::new(chemin).exists() {
                return chemin.clone();
            }
        }
        "logo.png".to_string()
    }

    fn nom_application(&self) -> String {
        let nom = self.donnees.parametres.nom_application.trim();
        if nom.is_empty() {
            "JHotel".to_string()
        } else {
            nom.to_string()
        }
    }

    fn barre_haut(&mut self, ctx: &egui::Context) {
        let largeur = ctx.available_rect().width();
        let compact = largeur < 860.0;
        let logo = self.logo_affiche();
        let nom = self.nom_application();

        egui::TopBottomPanel::top("topbar")
            .exact_height(68.0)
            .frame(egui::Frame::none().fill(theme::palette().topbar_ciel))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(16.0);
                    theme::logo_cadre(ui, &logo, 44.0, 1.5_f32, theme::palette().logo_contour);
                    ui.add_space(12.0);

                    if !compact {
                        ui.vertical(|ui| {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new(nom)
                                    .size(17.0)
                                    .color(theme::palette().primaire)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(
                                    if self.donnees.parametres.langue == LangueApp::English {
                                        "Hotel management"
                                    } else {
                                        "Gestion hoteliere"
                                    },
                                )
                                .size(11.5)
                                .color(theme::palette().texte_secondaire),
                            );
                        });
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(16.0);
                        egui::ScrollArea::horizontal()
                            .id_source("nav_scroll")
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let onglets = [
                                        Onglet::Parametres,
                                        Onglet::Recherche,
                                        Onglet::Occupations,
                                        Onglet::Clients,
                                        Onglet::Chambres,
                                        Onglet::Accueil,
                                    ];
                                    for o in onglets {
                                        let actif = self.onglet == o;
                                        let libelle = match self.donnees.parametres.langue {
                                            LangueApp::English => o.libelle_en(),
                                            _ => o.libelle(),
                                        };
                                        if theme::bouton_nav(ui, o.image(), libelle, actif) {
                                            self.onglet = o;
                                        }
                                    }
                                });
                            });
                    });
                });
            });
    }

    fn barre_bas(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("statut")
            .exact_height(30.0)
            .frame(
                egui::Frame::none()
                    .fill(theme::palette().fond_carte)
                    .stroke(egui::Stroke::new(1.0_f32, theme::palette().bordure)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(16.0);
                    let solde = self.donnees.solde();
                    ui.label(
                        RichText::new(match self.donnees.parametres.langue {
                            LangueApp::English => format!("Current balance: {}", solde),
                            _ => format!("Solde actuel : {}", solde),
                        })
                        .size(12.5)
                        .color(theme::palette().succes)
                        .strong(),
                    );
                    ui.add_space(20.0);
                    if let Some((msg, coul)) = &self.message {
                        ui.label(RichText::new(msg).size(12.0).color(*coul));
                    }
                });
            });
    }

    fn contenu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(theme::palette().fond)
                    .inner_margin(egui::Margin::symmetric(24.0, 18.0)),
            )
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| match self.onglet {
                        Onglet::Accueil => self.page_accueil(ui),
                        Onglet::Chambres => {
                            if self.vue_chambres.afficher(ui, &mut self.donnees) {
                                self.donnees_modifiees = true;
                                self.notifier("Chambre enregistree", theme::palette().succes);
                            }
                        }
                        Onglet::Clients => {
                            if let Some(msg) = self.vue_clients.afficher(ui, &mut self.donnees) {
                                self.donnees_modifiees = true;
                                self.notifier(msg, theme::palette().accent);
                            }
                        }
                        Onglet::Occupations => {
                            if let Some(msg) = self.vue_occupations.afficher(ui, &mut self.donnees)
                            {
                                self.donnees_modifiees = true;
                                self.notifier(msg, theme::palette().accent);
                            }
                        }
                        Onglet::Recherche => {
                            self.vue_recherche.afficher(ui, &self.donnees);
                        }
                        Onglet::Parametres => self.page_parametres(ui),
                    });
            });
    }

    fn page_accueil(&mut self, ui: &mut egui::Ui) {
        let anglais = self.donnees.parametres.langue == LangueApp::English;
        let nom = self.nom_application();

        ui.add_space(50.0);

        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(if anglais {
                    format!("Welcome to {}", nom)
                } else {
                    format!("Bienvenue dans {}", nom)
                })
                .size(44.0)
                .color(theme::palette().primaire)
                .strong(),
            );
            ui.add_space(14.0);

            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(220.0, 3.0),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(rect, 1.5, theme::palette().accent);
        });

        ui.add_space(46.0);

        let maintenant =
            chrono::Utc::now().naive_utc() + chrono::Duration::hours(OFFSET_HEURES);
        let date_txt = format!(
            "{} {} {} {}",
            nom_jour(maintenant.weekday(), anglais),
            maintenant.day(),
            nom_mois(maintenant.month(), anglais),
            maintenant.year()
        );
        let heure_txt = maintenant.format("%H:%M:%S").to_string();
        let lbl_date = "DATE";
        let lbl_heure = if anglais { "TIME" } else { "HEURE" };

        let largeur_dispo = ui.available_width();
        let seuil = 680.0_f32;

        if largeur_dispo < seuil {
            ui.vertical_centered(|ui| {
                bloc_texte_centre(ui, lbl_date, &date_txt, 20.0, theme::palette().primaire);
                ui.add_space(22.0);
                bloc_texte_centre(ui, lbl_heure, &heure_txt, 38.0, theme::palette().accent);
            });
        } else {
            let hauteur_label = 14.0_f32;
            let hauteur_date = 26.0_f32;
            let hauteur_heure = 48.0_f32;
            let hauteur_bloc = hauteur_label + 6.0 + hauteur_heure;

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    let galerie_date = ui.fonts(|f| {
                        f.layout_no_wrap(
                            date_txt.clone(),
                            egui::FontId::proportional(20.0),
                            theme::palette().primaire,
                        )
                    });
                    let galerie_heure = ui.fonts(|f| {
                        f.layout_no_wrap(
                            heure_txt.clone(),
                            egui::FontId::proportional(38.0),
                            theme::palette().accent,
                        )
                    });
                    let largeur_date = galerie_date.size().x.max(140.0);
                    let largeur_heure = galerie_heure.size().x.max(160.0);
                    let gap = 44.0_f32;
                    let total = largeur_date + gap + largeur_heure;
                    let marge_gauche = ((largeur_dispo - total) / 2.0).max(0.0);
                    ui.add_space(marge_gauche);

                    ui.allocate_ui(egui::vec2(largeur_date, hauteur_bloc), |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space((hauteur_bloc - hauteur_label - 6.0 - hauteur_date) / 2.0);
                            ui.label(
                                RichText::new(lbl_date)
                                    .size(11.0)
                                    .color(theme::palette().texte_secondaire)
                                    .strong(),
                            );
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(&date_txt)
                                    .size(20.0)
                                    .color(theme::palette().primaire)
                                    .strong(),
                            );
                        });
                    });

                    ui.allocate_ui(egui::vec2(gap, hauteur_bloc), |ui| {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(gap, hauteur_bloc),
                            egui::Sense::hover(),
                        );
                        let ligne_rect = egui::Rect::from_center_size(
                            rect.center(),
                            egui::vec2(1.0, 64.0),
                        );
                        ui.painter().rect_filled(
                            ligne_rect,
                            0.0,
                            theme::palette().bordure,
                        );
                    });

                    ui.allocate_ui(egui::vec2(largeur_heure, hauteur_bloc), |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space((hauteur_bloc - hauteur_label - 6.0 - hauteur_heure) / 2.0);
                            ui.label(
                                RichText::new(lbl_heure)
                                    .size(11.0)
                                    .color(theme::palette().texte_secondaire)
                                    .strong(),
                            );
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(&heure_txt)
                                    .size(38.0)
                                    .color(theme::palette().accent)
                                    .strong(),
                            );
                        });
                    });
                });
            });
        }

        ui.add_space(46.0);
        ui.vertical_centered(|ui| {
            let largeur_sep = (largeur_dispo - 40.0).clamp(0.0, 520.0);
            let (rect_sep, _) = ui.allocate_exact_size(
                egui::vec2(largeur_sep, 1.0),
                egui::Sense::hover(),
            );
            ui.painter()
                .rect_filled(rect_sep, 0.0, theme::palette().bordure);
        });
        ui.add_space(28.0);

        let solde = self.donnees.solde();
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(format!("{}", solde))
                    .size(40.0)
                    .color(theme::palette().succes)
                    .strong(),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new(if anglais {
                    "Today's balance"
                } else {
                    "Solde du jour"
                })
                .size(12.0)
                .color(theme::palette().texte_secondaire),
            );
            ui.add_space(50.0);
        });
    }

    fn page_parametres(&mut self, ui: &mut egui::Ui) {
        let parametres_avant = (
            self.donnees.parametres.theme,
            self.donnees.parametres.langue,
            self.donnees.parametres.nom_application.clone(),
            self.donnees.parametres.logo_personnalise.clone(),
        );
        let anglais = self.donnees.parametres.langue == LangueApp::English;

        ui.label(
            RichText::new(if anglais { "Settings" } else { "Paramètres" })
                .size(22.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(6.0);
        ui.label(
            RichText::new(if anglais {
                "Customize the appearance and language of JHotel."
            } else {
                "Personnalisez l'apparence et la langue de JHotel."
            })
            .size(13.0)
            .color(theme::palette().texte_secondaire),
        );

        ui.add_space(24.0);
        theme::separateur(ui);
        ui.add_space(18.0);

        ui.label(
            RichText::new(if anglais { "Identity" } else { "Identité" })
                .size(16.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(10.0);

        ui.label(
            RichText::new(if anglais {
                "Application name"
            } else {
                "Nom de l'application"
            })
            .size(12.5)
            .color(theme::palette().texte_secondaire)
            .strong(),
        );
        ui.add_space(4.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.donnees.parametres.nom_application)
                .id(egui::Id::new("param_nom_application"))
                .desired_width(320.0)
                .hint_text("JHotel")
                .margin(egui::vec2(12.0, 9.0)),
        );

        ui.add_space(18.0);
        ui.label(
            RichText::new(if anglais {
                "Custom logo"
            } else {
                "Logo personnalisé"
            })
            .size(12.5)
            .color(theme::palette().texte_secondaire)
            .strong(),
        );
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            let logo = self.logo_affiche();
            theme::logo_cadre(ui, &logo, 64.0, 1.0_f32, theme::palette().logo_contour);
            ui.add_space(12.0);
            ui.vertical(|ui| {
                if theme::bouton_primaire(
                    ui,
                    if anglais {
                        "Choose an image"
                    } else {
                        "Choisir une image"
                    },
                    200.0,
                ) {
                    if let Some(source) = rfd::FileDialog::new()
                        .set_title(if anglais {
                            "Choose an image"
                        } else {
                            "Choisir une image"
                        })
                        .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "ico"])
                        .pick_file()
                    {
                        match copier_logo_personnalise(&source) {
                            Ok(chemin) => {
                                theme::vider_cache();
                                self.donnees.parametres.logo_personnalise = Some(chemin);
                                self.donnees_modifiees = true;
                                self.message_parametres = Some((
                                    if anglais {
                                        "Logo updated.".to_string()
                                    } else {
                                        "Logo mis à jour.".to_string()
                                    },
                                    theme::palette().succes,
                                ));
                            }
                            Err(e) => {
                                self.message_parametres =
                                    Some((e, theme::palette().danger));
                            }
                        }
                    }
                }
                ui.add_space(6.0);
                if self.donnees.parametres.logo_personnalise.is_some() {
                    if theme::bouton_secondaire(
                        ui,
                        if anglais { "Reset" } else { "Réinitialiser" },
                        200.0,
                    ) {
                        theme::vider_cache();
                        self.donnees.parametres.logo_personnalise = None;
                        self.donnees_modifiees = true;
                        self.message_parametres = Some((
                            if anglais {
                                "Custom logo removed.".to_string()
                            } else {
                                "Logo personnalisé retiré.".to_string()
                            },
                            theme::palette().succes,
                        ));
                    }
                } else {
                    ui.label(
                        RichText::new(if anglais {
                            "No custom logo"
                        } else {
                            "Aucun logo personnalisé"
                        })
                        .size(11.5)
                        .color(theme::palette().texte_secondaire),
                    );
                }
                if let Some((msg, coul)) = &self.message_parametres {
                    ui.add_space(6.0);
                    ui.label(RichText::new(msg).size(11.5).color(*coul));
                }
            });
        });

        ui.add_space(28.0);
        theme::separateur(ui);
        ui.add_space(18.0);

        ui.label(
            RichText::new(if anglais { "Appearance" } else { "Apparence" })
                .size(16.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(8.0);
        ui.label(
            RichText::new(if anglais {
                "Application theme"
            } else {
                "Thème de l'application"
            })
            .size(12.5)
            .color(theme::palette().texte_secondaire),
        );
        ui.add_space(6.0);
        let mut theme_selection = self.donnees.parametres.theme;
        ui.horizontal(|ui| {
            if ui
                .selectable_value(
                    &mut theme_selection,
                    ThemeApp::Clair,
                    if anglais { "Light" } else { "Clair" },
                )
                .clicked()
            {
                self.donnees.parametres.theme = theme_selection;
            }
            ui.add_space(8.0);
            if ui
                .selectable_value(
                    &mut theme_selection,
                    ThemeApp::Sombre,
                    if anglais { "Dark" } else { "Sombre" },
                )
                .clicked()
            {
                self.donnees.parametres.theme = theme_selection;
            }
        });

        ui.add_space(28.0);
        theme::separateur(ui);
        ui.add_space(18.0);

        ui.label(
            RichText::new(if anglais { "Language" } else { "Langue" })
                .size(16.0)
                .color(theme::palette().primaire)
                .strong(),
        );
        ui.add_space(10.0);
        let mut langue = self.donnees.parametres.langue;
        ui.horizontal(|ui| {
            if ui
                .selectable_value(&mut langue, LangueApp::Francais, "Français")
                .clicked()
            {
                self.donnees.parametres.langue = langue;
            }
            ui.add_space(8.0);
            if ui
                .selectable_value(&mut langue, LangueApp::English, "English")
                .clicked()
            {
                self.donnees.parametres.langue = langue;
            }
        });

        ui.add_space(24.0);
        ui.label(
            RichText::new(if anglais {
                "Your preferences are saved with your hotel data."
            } else {
                "Vos préférences sont enregistrées avec les données de l'hôtel."
            })
            .size(11.5)
            .color(theme::palette().texte_secondaire),
        );

        if parametres_avant
            != (
                self.donnees.parametres.theme,
                self.donnees.parametres.langue,
                self.donnees.parametres.nom_application.clone(),
                self.donnees.parametres.logo_personnalise.clone(),
            )
        {
            theme::configurer_langue(self.donnees.parametres.langue == LangueApp::English);
            self.donnees_modifiees = true;
        }
    }
}

fn bloc_texte_centre(
    ui: &mut egui::Ui,
    label: &str,
    texte: &str,
    taille: f32,
    couleur: Color32,
) {
    ui.label(
        RichText::new(label)
            .size(11.0)
            .color(theme::palette().texte_secondaire)
            .strong(),
    );
    ui.add_space(4.0);
    ui.label(RichText::new(texte).size(taille).color(couleur).strong());
}

fn nom_jour(j: chrono::Weekday, anglais: bool) -> &'static str {
    match (j, anglais) {
        (chrono::Weekday::Mon, false) => "lundi",
        (chrono::Weekday::Tue, false) => "mardi",
        (chrono::Weekday::Wed, false) => "mercredi",
        (chrono::Weekday::Thu, false) => "jeudi",
        (chrono::Weekday::Fri, false) => "vendredi",
        (chrono::Weekday::Sat, false) => "samedi",
        (chrono::Weekday::Sun, false) => "dimanche",
        (chrono::Weekday::Mon, true) => "Monday",
        (chrono::Weekday::Tue, true) => "Tuesday",
        (chrono::Weekday::Wed, true) => "Wednesday",
        (chrono::Weekday::Thu, true) => "Thursday",
        (chrono::Weekday::Fri, true) => "Friday",
        (chrono::Weekday::Sat, true) => "Saturday",
        (chrono::Weekday::Sun, true) => "Sunday",
    }
}

fn nom_mois(m: u32, anglais: bool) -> &'static str {
    if anglais {
        match m {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            _ => "December",
        }
    } else {
        match m {
            1 => "janvier",
            2 => "février",
            3 => "mars",
            4 => "avril",
            5 => "mai",
            6 => "juin",
            7 => "juillet",
            8 => "août",
            9 => "septembre",
            10 => "octobre",
            11 => "novembre",
            _ => "décembre",
        }
    }
}

fn copier_logo_personnalise(source: &std::path::Path) -> Result<String, String> {
    let dossier = match std::env::var("APPDATA") {
        Ok(v) if !v.is_empty() => std::path::PathBuf::from(v).join("JHotel").join("images"),
        _ => std::env::temp_dir().join("JHotel").join("images"),
    };
    std::fs::create_dir_all(&dossier)
        .map_err(|e| format!("Impossible de créer le dossier images : {}", e))?;
    let extension = source
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png")
        .to_lowercase();
    let cible = dossier.join(format!("logo_custom.{}", extension));
    if cible.exists() {
        let _ = std::fs::remove_file(&cible);
    }
    std::fs::copy(source, &cible)
        .map_err(|e| format!("Copie du logo impossible : {}", e))?;
    Ok(cible.to_string_lossy().to_string())
}

impl eframe::App for JHotelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.theme_applique != self.donnees.parametres.theme {
            self.theme_applique = self.donnees.parametres.theme;
            theme::configurer_style(ctx, self.theme_applique == ThemeApp::Sombre);
        }
        self.barre_haut(ctx);
        self.barre_bas(ctx);
        self.contenu(ctx);
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
        if self.donnees_modifiees {
            match persistance::sauvegarder(&self.donnees) {
                Ok(()) => self.donnees_modifiees = false,
                Err(erreur) => self.notifier(erreur, theme::palette().danger),
            }
        }
    }
}