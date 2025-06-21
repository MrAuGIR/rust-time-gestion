
extern crate winres;


fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico")
    .set("FileDescription", "Application de gestion du temps de travail")
    .set("ProductName", "Gestion Temps")
    .set("CompanyName", "MrAuGIR")
    .set("LegalCopyright", "Copyright 2025")
    .set("OriginalFilename", "gestion_temps.exe")
    .set("InternalName", "gestion_temps");

    res.compile().expect("Failed to compile resources");
}