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

#### Status:
- ✅ Projekt zainicjowany
- ✅ Struktura katalogów utworzona
- ✅ Aplikacja uruchamia się poprawnie
- ✅ Git repository skonfigurowane
- 🔄 Gotowe do implementacji bazy danych (Krok 2)

---

## 📝 Notatki Architektoniczne

### Zasady projektowania:
1. **Małe, skoncentrowane pliki** - każdy plik ma jedną odpowiedzialność
2. **Separacja warstw** - wyraźne rozdzielenie Frontend/Backend
3. **Type safety** - TypeScript w całym projekcie
4. **Reactive state management** - Svelte stores
5. **Database-first approach** - SQLite jako single source of truth

### Następne kroki architektoniczne:
- [ ] Konfiguracja SQLite i migracji
- [ ] Zdefiniowanie pierwszych modeli danych
- [ ] Implementacja Tauri commands
- [ ] Konfiguracja komunikacji Frontend-Backend
