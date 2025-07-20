# PDRPG Local API Documentation

**PDRPG Local HTTP API Server** - umożliwia zewnętrznym narzędziom dostęp do danych aplikacji poprzez RESTful endpoints.

## 🚀 Uruchamianie API Server

### Metoda 1: Przez interfejs użytkownika (zalecana)
1. Uruchom aplikację: `pnpm tauri dev`
2. Przejdź do zakładki "🌐 API" w aplikacji
3. Kliknij przycisk "🚀 Start API Server"
4. Serwer zostanie uruchomiony na porcie 3000 (lub innym wybranym)

### Metoda 2: Programowo przez Tauri Commands
```javascript
import { invoke } from '@tauri-apps/api/core';

// Uruchom API server na porcie 3000
const result = await invoke('start_api_server', { port: 3000 });

// Sprawdź status API server
const isRunning = await invoke('check_api_status', { port: 3000 });
```

## 📋 Dostępne Endpoints

Base URL: `http://localhost:{PORT}` (domyślnie port 3000)

### 🔍 Health Check
**GET** `/api/health`

Sprawdza status serwera API.

**Odpowiedź:**
```json
{
  "status": "ok",
  "service": "PDRPG API",
  "version": "1.0.0",
  "timestamp": "2025-01-21T..."
}
```

---

### 📝 Tasks (Zadania)
**GET** `/api/tasks`

Pobiera wszystkie zadania z aplikacji.

**Odpowiedź:**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "title": "Ukończ projekt",
      "completed": false,
      "created_at": 1642780800,
      "updated_at": 1642780800
    }
  ],
  "count": 1
}
```

---

### 🎯 Habits (Nawyki)
**GET** `/api/habits`

Pobiera wszystkie nawyki wraz z dzisiejszymi wpisami.

**Odpowiedź:**
```json
{
  "success": true,
  "data": [
    {
      "habit": {
        "id": 1,
        "title": "Codzienna medytacja",
        "habit_type": "Boolean",
        "target_value": null,
        "current_streak": 7,
        "created_at": 1642780800,
        "updated_at": 1642780800
      },
      "today_entry": {
        "id": 15,
        "habit_id": 1,
        "date": "2025-01-21",
        "completed": true,
        "value": 0,
        "created_at": 1642867200
      },
      "today_completed": true
    }
  ],
  "count": 1,
  "date": "2025-01-21"
}
```

**Typy nawyków:**
- `Boolean`: Tak/Nie (medytacja, czytanie, etc.)
- `Counter`: Licznik z opcjonalną wartością docelową (szklanki wody, km biegu)

---

### ⚔️ Character (Postać)
**GET** `/api/character`

Pobiera dane postaci gracza z informacjami o poziomie i EXP.

**Odpowiedź:**
```json
{
  "success": true,
  "data": {
    "character": {
      "id": 1,
      "level": 3,
      "experience": 485,
      "character_class": "Warrior",
      "attributes": {
        "strength": 12,
        "intelligence": 8,
        "charisma": 5,
        "dexterity": 7,
        "wisdom": 10,
        "constitution": 9
      },
      "created_at": 1642780800,
      "updated_at": 1642867200
    },
    "level_progress": {
      "current_level_exp": 400,
      "next_level_exp": 900,
      "progress_percentage": 17.0,
      "exp_to_next_level": 415
    }
  }
}
```

**Klasy postaci:**
- `Warrior`: Skupia się na sile i kondycji
- `Mage`: Rozwija intelekt i mądrość  
- `Bard`: Buduje charyzmę i kreatywność
- `Rogue`: Zwiększa zręczność i spryt

## 🔧 Konfiguracja

### Porty
- **Domyślny port:** 3000
- **Dostępne porty:** 1000-65535
- **Konfiguracja:** Przez interfejs aplikacji w zakładce API

### CORS
API server ma włączone CORS dla wszystkich origins i metod HTTP, umożliwiając dostęp z:
- Przeglądarek internetowych
- Skryptów JavaScript
- Aplikacji zewnętrznych
- Narzędzi API (Postman, curl, etc.)

### Bezpieczeństwo
- **Lokalny dostęp:** Server działa tylko na `127.0.0.1` (localhost)
- **Brak uwierzytelniania:** API jest otwarte dla wszystkich lokalnych połączeń
- **Read-only:** Obecnie wszystkie endpoints są tylko do odczytu

## 🛠️ Przykłady użycia

### cURL
```bash
# Health check
curl http://localhost:3000/api/health

# Pobierz wszystkie zadania
curl http://localhost:3000/api/tasks

# Pobierz nawyki z dzisiejszymi wpisami
curl http://localhost:3000/api/habits

# Pobierz dane postaci
curl http://localhost:3000/api/character
```

### JavaScript/Node.js
```javascript
// Pobierz wszystkie zadania
async function getTasks() {
  const response = await fetch('http://localhost:3000/api/tasks');
  const data = await response.json();
  
  if (data.success) {
    console.log(`Znaleziono ${data.count} zadań:`, data.data);
  }
}

// Sprawdź postęp postaci
async function getCharacterProgress() {
  const response = await fetch('http://localhost:3000/api/character');
  const data = await response.json();
  
  if (data.success) {
    const { character, level_progress } = data.data;
    console.log(`Poziom ${character.level} (${level_progress.progress_percentage.toFixed(1)}% do następnego)`);
  }
}
```

### Python
```python
import requests

# Pobierz nawyki
def get_habits():
    response = requests.get('http://localhost:3000/api/habits')
    data = response.json()
    
    if data['success']:
        for habit_data in data['data']:
            habit = habit_data['habit']
            completed = habit_data['today_completed']
            status = "✅" if completed else "❌"
            print(f"{status} {habit['title']} (streak: {habit['current_streak']})")

get_habits()
```

## 🤖 Integracja z AI

API umożliwia łatwą integrację z narzędziami AI do:

### Analizy postępów
```python
# Przykład skryptu AI do analizy nawyków
def analyze_habit_consistency():
    habits = requests.get('http://localhost:3000/api/habits').json()
    character = requests.get('http://localhost:3000/api/character').json()
    
    # Analiza streaks, predykcje, rekomendacje
    # Generowanie raportów, wykresów
    # Automatyczne komentarze motywacyjne
```

### Automatyzacji
- Automatyczne tworzenie raportów postępów
- Integracja z zewnętrznymi kalendarzami  
- Synchronizacja z innymi aplikacjami fitness/productivity
- AI-powered coaching i rekomendacje

### Dashboardy zewnętrzne
- Własne dashboardy w React/Vue/Angular
- Integracja z Grafana, Power BI
- Mobile apps z dostępem do danych PDRPG

## 🚨 Rozwiązywanie problemów

### API server nie uruchamia się
1. Sprawdź czy port nie jest zajęty: `netstat -an | findstr "3000"`
2. Spróbuj innego portu (np. 3001, 3002)
3. Sprawdź logi w konsoli aplikacji

### Błędy połączenia
1. Upewnij się że API server działa (status "🟢 Running")
2. Sprawdź URL - tylko `http://localhost:PORT` (nie https)
3. Sprawdź firewall - może blokować połączenia

### Błędy CORS
API ma skonfigurowany CORS dla wszystkich origins. Jeśli wystąpią problemy:
1. Używaj `fetch()` zamiast XMLHttpRequest
2. Sprawdź czy używasz prawidłowego protokołu (http, nie https)

## 📈 Roadmap

Planowane rozszerzenia API:
- **POST endpoints:** Tworzenie zadań i nawyków przez API
- **PUT/PATCH endpoints:** Edycja istniejących danych  
- **WebSocket:** Real-time aktualizacje
- **Uwierzytelnianie:** API keys dla bezpieczeństwa
- **Rate limiting:** Ograniczenie zapytań
- **OpenAPI/Swagger:** Automatyczna dokumentacja 