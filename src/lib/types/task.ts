/**
 * Interfejs reprezentujący zadanie w aplikacji PDRPG
 * Zgodny z modelem Task z backendu Rust
 */
export interface Task {
	/** Unikalny identyfikator zadania */
	id: number;
	/** Tytuł/nazwa zadania */
	title: string;
	/** Status ukończenia zadania */
	completed: boolean;
	/** Timestamp utworzenia zadania (Unix timestamp) */
	created_at: number;
	/** Timestamp ostatniej modyfikacji (Unix timestamp) */
	updated_at: number;
}

/**
 * Interfejs dla danych tworzenia nowego zadania
 */
export interface CreateTaskRequest {
	/** Tytuł nowego zadania */
	title: string;
}

/**
 * Typy stanów dla zarządzania operacjami asynchronicznymi
 */
export type TaskLoadingState = 'idle' | 'loading' | 'success' | 'error';

/**
 * Interfejs dla błędów związanych z zadaniami
 */
export interface TaskError {
	/** Wiadomość błędu */
	message: string;
	/** Kod błędu (opcjonalny) */
	code?: string;
} 