use eframe::egui::{
    self, Color32, FontId, Rounding, Stroke, TextureFilter, TextureOptions, TextureWrapMode,
};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

static MODE_SOMBRE: AtomicBool = AtomicBool::new(false);

static LANGUE_ANGLAIS: AtomicBool = AtomicBool::new(false);

pub fn configurer_langue(anglais: bool) {
    LANGUE_ANGLAIS.store(anglais, Ordering::Relaxed);
}

pub fn anglais() -> bool {
    LANGUE_ANGLAIS.load(Ordering::Relaxed)
}

pub fn est_sombre() -> bool {
    MODE_SOMBRE.load(Ordering::Relaxed)
}

pub fn tr(francais: &'static str) -> &'static str {
    if !LANGUE_ANGLAIS.load(Ordering::Relaxed) {
        return francais;
    }
    match francais {
        "Nouvelle chambre" => "New room",
        "Numéro" => "Number",
        "Désignation" => "Description",
        "Type" => "Type",
        "Prix" => "Price",
        "Ajouter" => "Add",
        "Modifier" => "Edit",
        "Supprimer" => "Delete",
        "Liste des chambres" => "Room list",
        "Aucune chambre enregistrée" => "No rooms registered",
        "Réservations" => "Bookings",
        "Séjours" => "Stays",
        "Chambre" => "Room",
        "Choisir" => "Choose",
        "Téléphone" => "Phone",
        "Nouvelle réservation" => "New booking",
        "Réservations en cours" => "Current bookings",
        "Aucune réservation" => "No bookings",
        "Annuler" => "Cancel",
        "Réservation annulée" => "Booking cancelled",
        "Nouveau séjour" => "New stay",
        "Enregistrer le séjour" => "Save stay",
        "Séjours enregistrés" => "Saved stays",
        "Aucun séjour" => "No stays",
        "Suppr." => "Delete",
        "Reçu" => "Receipt",
        "Séjour supprimé" => "Stay deleted",
        "Champs obligatoires manquants" => "Required fields are missing",
        "Chambre non disponible pour ces dates" => "Room unavailable for these dates",
        "Chambre non disponible" => "Room unavailable",
        "Séjour enregistré" => "Stay saved",
        "Transformer une réservation en occupation" => "Convert a booking into an occupancy",
        "Réservation" => "Booking",
        "Occuper" => "Check in",
        "Occupation enregistrée" => "Occupancy saved",
        "Supprimer la sélection" => "Delete selected",
        "Occupation supprimée" => "Occupancy deleted",
        "Occupations en cours" => "Current occupancies",
        "Aucune occupation en cours" => "No current occupancy",
        "Date d'entrée" => "Check-in date",
        "Disponibilités" => "Availability",
        "Toutes" => "All",
        "Aucun résultat" => "No results",
        "LIBRE" => "AVAILABLE",
        "OCCUPEE" => "OCCUPIED",
        "RESERVEE" => "RESERVED",
        "Nombre de jours" => "Number of days",
        "Nom du client" => "Guest name",
        "Email" => "Email",
        "Réserver et envoyer email" => "Book and send email",
        "OCCUPATION(S)" => "OCCUPANCY(IES)",
        "RÉSERVATION(S)" => "BOOKING(S)",
        "CHAMBRE(S)" => "ROOM(S)",
        "SÉJOUR(S)" => "STAY(S)",
        "EN COURS" => "IN PROGRESS",
        "Reçu généré" => "Receipt generated",
        "Réservation enregistrée, email préparé" => "Booking saved, email prepared",
        "RÉSERVÉ" => "RESERVED",
        "TERMINÉ" => "COMPLETED",
        "Solde actuel" => "Current balance",
        "Gestion hoteliere" => "Hotel management",
        "Paramètres" => "Settings",
        "Apparence" => "Appearance",
        "Langue" => "Language",
        "Thème de l'application" => "Application theme",
        "Clair" => "Light",
        "Sombre" => "Dark",
        "Identité" => "Identity",
        "Nom de l'application" => "Application name",
        "Logo personnalisé" => "Custom logo",
        "Choisir une image" => "Choose an image",
        "Réinitialiser" => "Reset",
        "Aucun logo personnalisé" => "No custom logo",
        "Vos préférences sont enregistrées avec les données de l'hôtel." => {
            "Your preferences are saved with your hotel data."
        }
        "Personnalisez l'apparence et la langue de JHotel." => {
            "Customize the appearance and language of JHotel."
        }
        "Aperçu" => "Preview",
        "Jour" => "Day",
        "Mois" => "Month",
        "Année" => "Year",
        "Fichier image" => "Image file",
        "Standard" => "Standard",
        "Luxe" => "Luxury",
        "Economique" => "Economical",
        "DATE" => "DATE",
        "HEURE" => "TIME",
        _ => francais,
    }
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub primaire: Color32,
    pub accent: Color32,
    pub topbar_ciel: Color32,
    pub logo_contour: Color32,
    pub fond: Color32,
    pub fond_carte: Color32,
    pub fond_bilan: Color32,
    pub bordure: Color32,
    pub texte: Color32,
    pub texte_secondaire: Color32,
    pub succes: Color32,
    pub danger: Color32,
    pub warning: Color32,
}

pub fn palette() -> Palette {
    if MODE_SOMBRE.load(Ordering::Relaxed) {
        Palette {
            primaire: Color32::from_rgb(145, 180, 235),
            accent: Color32::from_rgb(75, 155, 225),
            topbar_ciel: Color32::from_rgb(28, 43, 64),
            logo_contour: Color32::from_rgb(95, 110, 130),
            fond: Color32::from_rgb(24, 28, 36),
            fond_carte: Color32::from_rgb(34, 40, 50),
            fond_bilan: Color32::from_rgb(39, 51, 68),
            bordure: Color32::from_rgb(65, 74, 88),
            texte: Color32::from_rgb(230, 234, 241),
            texte_secondaire: Color32::from_rgb(165, 175, 190),
            succes: Color32::from_rgb(80, 190, 125),
            danger: Color32::from_rgb(235, 100, 100),
            warning: Color32::from_rgb(230, 175, 75),
        }
    } else {
        Palette {
            primaire: Color32::from_rgb(21, 43, 84),
            accent: Color32::from_rgb(0, 122, 204),
            topbar_ciel: Color32::from_rgb(180, 220, 245),
            logo_contour: Color32::BLACK,
            fond: Color32::from_rgb(244, 246, 249),
            fond_carte: Color32::WHITE,
            fond_bilan: Color32::from_rgb(235, 242, 250),
            bordure: Color32::from_rgb(222, 226, 232),
            texte: Color32::from_rgb(30, 35, 45),
            texte_secondaire: Color32::from_rgb(110, 118, 130),
            succes: Color32::from_rgb(34, 148, 84),
            danger: Color32::from_rgb(200, 55, 55),
            warning: Color32::from_rgb(220, 145, 20),
        }
    }
}

pub fn r_sm() -> Rounding {
    Rounding::same(6.0)
}
pub fn r_md() -> Rounding {
    Rounding::same(10.0)
}
pub fn r_lg() -> Rounding {
    Rounding::same(14.0)
}

pub fn configurer_style(ctx: &egui::Context, sombre: bool) {
    MODE_SOMBRE.store(sombre, Ordering::Relaxed);
    let p = palette();
    let mut visuals = if sombre {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    visuals.panel_fill = p.fond;
    visuals.window_fill = p.fond_carte;
    visuals.extreme_bg_color = p.fond_carte;
    visuals.faint_bg_color = p.fond;
    visuals.widgets.noninteractive.bg_fill = p.fond_carte;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, p.bordure);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, p.texte);
    visuals.widgets.noninteractive.rounding = r_md();
    visuals.widgets.inactive.bg_fill = p.fond_carte;
    visuals.widgets.inactive.weak_bg_fill = p.fond;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, p.bordure);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, p.texte);
    visuals.widgets.inactive.rounding = r_md();
    visuals.widgets.hovered.bg_fill = p.fond_bilan;
    visuals.widgets.hovered.weak_bg_fill = p.fond_bilan;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, p.accent);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, p.texte);
    visuals.widgets.hovered.rounding = r_md();
    visuals.widgets.active.bg_fill = p.fond_carte;
    visuals.widgets.active.weak_bg_fill = p.fond_carte;
    visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, p.accent);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, p.texte);
    visuals.widgets.active.rounding = r_md();
    visuals.selection.bg_fill = p.fond_bilan;
    visuals.selection.stroke = Stroke::new(1.0_f32, p.accent);
    visuals.window_rounding = r_lg();
    visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 16.0,
        spread: 0.0,
        color: Color32::from_black_alpha(if sombre { 100 } else { 30 }),
    };
    visuals.popup_shadow = visuals.window_shadow;
    ctx.set_visuals(visuals);
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(14.0, 10.0);
    style.spacing.interact_size = egui::vec2(38.0, 34.0);
    style.spacing.window_margin = egui::Margin::same(18.0);
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::proportional(22.0)),
        (egui::TextStyle::Body, FontId::proportional(13.5)),
        (egui::TextStyle::Button, FontId::proportional(13.5)),
        (egui::TextStyle::Small, FontId::proportional(11.5)),
        (egui::TextStyle::Monospace, FontId::monospace(12.5)),
    ]
    .into();
    ctx.set_style(style);
}

static IMAGES: Mutex<Option<HashMap<String, egui::TextureHandle>>> = Mutex::new(None);

fn chemins_possibles(nom: &str) -> Vec<String> {
    if std::path::Path::new(nom).is_absolute() {
        return vec![nom.to_string()];
    }
    let mut chemins = vec![format!("images/{}", nom)];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            chemins.push(dir.join("images").join(nom).to_string_lossy().to_string());
        }
    }
    chemins.push(format!("{}/images/{}", env!("CARGO_MANIFEST_DIR"), nom));
    chemins
}

fn options_texture() -> TextureOptions {
    TextureOptions {
        magnification: TextureFilter::Linear,
        minification: TextureFilter::Linear,
        wrap_mode: TextureWrapMode::ClampToEdge,
    }
}

fn cle_cache(nom: &str, w_phys: u32, h_phys: u32, inverse: bool) -> String {
    if inverse {
        format!("{}|{}x{}|inv", nom, w_phys, h_phys)
    } else {
        format!("{}|{}x{}", nom, w_phys, h_phys)
    }
}

fn charger_source(nom: &str) -> Option<image::RgbaImage> {
    for chemin in chemins_possibles(nom) {
        if let Ok(bytes) = std::fs::read(&chemin) {
            if let Ok(img) = image::load_from_memory(&bytes) {
                return Some(img.to_rgba8());
            }
        }
    }
    None
}

fn pixels_physiques(ctx: &egui::Context, points: u32) -> u32 {
    let ppp = ctx.pixels_per_point();
    let facteur = if ppp > 0.0 { ppp } else { 1.0 };
    ((points as f32) * facteur).round().max(1.0) as u32
}

pub fn image_dimensionnee(
    ctx: &egui::Context,
    nom: &str,
    largeur_points: u32,
    hauteur_points: u32,
) -> Option<egui::TextureHandle> {
    image_dimensionnee_avec_inversion(ctx, nom, largeur_points, hauteur_points, false)
}

pub fn image_dimensionnee_avec_inversion(
    ctx: &egui::Context,
    nom: &str,
    largeur_points: u32,
    hauteur_points: u32,
    inverser: bool,
) -> Option<egui::TextureHandle> {
    let largeur_phys = pixels_physiques(ctx, largeur_points);
    let hauteur_phys = pixels_physiques(ctx, hauteur_points);
    let cle = cle_cache(nom, largeur_phys, hauteur_phys, inverser);
    {
        let cache = IMAGES.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(map) = cache.as_ref() {
            if let Some(tex) = map.get(&cle) {
                return Some(tex.clone());
            }
        }
    }
    let src = charger_source(nom)?;
    let redim = image::imageops::resize(
        &src,
        largeur_phys,
        hauteur_phys,
        image::imageops::FilterType::Lanczos3,
    );
    let pixels: Vec<u8> = if inverser {
        let mut v = redim.into_raw();
        let mut i = 0;
        while i + 3 < v.len() {
            v[i] = 255 - v[i];
            v[i + 1] = 255 - v[i + 1];
            v[i + 2] = 255 - v[i + 2];
            i += 4;
        }
        v
    } else {
        redim.into_raw()
    };
    let taille = [largeur_phys as usize, hauteur_phys as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(taille, &pixels);
    let texture = ctx.load_texture(cle.clone(), color_image, options_texture());
    let mut cache = IMAGES.lock().unwrap_or_else(|e| e.into_inner());
    let map = cache.get_or_insert_with(HashMap::new);
    map.insert(cle, texture.clone());
    Some(texture)
}

pub fn vider_cache() {
    let mut cache = IMAGES.lock().unwrap_or_else(|e| e.into_inner());
    *cache = None;
}

pub fn logo_cadre(
    ui: &mut egui::Ui,
    nom: &str,
    taille: f32,
    epaisseur_contour: f32,
    couleur_contour: Color32,
) -> egui::Response {
    let ctx = ui.ctx().clone();
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(taille, taille), egui::Sense::hover());
    let painter = ui.painter();

    let cote = taille.round() as u32;
    if let Some(tex) = image_dimensionnee(&ctx, nom, cote, cote) {
        let img_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(taille, taille));
        painter.image(
            tex.id(),
            img_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    } else {
        painter.rect_filled(rect, r_md(), palette().bordure);
    }

    painter.rect_stroke(
        rect,
        r_md(),
        Stroke::new(epaisseur_contour, couleur_contour),
    );
    resp
}

pub fn bouton_secondaire(ui: &mut egui::Ui, texte: &str, largeur: f32) -> bool {
    let b = egui::Button::new(egui::RichText::new(texte).color(palette().texte).size(13.0))
        .fill(palette().fond_carte)
        .stroke(Stroke::new(1.0_f32, palette().bordure))
        .rounding(r_md())
        .min_size(egui::vec2(largeur, 40.0));
    ui.add(b).clicked()
}

pub fn bouton_primaire(ui: &mut egui::Ui, texte: &str, largeur: f32) -> bool {
    let b = egui::Button::new(
        egui::RichText::new(texte)
            .color(Color32::WHITE)
            .size(13.5)
            .strong(),
    )
    .fill(palette().accent)
    .stroke(Stroke::NONE)
    .rounding(r_md())
    .min_size(egui::vec2(largeur, 42.0));
    ui.add(b).clicked()
}

pub fn bouton_couleur(
    ui: &mut egui::Ui,
    texte: &str,
    largeur: f32,
    hauteur: f32,
    couleur: Color32,
) -> bool {
    let b = egui::Button::new(
        egui::RichText::new(texte)
            .color(Color32::WHITE)
            .size(13.0)
            .strong(),
    )
    .fill(couleur)
    .stroke(Stroke::NONE)
    .rounding(r_md())
    .min_size(egui::vec2(largeur, hauteur));
    ui.add(b).clicked()
}

pub fn bouton_avec_icone(
    ui: &mut egui::Ui,
    fichier_icone: &str,
    texte: &str,
    largeur: f32,
    hauteur: f32,
    couleur: Color32,
) -> bool {
    let ctx = ui.ctx().clone();
    let galerie = ui.fonts(|f| {
        f.layout_no_wrap(
            texte.to_string(),
            FontId::proportional(13.5),
            Color32::WHITE,
        )
    });
    let largeur_texte = galerie.size().x;

    let cote_icone_pts = 18_u32;
    let has_icon =
        image_dimensionnee(&ctx, fichier_icone, cote_icone_pts, cote_icone_pts).is_some();
    let largeur_contenu = if has_icon {
        cote_icone_pts as f32 + 8.0 + largeur_texte
    } else {
        largeur_texte
    };
    let largeur_btn = largeur.max(largeur_contenu + 40.0);

    let (rect, resp) =
        ui.allocate_exact_size(egui::vec2(largeur_btn, hauteur), egui::Sense::click());
    let fond = if resp.hovered() {
        couleur.linear_multiply(1.15)
    } else {
        couleur
    };
    let painter = ui.painter();
    painter.rect_filled(rect, r_md(), fond);

    let mut x = rect.center().x - largeur_contenu / 2.0;
    if let Some(tex) = image_dimensionnee(&ctx, fichier_icone, cote_icone_pts, cote_icone_pts) {
        let icon_rect = egui::Rect::from_min_size(
            egui::pos2(x, rect.center().y - cote_icone_pts as f32 / 2.0),
            egui::vec2(cote_icone_pts as f32, cote_icone_pts as f32),
        );
        painter.image(
            tex.id(),
            icon_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        x += cote_icone_pts as f32 + 8.0;
    }
    painter.text(
        egui::pos2(x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        texte,
        FontId::proportional(13.5),
        Color32::WHITE,
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}

pub fn bouton_desactive_couleur(
    ui: &mut egui::Ui,
    texte: &str,
    largeur: f32,
    hauteur: f32,
) {
    let b = egui::Button::new(
        egui::RichText::new(texte)
            .color(palette().texte_secondaire)
            .size(13.0),
    )
    .fill(palette().fond)
    .stroke(Stroke::new(1.0_f32, palette().bordure))
    .rounding(r_md())
    .min_size(egui::vec2(largeur, hauteur));
    ui.add_enabled(false, b);
}

pub fn badge(ui: &mut egui::Ui, texte: &str, fond: Color32, couleur: Color32) {
    egui::Frame::none()
        .fill(fond)
        .rounding(r_sm())
        .inner_margin(egui::Margin::symmetric(10.0, 4.0))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(texte)
                    .color(couleur)
                    .size(11.5)
                    .strong(),
            );
        });
}

pub fn separateur(ui: &mut egui::Ui) {
    let largeur = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(largeur, 1.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, palette().bordure);
}

pub fn bouton_nav(ui: &mut egui::Ui, fichier_image: &str, texte: &str, actif: bool) -> bool {
    let ctx = ui.ctx().clone();
    let galerie = ui.fonts(|f| {
        f.layout_no_wrap(
            texte.to_string(),
            FontId::proportional(12.5),
            palette().texte,
        )
    });
    let largeur_texte = galerie.size().x;
    let largeur = 20.0 + 6.0 + largeur_texte + 20.0;

    let (rect, resp) = ui.allocate_exact_size(egui::vec2(largeur, 40.0), egui::Sense::click());

    let fond = if resp.hovered() {
        palette().accent
    } else if actif {
        palette().accent
    } else {
        palette().fond_bilan
    };

    let couleur_texte = if actif || resp.hovered() {
        Color32::WHITE
    } else {
        palette().primaire
    };

    let painter = ui.painter();
    painter.rect_filled(rect, r_md(), fond);

    let mut x = rect.left() + 10.0;
    let cote_icone = 18_u32;
    let inverser = est_sombre();
    if let Some(tex) =
        image_dimensionnee_avec_inversion(&ctx, fichier_image, cote_icone, cote_icone, inverser)
    {
        let icon_rect = egui::Rect::from_min_size(
            egui::pos2(x, rect.center().y - cote_icone as f32 / 2.0),
            egui::vec2(cote_icone as f32, cote_icone as f32),
        );
        painter.image(
            tex.id(),
            icon_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        x += cote_icone as f32 + 6.0;
    }

    painter.text(
        egui::pos2(x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        texte,
        FontId::proportional(12.5),
        couleur_texte,
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp.clicked()
}

pub fn badge_statut(libelle: &str) -> (Color32, Color32) {
    match libelle {
        "LIBRE" => (palette().succes, palette().fond_carte),
        "OCCUPEE" => (palette().warning, palette().fond_carte),
        "RESERVEE" => (Color32::from_rgb(155, 89, 182), palette().fond_carte),
        _ => (palette().fond, palette().texte),
    }
}

pub fn charger_icone_fenetre(nom: &str) -> Option<egui::IconData> {
    for chemin in chemins_possibles(nom) {
        if let Ok(bytes) = std::fs::read(&chemin) {
            if let Ok(img) = image::load_from_memory(&bytes) {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                let redim = if w > 256 || h > 256 {
                    let facteur = (256.0 / w.max(h) as f32).min(1.0);
                    let nw = (w as f32 * facteur) as u32;
                    let nh = (h as f32 * facteur) as u32;
                    image::imageops::resize(&rgba, nw, nh, image::imageops::FilterType::Lanczos3)
                } else {
                    rgba
                };
                let (nw, nh) = redim.dimensions();
                return Some(egui::IconData {
                    rgba: redim.into_raw(),
                    width: nw,
                    height: nh,
                });
            }
        }
    }
    None
}