use clap::{Parser, Subcommand};
use quick_xml::de::from_str;
use std::fs;
use std::path::PathBuf;
use colored::Colorize; 

// Import modelu i konektora
use my_libs::domain::antroponim::AntroponimXml;
use my_libs::db::connect::DatabaseConnection;

// ⚠️ UWAGA: Definiujemy ścieżki LOKALNIE dla tego mastykonu.
// Nie zaśmiecamy głównego consts.rs, bo inne mastykony będą miały inne foldery.
const MASTYKON_FOLDER: &str = "dict/antropomastykon"; // SurrealDB utworzy ten FOLDER
const MASTYKON_NS: &str = "mastykon_space";
const MASTYKON_DB: &str = "antroponim";

#[derive(Parser)]
#[command(name = "Antroponomastykon CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Verify { #[arg(short, long)] file: PathBuf },
    Status,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Verify { file } => {
            println!("{} {:?}", "🔍 Weryfikacja pliku:".cyan().bold(), file);
            // ... (logika odczytu pliku XML - bez zmian)
            let content = fs::read_to_string(file).unwrap_or_default();
            match from_str::<AntroponimXml>(&content) {
                Ok(m) => {
                     println!("{}", "✅ XML Poprawny!".green());
                     println!("   📊 Koncepty: {}", m.concepts.len());
                },
                Err(e) => eprintln!("❌ Błąd XML: {}", e),
            }
        },

        Commands::Status => {
            println!("{} {}", "🔌 Łączenie z folderem bazy:".cyan(), MASTYKON_FOLDER);

            // Tu używamy uniwersalnego inita z connect.rs
            match DatabaseConnection::init(MASTYKON_FOLDER, MASTYKON_NS, MASTYKON_DB).await {
                Ok(db) => {
                    println!("{}", "✅ Połączono!".green());
                    
                    // Sprawdzamy tabelę 'concept'
                    let count = db.get_count("concept").await.unwrap_or(0);
                    println!("📊 Liczba haseł (antroponimów): {}", count.to_string().yellow().bold());
                },
                Err(e) => {
                    eprintln!("{} {}", "❌ Błąd połączenia:".red().bold(), e);
                }
            }
        }
    }
}

// cargo run --bin mastykon_cli -- verify --file ./batch/test_antro.xml 