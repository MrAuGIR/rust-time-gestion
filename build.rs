
extern crate winres;


fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico")
        .set("FileDescription", "Application de gestion du temps de travail")
        .set("ProductName", "Gestion Temps")
        .set("CompanyName", "Votre Entreprise")
        .set("LegalCopyright", "Copyright 2024")
        .set("OriginalFilename", "gestion_temps.exe")
        .set("InternalName", "gestion_temps");
    }
}