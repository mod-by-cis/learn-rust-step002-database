use std::io; // Importujemy bibliotekę wejścia/wyjścia

fn main() {
    println!("Hello, world!");
    println!("\nProgram zakonczony. Nacisnij Enter, aby zamknac to okno...");
    
    // Tworzymy pusty tekst, aby "złapać" wciśnięcie Enter
    let mut pauza = String::new();
    io::stdin()
        .read_line(&mut pauza)
        .expect("Nie udalo sie odczytac linii");
}