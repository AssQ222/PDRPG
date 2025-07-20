# App Design Document SelfDevelopmentApp

Nazwa pliku `personal-development.md`
Wersja 1.0
Autor AI Assistant
Data 20.07.2025

---

## 1. Wprowadzenie i Wizja

SelfDevelopmentApp to w pełni desktopowa aplikacja do samorozwoju, która łączy zaawansowane narzędzia do zarządzania zadaniami, celami i nawykami z wciągającymi elementami gry RPG. Celem aplikacji jest przekształcenie procesu samodoskonalenia w motywującą i angażującą podróż, gdzie użytkownik widzi realny progres zarówno w życiu, jak i w wirtualnym świecie aplikacji. Aplikacja jest przeznaczona dla osób, które chcą w sposób zorganizowany i konsekwentny pracować nad swoimi celami, jednocześnie czerpiąc z tego satysfakcję i zabawę.

Kluczowe założenia
 Platforma Aplikacja natywna na systemy desktopowe (Windows, macOS).
 Model Aplikacja, nie gra. Elementy RPG są mechanizmem motywacyjnym, a nie głównym celem.
 Integracje Możliwość integracji z kalendarzami (Google, Outlook) oraz otwarte API do zaawansowanej analityki danych z pomocą AI.

---

## 2. Główne Moduły i Funkcjonalności

### 2.1. Dashboard
Główny ekran aplikacji, który w przejrzysty sposób podsumowuje najważniejsze informacje ze wszystkich modułów.
 Widżety
     Podsumowanie zadań na dziś.
     Aktualne streaki z habit trackera.
     Postęp w realizacji kluczowego celu kwartalnego (%).
     Skrót z kalendarza na nadchodzące 3 wydarzenia.
     Statystyki postaci (poziom, EXP, postęp do następnego poziomu).
     Aktywne Questy Tygodniowe.

### 2.2. Tablica Zadań  Inbox Zero
Miejsce do szybkiego zapisywania wszystkich myśli, pomysłów i zadań.
 Funkcjonalność
     Proste pole tekstowe do błyskawicznego dodawania zadań.
     Możliwość późniejszego przypisania zadania do konkretnego projektu, daty w kalendarzu lub usunięcia go (triage).
     System tagów i priorytetów.

### 2.3. Harmonogram Tygodniowy i Kalendarz
Wizualne narzędzie do planowania czasu, łączące zadania z realnym harmonogramem.
 Funkcjonalność
     Widok dzienny i tygodniowy.
     Funkcja przeciągnij i upuść (drag-and-drop) do planowania zadań z Inboxa.
     Dwukierunkowa integracja z kalendarzem Google i Outlook.

### 2.4. Habit Tracker
Moduł do śledzenia i budowania nawyków.
 Funkcjonalność
     Definiowanie nawyków dziennych i tygodniowych.
     Prosty system oznaczania TakNie lub licznik (np. wypij 4 szklanki wody).
     Wizualizacja postępów (streaki, kalendarz wypełnienia).
     Konfigurowalne przypomnienia push.

### 2.5. Cele i Kamienie Milowe (OKRSMART)
Hierarchiczna struktura do zarządzania celami długo- i krótkoterminowymi.
 Funkcjonalność
     Struktura drzewa Wizja  Cel Roczny  Cel Kwartalny  Cel Miesięczny.
     Możliwość powiązania konkretnych zadań z Inboxa z poszczególnymi celami, co pozwala śledzić realny wkład pracy w ich realizację.
     Wizualizacja postępów w formie procentowej dla każdego celu.

### 2.6. Dodatkowe Narzędzia
 Focus Timer
     Zintegrowany timer oparty na technice Pomodoro (konfigurowalne interwały pracy i przerw).
     Możliwość powiązania sesji timera z konkretnym zadaniem.
 Tracker Pomiarów Ciała
     Codzienne wprowadzanie wagi z automatycznym wyliczaniem średniej tygodniowej.
     Możliwość dodawania pomiarów klatki piersiowej, pasa, bicepsa, uda, bioder.
     Graficzne podpowiedzi z zaznaczonymi miejscami do prawidłowego dokonywania pomiarów.
     Wykresy pokazujące progres w czasie.

---

## 3. Elementy Grywalizacji (RPG)

Grywalizacja ma na celu zwiększenie motywacji i zaangażowania użytkownika.

### 3.1. Awatar i Poziomy
Każdy użytkownik ma swojego personalizowanego awatara.
 Mechanika Użytkownik zdobywa punkty doświadczenia (EXP) za
     Ukończenie zadań (więcej EXP za zadania powiązane z celami).
     Utrzymywanie nawyków (codzienny bonus EXP za każdy nawyk).
     Ukończenie sesji z Focus Timerem.
 Nagrody za zdobycie poziomu Zdobycie nowego poziomu odblokowuje nagrody, które motywują do dalszej gry i rozwoju. Mogą to być
     Elementy kosmetyczne dla awatara Nowe ubrania, fryzury, tła.
     Nowe skórki i motywy dla interfejsu aplikacji.
     Odblokowanie nowych klas postaci (patrz niżej).
     Punkty Talentu, które można zainwestować w rozwój klasy postaci.

### 3.2. Klasy Postaci i Progres
Klasy symbolizują obszary życia, na których skupia się użytkownik.
 Mechanika Użytkownik może wybrać jedną lub więcej klas, np. Wojownik (rozwój fizyczny, sport), Mag (rozwój intelektualny, nauka), Bard (kreatywność, umiejętności społeczne), Łotrzyk (finanse, przedsiębiorczość).
 Progres Wykonywanie zadań i nawyków z danej kategorii (oznaczonych tagiem klasy) rozwija konkretną klasę.
 Wizualizacja Postęp jest przedstawiony na atrakcyjnym wizualnie wykresie pajęczynowym. Każde ramię pajęczyny to inna klasa lub atrybut (np. Siła, Intelekt, Charyzma). Wypełniający się kolorami wykres daje natychmiastową informację zwrotną o zbalansowanym (lub nie) rozwoju. To, co przyciąga uwagę w grach, to widoczny i satysfakcjonujący feedback – użytkownik musi czuć, że jego wysiłek bezpośrednio przekłada się na moc postaci.

### 3.3. Questy Tygodniowe
Spersonalizowane wyzwania generowane przez algorytm.
 Mechanika Co tydzień aplikacja analizuje
     Zadania zaległe i te o najwyższym priorytecie.
     Nawyki, z którymi użytkownik ma problemy.
     Główne cele kwartalne.
    Na tej podstawie generuje 3-5 Questów Tygodniowych, np. Ukończ 5 zadań związanych z celem 'Nauka Języka' lub Utrzymaj nawyk 'Codzienna medytacja' przez 7 dni z rzędu. Ukończenie questów daje duży bonus EXP i inne nagrody.

### 3.4. Odznaki za Przełomy (Achievements)
System nagród za osiągnięcie kamieni milowych.
 Mechanika Użytkownik zdobywa unikalne odznaki za
     Streaki w nawykach Specjalne odznaki za 30, 90, 150 i 230 dni ciągłości.
     Osiągnięcia projektowe Ukończenie ważnego celu, zamknięcie dużego projektu.
     Konsekwencję Np. Wykonano 1000 zadań lub Przepracowano 100 godzin z Focus Timerem.
 Nagrody Zdobycie rzadkiej odznaki może wiązać się z jednorazową, wartościową nagrodą, np. unikalnym przedmiotem dla awatara lub odblokowaniem specjalnej funkcji w aplikacji.

---

## 4. Wymagania Techniczne

 Typ Aplikacji Natywna aplikacja desktopowa (nie webowa ani PWA).
 API Aplikacja musi posiadać wbudowane API, które umożliwi zaawansowanym użytkownikom lub zewnętrznym narzędziom (np. skryptom AI) na
     Pobieranie danych o zadaniach, postępach i nawykach.
     Generowanie spersonalizowanych podsumowań i wykresów.
     Automatyzację wprowadzania danych.
 Przechowywanie danych Dane przechowywane lokalnie z opcjonalną synchronizacją w chmurze (end-to-end encryption) w celu używania na wielu urządzeniach.