pub fn ouvrir_mailto(destinataire: &str, sujet: &str, corps: &str) {
    let sujet_enc = encoder(sujet);
    let corps_enc = encoder(corps);
    let url = format!(
        "mailto:{}?subject={}&body={}",
        destinataire, sujet_enc, corps_enc
    );
    ouvrir_url(&url);
}

fn encoder(s: &str) -> String {
    let mut out = String::new();
    for octet in s.bytes() {
        match octet {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(octet as char);
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{:02X}", octet)),
        }
    }
    out
}

fn ouvrir_url(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn();
    }
}
