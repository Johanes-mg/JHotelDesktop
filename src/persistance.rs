use crate::donnees::Donnees;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn dossier_app() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        if !appdata.is_empty() {
            return PathBuf::from(appdata).join("JHotel");
        }
    }
    std::env::temp_dir().join("JHotel")
}

fn chemin_donnees() -> PathBuf {
    dossier_app().join("donnees.json")
}

/// Charge les données. En cas de JSON invalide, le fichier original est conservé
/// sous forme de sauvegarde au lieu d'être écrasé silencieusement.
pub fn charger() -> (Donnees, Option<String>) {
    let chemin = chemin_donnees();
    match fs::read_to_string(&chemin) {
        Ok(contenu) => match serde_json::from_str(&contenu) {
            Ok(donnees) => (donnees, None),
            Err(erreur) => {
                let sauvegarde = chemin.with_extension("json.corrompu");
                let message = match fs::copy(&chemin, &sauvegarde) {
                    Ok(_) => format!(
                        "Le fichier de données est illisible ({}). Une copie a été conservée dans {}.",
                        erreur, sauvegarde.display()
                    ),
                    Err(e) => format!(
                        "Le fichier de données est illisible ({}). Impossible de créer une copie de sécurité : {}.",
                        erreur, e
                    ),
                };
                (Donnees::default(), Some(message))
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let backup = chemin.with_extension("json.bak");
            match fs::read_to_string(&backup) {
                Ok(contenu) => match serde_json::from_str(&contenu) {
                    Ok(donnees) => (
                        donnees,
                        Some(
                            "Les données ont été récupérées depuis la sauvegarde de sécurité."
                                .to_string(),
                        ),
                    ),
                    Err(erreur) => (
                        Donnees::default(),
                        Some(format!(
                            "Le fichier principal et sa sauvegarde sont illisibles : {}",
                            erreur
                        )),
                    ),
                },
                Err(_) => (Donnees::default(), None),
            }
        }
        Err(e) => (
            Donnees::default(),
            Some(format!("Impossible de lire les données : {}", e)),
        ),
    }
}

/// Écriture temporaire puis remplacement, afin de limiter les fichiers JSON tronqués.
pub fn sauvegarder(d: &Donnees) -> Result<(), String> {
    let dossier = dossier_app();
    fs::create_dir_all(&dossier)
        .map_err(|e| format!("Impossible de créer le dossier de données : {}", e))?;

    let chemin = chemin_donnees();
    let temporaire = chemin.with_extension("json.tmp");
    let contenu = serde_json::to_vec_pretty(d)
        .map_err(|e| format!("Impossible de préparer les données : {}", e))?;

    {
        let mut fichier = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporaire)
            .map_err(|e| format!("Impossible d'ouvrir le fichier temporaire : {}", e))?;
        fichier
            .write_all(&contenu)
            .and_then(|_| fichier.sync_all())
            .map_err(|e| format!("Impossible d'écrire les données : {}", e))?;
    }

    let backup = chemin.with_extension("json.bak");
    let avait_original = chemin.exists();
    if avait_original {
        let _ = fs::remove_file(&backup);
        fs::rename(&chemin, &backup).map_err(|e| {
            format!(
                "Impossible de sécuriser l'ancien fichier de données : {}",
                e
            )
        })?;
    }

    if let Err(erreur) = fs::rename(&temporaire, &chemin) {
        if avait_original {
            let _ = fs::rename(&backup, &chemin);
        }
        return Err(format!(
            "Impossible de remplacer le fichier de données : {}",
            erreur
        ));
    }

    if avait_original {
        let _ = fs::remove_file(backup);
    }
    Ok(())
}
