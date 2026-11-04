use crate::modele::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Donnees {
    pub chambres: Vec<Chambre>,
    pub reservations: Vec<Reservation>,
    pub sejours: Vec<Sejour>,
    pub occupations: Vec<Occupation>,
    pub prochain_id: u64,
    #[serde(default)]
    pub parametres: Parametres,
}

impl Donnees {
    pub fn nouvel_id(&mut self) -> u64 {
        self.prochain_id = self.prochain_id.saturating_add(1);
        self.prochain_id
    }

    pub fn chambre_existe(&self, numero: &str) -> bool {
        self.chambres.iter().any(|c| c.numero == numero)
    }

    pub fn trouver_chambre(&self, numero: &str) -> Option<&Chambre> {
        self.chambres.iter().find(|c| c.numero == numero)
    }

    pub fn chambre_occupee_a(&self, numero: &str, date: NaiveDate) -> bool {
        self.sejours
            .iter()
            .any(|s| s.numero_chambre == numero && s.couvre(date))
            || self
                .occupations
                .iter()
                .filter_map(|o| self.reservations.iter().find(|r| r.id == o.id_reservation))
                .any(|r| r.numero_chambre == numero && r.couvre(date))
    }

    pub fn chambre_reservee_a(&self, numero: &str, date: NaiveDate) -> bool {
        if self.chambre_occupee_a(numero, date) {
            return false;
        }
        self.reservations
            .iter()
            .any(|r| r.numero_chambre == numero && r.couvre(date))
    }

    pub fn statut_chambre(&self, numero: &str, date: NaiveDate) -> StatutChambre {
        if self.chambre_occupee_a(numero, date) {
            StatutChambre::Occupee
        } else if self.chambre_reservee_a(numero, date) {
            StatutChambre::Reservee
        } else {
            StatutChambre::Libre
        }
    }

    pub fn est_disponible(&self, numero: &str, date_entree: NaiveDate, jours: i64) -> bool {
        if jours <= 0 {
            return false;
        }
        for i in 0..jours {
            let d = date_entree + chrono::Duration::days(i);
            if self.chambre_occupee_a(numero, d) || self.chambre_reservee_a(numero, d) {
                return false;
            }
        }
        true
    }

    pub fn solde(&self) -> i64 {
        let today = aujourdhui();
        let mut total = 0i64;
        for s in &self.sejours {
            if let Some(c) = self.trouver_chambre(&s.numero_chambre) {
                if s.couvre(today) {
                    total += c.prix;
                }
            }
        }
        total
    }

    pub fn reservation_occupee(&self, id_reservation: u64) -> bool {
        self.occupations
            .iter()
            .any(|o| o.id_reservation == id_reservation)
    }

    pub fn details_occupant(
        &self,
        numero: &str,
        date: NaiveDate,
    ) -> Option<(String, NaiveDate, NaiveDate)> {
        for s in &self.sejours {
            if s.numero_chambre == numero && s.couvre(date) {
                return Some((
                    s.nom_client.clone(),
                    s.date_entree,
                    date_sortie(s.date_entree, s.nombre_jours),
                ));
            }
        }
        for r in &self.reservations {
            if r.numero_chambre == numero && r.couvre(date) {
                return Some((
                    r.nom_client.clone(),
                    r.date_entree,
                    date_sortie(r.date_entree, r.nombre_jours),
                ));
            }
        }
        None
    }
}
