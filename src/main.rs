mod app;
mod donnees;
mod email;
mod modele;
mod persistance;
mod theme;
mod vue_chambres;
mod vue_clients;
mod vue_occupations;
mod vue_recherche;

use app::JHotelApp;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 780.0])
        .with_min_inner_size([880.0, 560.0])
        .with_resizable(true)
        .with_title("JHotel");

    if let Some(icone) = theme::charger_icone_fenetre("logo.png") {
        viewport = viewport.with_icon(Arc::new(icone));
    }

    let options = eframe::NativeOptions {
        viewport,
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "JHotel",
        options,
        Box::new(|cc| Ok(Box::new(JHotelApp::new(cc)))),
    )
}
