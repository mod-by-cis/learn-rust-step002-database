// Magia Slinta - to makro wczytuje skompilowany plik .slint
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Tworzymy okno
    let ui = AppWindow::new()?;

    // Obsługa zdarzenia (kliknięcie przycisku)
    // clone_strong() jest potrzebne, żeby przekazać UI do środka callbacka (jeśli byśmy chcieli coś zmieniać w oknie)
    let _ui_handle = ui.as_weak();
    
    ui.on_szukaj_klik(move |tekst| {
        // Tutaj w przyszłości podepniesz: db.query(...)
        println!("🔍 Kliknięto szukaj! Użytkownik wpisał: {}", tekst);
    });

    println!("🚀 Uruchamiam GUI...");
    ui.run()
}