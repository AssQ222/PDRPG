use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use rusqlite_migration::{Migrations, M};
use std::path::PathBuf;

/// Struktura zarządzająca połączeniem z bazą danych
pub struct Database {
    connection: Connection,
}

impl Database {
    /// Inicjalizuje nowe połączenie z bazą danych SQLite
    ///
    /// # Arguments
    /// * `db_path` - Ścieżka do pliku bazy danych
    ///
    /// # Returns
    /// * `Result<Database>` - Nowa instancja Database lub błąd
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let connection = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        // Włączenie foreign keys
        connection.execute("PRAGMA foreign_keys = ON;", [])?;

        let mut db = Database { connection };

        // Uruchomienie migracji
        db.run_migrations()?;

        Ok(db)
    }

    /// Uruchamia migracje bazy danych
    fn run_migrations(&mut self) -> Result<()> {
        let migrations = Migrations::new(vec![
            M::up(include_str!("../../migrations/0001_create_tasks.sql")),
            M::up(include_str!("../../migrations/0002_create_habits.sql")),
        ]);

        migrations.to_latest(&mut self.connection)?;

        Ok(())
    }

    /// Zwraca referencję do połączenia z bazą danych
    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}

/// Inicjalizuje bazę danych w standardowej lokalizacji aplikacji
pub fn initialize_database() -> Result<Database> {
    let mut db_path = get_app_data_dir()?;
    db_path.push("pdrpg.db");

    // Upewnij się, że katalog istnieje
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    Database::new(db_path)
}

/// Zwraca ścieżkę do katalogu danych aplikacji
fn get_app_data_dir() -> Result<PathBuf> {
    let app_data_dir = if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    } else if cfg!(target_os = "macos") {
        dirs::home_dir()
            .map(|home| home.join("Library").join("Application Support"))
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        // Linux i inne systemy Unix
        dirs::home_dir()
            .map(|home| home.join(".local").join("share"))
            .unwrap_or_else(|| PathBuf::from("."))
    };

    Ok(app_data_dir.join("pdrpg"))
}
