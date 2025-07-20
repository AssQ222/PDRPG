# Architecture Documentation - PDRPG

## 📋 Project Overview
PDRPG (Personal Development RPG) - desktopowa aplikacja do samorozwoju z elementami grywalizacji RPG.

**Tech Stack:** Tauri + Rust + Svelte + TypeScript + SQLite

---

## 🏗️ Uwagi Techniczne z Implementacji

### Krok 1: Inicjalizacja Projektu (20.07.2025)

#### Środowisko deweloperskie:
- **OS:** Windows 10.0.26100 (PowerShell)
- **Rust:** 1.88.0 (6b00bc388 2025-06-23)
- **Cargo:** 1.88.0 (873a06493 2025-05-10)
- **Node.js:** v20.19.1
- **pnpm:** v10.13.1
- **create-tauri-app:** v4.6.0

#### Struktura katalogów (zaimplementowana):
```
src/
├── lib/
│   ├── components/         # Komponenty Svelte
│   ├── stores/            # Zarządzanie stanem Svelte
│   ├── types/             # Definicje typów TypeScript
│   ├── utils/             # Funkcje pomocnicze
│   └── api/               # Klient API
├── routes/                # Routing SvelteKit
├── app.html              # Szablon HTML
└── app.css               # Style globalne

src-tauri/
├── src/
│   ├── models/           # Modele danych Rust
│   ├── services/         # Logika biznesowa
│   ├── api/              # Endpointy API
│   ├── database/         # Operacje bazodanowe
│   ├── main.rs           # Entry point
│   └── lib.rs            # Biblioteka główna
├── migrations/           # Migracje SQL
└── Cargo.toml           # Zależności Rust
```

#### Rozwiązane problemy techniczne:

**1. Problem z tworzeniem projektu:**
- Komenda `cargo create-tauri-app` nieprawidłowo sparsowała argumenty
- Utworzono katalog `--name` zamiast `pdrpg`
- Rozwiązanie: Ręczne przeniesienie plików i korekta nazw

**2. Błędne nazwy pakietów:**
- **Problem:** Wszystkie pliki konfiguracyjne miały nazwę `"--name"` 
- **Pliki do korekty:** `Cargo.toml`, `package.json`, `tauri.conf.json`
- **Rozwiązanie:** Zmiana nazw na `"pdrpg"` i dostosowanie identyfikatorów

**Szczegóły zmian:**
```toml
# Cargo.toml
name = "pdrpg"           # było: "--name"
name = "pdrpg_lib"       # było: "__name_lib"
```

```json
// package.json
"name": "pdrpg"          // było: "--name"

// tauri.conf.json  
"productName": "pdrpg"   // było: "--name"
"identifier": "com.macie.pdrpg"  // było: "com.macie.--name"
"title": "PDRPG"         // było: "--name"
```

#### Konfiguracja aplikacji:
- **Port deweloperski:** 1420 (http://localhost:1420)
- **Frontend framework:** Svelte 5.36.10 + SvelteKit 2.25.1
- **Backend framework:** Tauri 2.6.0
- **Bundler:** Vite 6.3.5
- **Package manager:** pnpm 10.13.1

---

### Krok 2: Konfiguracja Bazy Danych i Pierwsze Modele (20.07.2025)

#### Zależności Rust (Cargo.toml):
```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.32", features = ["bundled"] }
rusqlite_migration = "1.2"
anyhow = "1.0"
tokio = { version = "1", features = ["full"] }
dirs = "5.0"
chrono = { version = "0.4", features = ["serde"] }
```

#### Architektura bazy danych:
- **Engine:** SQLite (embedded, bundled z rusqlite)
- **Lokalizacja:** `%APPDATA%\pdrpg\pdrpg.db` (Windows)
- **Migracje:** Automatyczne z `rusqlite_migration`
- **Connection pooling:** Single connection z Mutex dla thread-safety

#### Struktura Database Layer:
```rust
// src-tauri/src/database/mod.rs
pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self>
    fn run_migrations(&mut self) -> Result<()>
    pub fn connection(&self) -> &Connection
}

pub fn initialize_database() -> Result<Database>
```

#### Model Task:
```rust
// src-tauri/src/models/mod.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: i64,    // Unix timestamp
    pub updated_at: i64,    // Unix timestamp
}
```

#### Pierwsza migracja (0001_create_tasks.sql):
```sql
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_tasks_updated_at ON tasks(updated_at);
```

#### Application State:
```rust
// src-tauri/src/lib.rs
struct AppState {
    db: Mutex<Database>,
}

pub fn run() {
    let db = database::initialize_database()
        .expect("Failed to initialize database");
    
    let app_state = AppState {
        db: Mutex::new(db),
    };

    tauri::Builder::default()
        .manage(app_state)
        // ...
}
```

#### Rozwiązane problemy techniczne:

**1. Błędna ścieżka do migracji:**
- **Problem:** `include_str!("../../../migrations/...")` 
- **Rozwiązanie:** `include_str!("../../migrations/...")`

**2. Mutable reference dla migracji:**
- **Problem:** `migrations.to_latest(&self.connection)` wymagał `&mut`
- **Rozwiązanie:** Zmiana `fn run_migrations(&self)` na `fn run_migrations(&mut self)`

**3. Cargo nie w PATH:**
- **Problem:** Windows PowerShell nie miał dostępu do `cargo`
- **Rozwiązanie:** `$env:PATH += ";$env:USERPROFILE\.cargo\bin"`

#### Status bazy danych:
- ✅ **Plik:** `%APPDATA%\pdrpg\pdrpg.db` (24KB)
- ✅ **Tabela:** `tasks` z indeksami
- ✅ **Migracje:** Automatyczne przy starcie
- ✅ **Connection:** Thread-safe z Mutex
- ✅ **Kompilacja:** Bez błędów (tylko warnings o nieużywanych funkcjach)

---

## 📝 Notatki Architektoniczne

### Zasady projektowania:
1. **Małe, skoncentrowane pliki** - każdy plik ma jedną odpowiedzialność
2. **Separacja warstw** - wyraźne rozdzielenie Frontend/Backend
3. **Type safety** - TypeScript w całym projekcie + Rust
4. **Reactive state management** - Svelte stores (TODO: Krok 3)
5. **Database-first approach** - SQLite jako single source of truth

### Wzorce implementacji:
- **Database:** Connection pooling z Mutex dla thread-safety
- **Models:** Rich domain models z metodami biznesowymi
- **Migrations:** Automatyczne, wersjonowane z rusqlite_migration
- **Error handling:** Result<T, E> pattern z anyhow
- **Serialization:** Pełna obsługa Serde dla wszystkich struktur

### Następne kroki architektoniczne:
- [ ] **Krok 3:** Tauri Commands dla CRUD operations
- [ ] **Krok 3:** Svelte stores dla reaktywnego stanu
- [ ] **Krok 3:** Frontend components (TaskInput, TaskList)
- [ ] **Krok 4:** Habit model i drugi moduł
- [ ] **Krok 5:** Character model i system EXP

### Performance considerations:
- SQLite bundled - brak zewnętrznych zależności
- Indeksy na często używanych kolumnach
- Connection reuse z Mutex
- Lazy loading dla UI (planowane w Krok 3)
