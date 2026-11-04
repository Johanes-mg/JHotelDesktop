use chrono::{Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

pub const OFFSET_HEURES: i64 = 3;

pub fn aujourdhui() -> NaiveDate {
    let maintenant = Utc::now().naive_utc() + Duration::hours(OFFSET_HEURES);
    NaiveDate::from_ymd_opt(maintenant.year(), maintenant.month(), maintenant.day())
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
}

pub fn formater_date_fr(d: NaiveDate) -> String {
    format!("{:02}/{:02}/{:04}", d.day(), d.month(), d.year())
}

pub fn parser_date_fr(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    let parties: Vec<&str> = s.split('/').collect();
    if parties.len() != 3 {
        return None;
    }
    let j: u32 = parties[0].parse().ok()?;
    let m: u32 = parties[1].parse().ok()?;
    let a: i32 = parties[2].parse().ok()?;
    NaiveDate::from_ymd_opt(a, m, j)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Chambre {
    pub numero: String,
    pub designation: String,
    pub type_chambre: String,
    pub prix: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: u64,
    pub numero_chambre: String,
    pub date_entree: NaiveDate,
    pub nombre_jours: i64,
    pub nom_client: String,
    pub email: String,
    pub creee_le: NaiveDate,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sejour {
    pub id: u64,
    pub numero_chambre: String,
    pub date_entree: NaiveDate,
    pub nombre_jours: i64,
    pub nom_client: String,
    pub telephone: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Occupation {
    pub id: u64,
    pub id_reservation: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatutChambre {
    Libre,
    Occupee,
    Reservee,
}

impl StatutChambre {
    pub fn libelle(&self) -> &'static str {
        match self {
            StatutChambre::Libre => "LIBRE",
            StatutChambre::Occupee => "OCCUPEE",
            StatutChambre::Reservee => "RESERVEE",
        }
    }
}

pub fn date_sortie(date_entree: NaiveDate, jours: i64) -> NaiveDate {
    date_entree + Duration::days(jours)
}

pub fn periode_contient(date_entree: NaiveDate, jours: i64, date: NaiveDate) -> bool {
    let fin = date_sortie(date_entree, jours);
    date >= date_entree && date < fin
}

impl Reservation {
    pub fn couvre(&self, date: NaiveDate) -> bool {
        periode_contient(self.date_entree, self.nombre_jours, date)
    }
}

impl Sejour {
    pub fn couvre(&self, date: NaiveDate) -> bool {
        periode_contient(self.date_entree, self.nombre_jours, date)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeApp {
    Clair,
    Sombre,
}
impl Default for ThemeApp {
    fn default() -> Self {
        Self::Clair
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LangueApp {
    Francais,
    English,
}
impl Default for LangueApp {
    fn default() -> Self {
        Self::Francais
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Parametres {
    pub theme: ThemeApp,
    pub langue: LangueApp,
    pub nom_application: String,
    pub logo_personnalise: Option<String>,
}
impl Default for Parametres {
    fn default() -> Self {
        Self {
            theme: ThemeApp::Clair,
            langue: LangueApp::Francais,
            nom_application: "JHotel".to_string(),
            logo_personnalise: None,
        }
    }
}