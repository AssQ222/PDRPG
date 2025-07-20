/**
 * Typ nawyku określający sposób śledzenia
 */
export type HabitType = 'Boolean' | 'Counter';

/**
 * Interfejs reprezentujący nawyk w aplikacji PDRPG
 * Zgodny z modelem Habit z backendu Rust
 */
export interface Habit {
	/** Unikalny identyfikator nawyku */
	id: number;
	/** Nazwa nawyku */
	title: string;
	/** Typ nawyku (Boolean vs Counter) */
	habit_type: HabitType;
	/** Wartość docelowa (dla typu Counter, np. 8 szklanek wody) */
	target_value?: number;
	/** Aktualny ciąg dni (streak) */
	current_streak: number;
	/** Timestamp utworzenia nawyku (Unix timestamp) */
	created_at: number;
	/** Timestamp ostatniej modyfikacji (Unix timestamp) */
	updated_at: number;
}

/**
 * Interfejs reprezentujący wpis nawyku na konkretny dzień
 */
export interface HabitEntry {
	/** Unikalny identyfikator wpisu */
	id: number;
	/** ID nawyku do którego należy wpis */
	habit_id: number;
	/** Data wpisu (YYYY-MM-DD format) */
	date: string;
	/** Czy nawyk został wykonany (dla typu Boolean) */
	completed: boolean;
	/** Wartość dla nawyków typu Counter */
	value: number;
	/** Timestamp utworzenia wpisu */
	created_at: number;
}

/**
 * Interfejs dla danych tworzenia nowego nawyku
 */
export interface CreateHabitRequest {
	/** Nazwa nowego nawyku */
	title: string;
	/** Typ nawyku */
	habit_type: HabitType;
	/** Wartość docelowa (opcjonalna, dla typu Counter) */
	target_value?: number;
}

/**
 * Interfejs dla danych aktualizacji nawyku
 */
export interface UpdateHabitRequest {
	/** Nowa nazwa nawyku */
	title?: string;
	/** Nowa wartość docelowa */
	target_value?: number;
}

/**
 * Interfejs dla danych tworzenia wpisu nawyku
 */
export interface CreateHabitEntryRequest {
	/** ID nawyku */
	habit_id: number;
	/** Data w formacie YYYY-MM-DD */
	date: string;
	/** Czy wykonano (dla typu Boolean) */
	completed?: boolean;
	/** Wartość (dla typu Counter) */
	value?: number;
}

/**
 * Typy stanów dla zarządzania operacjami asynchronicznymi
 */
export type HabitLoadingState = 'idle' | 'loading' | 'success' | 'error';

/**
 * Interfejs dla błędów związanych z nawykami
 */
export interface HabitError {
	/** Wiadomość błędu */
	message: string;
	/** Kod błędu (opcjonalny) */
	code?: string;
}

/**
 * Interfejs dla statystyk nawyków
 */
export interface HabitStats {
	/** Całkowita liczba nawyków */
	total: number;
	/** Liczba nawyków z aktywnym streak */
	with_streak: number;
	/** Średnia długość streak */
	average_streak: number;
	/** Najdłuższy streak */
	longest_streak: number;
}

/**
 * Interfejs dla widoku nawyku z jego dzisiejszym wpisem
 */
export interface HabitWithEntry {
	/** Dane nawyku */
	habit: Habit;
	/** Dzisiejszy wpis (jeśli istnieje) */
	today_entry?: HabitEntry;
	/** Czy nawyk został dzisiaj ukończony */
	completed_today: boolean;
	/** Wartość na dzisiaj (dla typu Counter) */
	today_value: number;
}

/**
 * Typ pomocniczy dla formatowania dat
 */
export type DateString = string; // YYYY-MM-DD format 