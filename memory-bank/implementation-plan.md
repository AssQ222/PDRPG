# Plan Implementacji: SelfDevelopmentApp

**Cel:** Stworzenie w pełni funkcjonalnej, desktopowej aplikacji do samorozwoju z elementami RPG, zgodnie z dostarczonymi dokumentami projektowymi i technicznymi.

---

### Krok 0: Przygotowanie Środowiska i Narzędzi
**Cel:** Zainstalowanie wszystkich niezbędnych narzędzi i zależności.

* **Zadanie:**
    1.  Zainstaluj **Rust** i **Cargo**.
    2.  Zainstaluj **Node.js** i **pnpm** (lub npm/yarn).
    3.  Zainstaluj **Tauri CLI**: `cargo install tauri-cli`.
    4.  Skonfiguruj edytor kodu (VS Code) z rekomendowanymi wtyczkami: `rust-analyzer`, `Svelte for VS Code`, `Tailwind CSS IntelliSense`.

* **Test Weryfikujący ✅:**
    * Uruchom w terminalu polecenia: `rustc --version`, `cargo --version`, `node --version`, `pnpm --version` i `cargo tauri --version`. Wszystkie muszą się wykonać i wyświetlić zainstalowane wersje bez błędów.

---

### Krok 1: Inicjalizacja Projektu i Struktura Katalogów
**Cel:** Stworzenie szkieletu projektu zgodnie z `cursorrules.md`.

* **Zadanie:**
    1.  Użyj Tauri CLI, aby zainicjować nowy projekt: `cargo create-tauri-app`.
    2.  Wybierz następujące opcje:
        * Nazwa aplikacji: `pdrpg`
        * Frontend: `Svelte`
        * TypeScript: `Tak`
    3.  Po inicjalizacji, ręcznie stwórz wymaganą strukturę katalogów zgodnie z plikiem `cursorrules.md`:
        * `src/lib/stores/`, `src/lib/types/`, `src/lib/utils/`, `src/lib/api/`
        * `src-tauri/src/models/`, `src-tauri/src/services/`, `src-tauri/src/api/`, `src-tauri/src/database/`
    4.  Zainicjuj repozytorium Git: `git init`, `git add .`, `git commit -m "feat: initial project setup"`.

* **Test Weryfikujący ✅:**
    * Uruchom aplikację w trybie deweloperskim: `cargo tauri dev`. Aplikacja musi się uruchomić i wyświetlić domyślny ekran startowy Svelte + Tauri.
    * Uruchom polecenie `tree` (lub `ls -R`), aby potwierdzić, że struktura katalogów jest identyczna z tą zdefiniowaną w `cursorrules.md`.

---

### Krok 2: Konfiguracja Bazy Danych i Pierwsze Modele
**Cel:** Przygotowanie warstwy persystencji danych.

* **Zadanie:**
    1.  **Backend (Rust):**
        * Dodaj `rusqlite` i `rusqlite_migration` do `src-tauri/Cargo.toml`. `rusqlite` jest rekomendowane dla SQLite w Ruście.
        * W `src-tauri/src/database/mod.rs` napisz logikę inicjalizacji połączenia z bazą danych SQLite (`pdrpg.db`).
        * W `src-tauri/src/models/mod.rs` zdefiniuj pierwszy model danych, np. `struct Task { id: i32, title: String, completed: bool }`.
        * Stwórz pierwszą migrację (`migrations/0001_create_tasks.sql`), która tworzy tabelę `tasks` z odpowiednimi kolumnami.
        * Zmodyfikuj `src-tauri/src/main.rs`, aby przy starcie aplikacji inicjalizować bazę danych i uruchamiać migracje.

* **Test Weryfikujący ✅:**
    * Uruchom aplikację (`cargo tauri dev`). Sprawdź, czy w katalogu `src-tauri` został utworzony plik `pdrpg.db`.
    * Użyj dowolnego narzędzia do przeglądania SQLite (np. `sqlite3` w terminalu), otwórz plik `pdrpg.db` i wykonaj polecenie `.schema tasks`. Wynik musi pokazać strukturę tabeli `tasks` zgodną z migracją.

---

### Krok 3: Implementacja Modułu Zadań (End-to-End)
**Cel:** Stworzenie pełnej funkcjonalności dla jednego modułu, aby zweryfikować architekturę. Moduł zadań jest kluczowy i opisany jako "Tablica zadań / Inbox Zero".

* **Zadanie:**
    1.  **Backend (Rust):**
        * W `src-tauri/src/services/task_service.rs` stwórz funkcje CRUD: `add_task(title: String)`, `get_all_tasks() -> Vec<Task>`, `toggle_task_status(id: i32)`, `delete_task(id: i32)`.
        * W `src-tauri/src/main.rs` zarejestruj te funkcje jako polecenia Tauri (commands) w `tauri::Builder`.
        * Napisz testy jednostkowe w Rust dla `task_service.rs`, mockując połączenie z bazą danych.
    2.  **Frontend (Svelte):**
        * W `src/lib/types/task.ts` zdefiniuj interfejs `Task`.
        * W `src/lib/stores/taskStore.ts` stwórz Svelte store, który będzie przechowywał listę zadań i komunikował się z backendem przez polecenia Tauri (`invoke`).
        * W `src/lib/components/` stwórz komponenty: `TaskInput.svelte` i `TaskList.svelte`.
        * Zastąp domyślny kod w `src/routes/+page.svelte` nowymi komponentami.

* **Test Weryfikujący ✅:**
    * **Test E2E:** Uruchom aplikację.
        1.  Wpisz "Moje pierwsze zadanie" w pole `TaskInput` i zatwierdź.
        2.  Zadanie musi pojawić się na liście w `TaskList`.
        3.  Kliknij na zadanie, aby oznaczyć je jako ukończone (np. zmiana stylu).
        4.  Zrestartuj całkowicie aplikację (`cargo tauri dev`).
        5.  Zadanie "Moje pierwsze zadanie" musi wciąż być widoczne na liście, zachowując swój stan (weryfikacja persystencji danych w SQLite).

---

### Krok 4: Implementacja Modułu Nawyków (Habit Tracker)
**Cel:** Dodanie kolejnego kluczowego modułu, powielając sprawdzony wzorzec. Moduł ma obsługiwać proste "tak/nie" lub liczniki.

* **Zadanie:**
    1.  **Backend (Rust):** Zdefiniuj model `Habit`, stwórz migrację, napisz `habit_service.rs` i zarejestruj polecenia Tauri dla CRUD nawyków.
    2.  **Frontend (Svelte):** Stwórz `habitStore.ts`, komponenty do dodawania i wyświetlania nawyków (np. `HabitTracker.svelte`), które pokazują dni tygodnia i pozwalają na odznaczanie wykonania.

* **Test Weryfikujący ✅:**
    * **Test E2E:** Uruchom aplikację.
        1.  Dodaj nowy nawyk, np. "Codzienna medytacja".
        2.  Nawyk musi pojawić się na liście.
        3.  Odznacz dzisiejszy dzień jako "wykonany".
        4.  Zrestartuj aplikację. Nawyk i jego dzisiejszy status muszą być poprawnie wczytane.

---

### Krok 5: Implementacja Rdzenia Grywalizacji (EXP i Poziomy)
**Cel:** Wprowadzenie podstawowych mechanik RPG powiązanych z istniejącymi modułami. Użytkownik zdobywa EXP za zadania i nawyki.

* **Zadanie:**
    1.  **Backend (Rust):**
        * Stwórz model `Character` w `models` i odpowiednią tabelę w bazie danych (poziom, EXP).
        * Stwórz `character_service.rs` do zarządzania stanem postaci.
        * Zmodyfikuj `task_service.rs` i `habit_service.rs`: ukończenie zadania lub nawyku musi wywoływać funkcję z `character_service`, która dodaje punkty EXP.
    2.  **Frontend (Svelte):**
        * Stwórz `characterStore.ts`.
        * W `src/lib/components/` dodaj `CharacterStatus.svelte`, który wyświetla awatar (na razie placeholder), poziom i pasek postępu EXP. Umieść go w głównym layoucie aplikacji.

* **Test Weryfikujący ✅:**
    * **Test E2E:** Uruchom aplikację.
        1.  Zanotuj początkowy stan paska EXP.
        2.  Ukończ jedno zadanie z listy.
        3.  Sprawdź, czy pasek postępu EXP w komponencie `CharacterStatus` wizualnie się zapełnił.
        4.  Zrestartuj aplikację. Nowy stan EXP musi być zachowany.

---

### Krok 6: Implementacja Dashboardu
**Cel:** Stworzenie głównego ekranu podsumowującego informacje ze wszystkich sekcji.

* **Zadanie:**
    1.  Stwórz nową stronę/komponent `Dashboard.svelte`.
    2.  Komponent ma subskrybować dane ze wszystkich istniejących store'ów: `taskStore`, `habitStore` i `characterStore`.
    3.  Zaimplementuj widżety opisane w dokumencie projektowym: podsumowanie zadań na dziś, aktualne streaki z nawyków, statystyki postaci.

* **Test Weryfikujący ✅:**
    * **Test E2E:**
        1.  Dodaj 3 zadania na dziś. Odznacz 2 nawyki.
        2.  Przejdź do Dashboardu.
        3.  Sprawdź, czy widżet zadań pokazuje poprawną liczbę zadań.
        4.  Sprawdź, czy widżet nawyków pokazuje poprawne streaki.
        5.  Sprawdź, czy widżet postaci pokazuje aktualny poziom i EXP.

---

### Krok 7: Implementacja Lokalnego API dla AI
**Cel:** Umożliwienie komunikacji z aplikacją zewnętrznym narzędziom, zgodnie z wymaganiami technicznymi i propozycją stacku.

* **Zadanie:**
    1.  **Backend (Rust):**
        * Dodaj do zależności `axum` lub `actix-web`.
        * W `src-tauri/src/api/mod.rs` stwórz logikę lekkiego serwera HTTP.
        * Uruchom serwer w osobnym wątku przy starcie aplikacji Tauri.
        * Stwórz pierwszy endpoint, np. `GET /api/tasks`, który zwraca listę zadań w formacie JSON, korzystając z `task_service`.

* **Test Weryfikujący ✅:**
    * **Test E2E:**
        1.  Uruchom aplikację.
        2.  W zewnętrznym terminalu lub narzędziu API (np. Postman) wykonaj zapytanie: `curl http://localhost:PORT/api/tasks` (PORT zdefiniowany w kodzie).
        3.  Odpowiedź musi być typu `application/json` i zawierać listę zadań, które aktualnie znajdują się w aplikacji.

---

### Kolejne Kroki (Skrócony Opis)
Po wykonaniu powyższych fundamentów, kolejne kroki polegają na iteracyjnym dodawaniu pozostałych funkcji w ten sam, testowalny sposób:

* **Krok 8: Klasy Postaci i Wykres Pajęczynowy:** Zaimplementuj logikę klas (Wojownik, Mag, etc.), powiąż zadania z atrybutami, użyj `Chart.js` lub podobnej biblioteki do wizualizacji.
    * **Test:** Ukończenie zadania oznaczonego tagiem "nauka" zwiększa atrybut "Intelekt" na wykresie pajęczynowym.
* **Krok 9: Questy Tygodniowe i Odznaki:** Stwórz algorytm generujący questy i system przyznawania odznak za streaki (30, 90, 150, 230 dni).
    * **Test:** Utrzymuj nawyk przez 30 dni. Sprawdź, czy odznaka "30-Day Streak" została odblokowana i jest widoczna w profilu.
* **Krok 10: Integracje z Kalendarzem:** Zaimplementuj OAuth2 dla Google/Outlook i synchronizację zadań.
    * **Test:** Stwórz zadanie z datą w aplikacji. Sprawdź, czy wydarzenie o tej samej nazwie i dacie pojawiło się w Kalendarzu Google.