import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type {
	Habit,
	HabitEntry,
	CreateHabitRequest,
	UpdateHabitRequest,
	CreateHabitEntryRequest,
	HabitLoadingState,
	HabitError,
	HabitWithEntry,
	HabitStats,
	DateString
} from '../types/habit';
import { characterActions } from './characterStore';

/**
 * Stan store'a dla nawyków
 */
interface HabitStoreState {
	/** Lista wszystkich nawyków */
	habits: Habit[];
	/** Wpisy nawyków na dzisiaj */
	todayEntries: HabitEntry[];
	/** Stan ładowania */
	loading: HabitLoadingState;
	/** Błąd (jeśli wystąpił) */
	error: HabitError | null;
}

/** Początkowy stan store'a */
const initialState: HabitStoreState = {
	habits: [],
	todayEntries: [],
	loading: 'idle',
	error: null
};

/** Główny store dla nawyków */
const habitStore = writable<HabitStoreState>(initialState);

/** Funkcja pomocnicza do formatowania dzisiejszej daty */
function getTodayDateString(): DateString {
	return new Date().toISOString().split('T')[0]; // YYYY-MM-DD
}

/**
 * Funkcje do zarządzania nawykami
 */
export const habitActions = {
	/**
	 * Ładuje wszystkie nawyki z backendu
	 */
	async loadHabits(): Promise<void> {
		habitStore.update(state => ({ ...state, loading: 'loading', error: null }));

		try {
			const habits: Habit[] = await invoke('get_all_habits');
			habitStore.update(state => ({
				...state,
				habits,
				loading: 'success',
				error: null
			}));
		} catch (error) {
			console.error('Failed to load habits:', error);
			habitStore.update(state => ({
				...state,
				loading: 'error',
				error: {
					message: error instanceof Error ? error.message : 'Failed to load habits',
					code: 'LOAD_HABITS_ERROR'
				}
			}));
		}
	},

	/**
	 * Ładuje wpisy nawyków na dzisiaj
	 */
	async loadTodayEntries(): Promise<void> {
		try {
			const today = getTodayDateString();
			const entries: HabitEntry[] = await invoke('get_habit_entries_for_date', { date: today });
			habitStore.update(state => ({
				...state,
				todayEntries: entries,
				error: null
			}));
		} catch (error) {
			console.error('Failed to load today entries:', error);
			habitStore.update(state => ({
				...state,
				error: {
					message: error instanceof Error ? error.message : 'Failed to load today entries',
					code: 'LOAD_ENTRIES_ERROR'
				}
			}));
		}
	},

	/**
	 * Dodaje nowy nawyk
	 */
	async addHabit(title: string, habitType: 'Boolean' | 'Counter', targetValue?: number): Promise<void> {
		if (!title.trim()) {
			habitStore.update(state => ({
				...state,
				error: {
					message: 'Habit title cannot be empty',
					code: 'EMPTY_TITLE'
				}
			}));
			return;
		}

		habitStore.update(state => ({ ...state, loading: 'loading', error: null }));

		try {
			const newHabit: Habit = await invoke('add_habit', {
				request: {
					title: title.trim(),
					habit_type: habitType,
					target_value: targetValue
				}
			});
			habitStore.update(state => ({
				...state,
				habits: [newHabit, ...state.habits], // Dodaj na początku (najnowsze pierwsze)
				loading: 'success',
				error: null
			}));
		} catch (error) {
			console.error('Failed to add habit:', error);
			habitStore.update(state => ({
				...state,
				loading: 'error',
				error: {
					message: error instanceof Error ? error.message : 'Failed to add habit',
					code: 'ADD_HABIT_ERROR'
				}
			}));
		}
	},

	/**
	 * Usuwa nawyk
	 */
	async deleteHabit(habitId: number): Promise<void> {
		try {
			await invoke('delete_habit', { id: habitId });
			habitStore.update(state => ({
				...state,
				habits: state.habits.filter(habit => habit.id !== habitId),
				todayEntries: state.todayEntries.filter(entry => entry.habit_id !== habitId),
				error: null
			}));
		} catch (error) {
			console.error('Failed to delete habit:', error);
			habitStore.update(state => ({
				...state,
				error: {
					message: error instanceof Error ? error.message : 'Failed to delete habit',
					code: 'DELETE_HABIT_ERROR'
				}
			}));
		}
	},

	/**
	 * Aktualizuje nawyk
	 */
	async updateHabit(habitId: number, updates: UpdateHabitRequest): Promise<void> {
		try {
			const updatedHabit: Habit = await invoke('update_habit', {
				habit_id: habitId,
				request: {
					title: updates.title,
					target_value: updates.target_value
				}
			});
			habitStore.update(state => ({
				...state,
				habits: state.habits.map(habit =>
					habit.id === habitId ? updatedHabit : habit
				),
				error: null
			}));
		} catch (error) {
			console.error('Failed to update habit:', error);
			habitStore.update(state => ({
				...state,
				error: {
					message: error instanceof Error ? error.message : 'Failed to update habit',
					code: 'UPDATE_HABIT_ERROR'
				}
			}));
		}
	},

	/**
	 * Dodaje lub aktualizuje wpis nawyku na dzisiaj
	 */
	async markHabitToday(habitId: number, completed?: boolean, value?: number): Promise<void> {
		try {
			const today = getTodayDateString();
			const request: CreateHabitEntryRequest = {
				habit_id: habitId,
				date: today,
				completed,
				value
			};

			const entry: HabitEntry = await invoke('add_habit_entry', {
				request: {
					habit_id: request.habit_id,
					date: request.date,
					completed: request.completed,
					value: request.value
				}
			});

			// Pobierz zaktualizowany nawyk (z nowym streak)
			await this.loadHabits();

			// Zaktualizuj wpisy na dzisiaj
			habitStore.update(state => {
				const updatedEntries = state.todayEntries.filter(e => e.habit_id !== habitId);
				return {
					...state,
					todayEntries: [...updatedEntries, entry],
					error: null
				};
			});

			// Odśwież character store po wykonaniu nawyku (może dodać EXP i atrybuty)
			await characterActions.getCharacter();
			console.log('✅ Character data refreshed after habit completion');
		} catch (error) {
			console.error('Failed to mark habit today:', error);
			habitStore.update(state => ({
				...state,
				error: {
					message: error instanceof Error ? error.message : 'Failed to mark habit',
					code: 'MARK_HABIT_ERROR'
				}
			}));
		}
	},

	/**
	 * Pobiera wpisy dla konkretnego nawyku
	 */
	async getHabitEntries(habitId: number): Promise<HabitEntry[]> {
		try {
			const entries: HabitEntry[] = await invoke('get_habit_entries_for_habit', { habit_id: habitId });
			return entries;
		} catch (error) {
			console.error('Failed to get habit entries:', error);
			habitStore.update(state => ({
				...state,
				error: {
					message: error instanceof Error ? error.message : 'Failed to get habit entries',
					code: 'GET_ENTRIES_ERROR'
				}
			}));
			return [];
		}
	},

	/**
	 * Czyści błędy
	 */
	clearError(): void {
		habitStore.update(state => ({ ...state, error: null }));
	},

	/**
	 * Inicjalizuje store (ładuje nawyki i dzisiejsze wpisy)
	 */
	async initialize(): Promise<void> {
		await Promise.all([
			this.loadHabits(),
			this.loadTodayEntries()
		]);
	}
};

/** Store do-tylko-do-odczytu dla nawyków */
export const habits = derived(habitStore, $store => $store.habits);

/** Store dla dzisiejszych wpisów */
export const todayEntries = derived(habitStore, $store => $store.todayEntries);

/** Store dla stanu ładowania */
export const isLoading = derived(habitStore, $store => $store.loading === 'loading');

/** Store dla błędów */
export const habitError = derived(habitStore, $store => $store.error);

/** Store dla nawyków z dzisiejszymi wpisami */
export const habitsWithEntries = derived(
	[habits, todayEntries],
	([$habits, $todayEntries]) => {
		return $habits.map(habit => {
			const todayEntry = $todayEntries.find(entry => entry.habit_id === habit.id);

			let completedToday = false;
			let todayValue = 0;

			if (todayEntry) {
				if (habit.habit_type === 'Boolean') {
					completedToday = todayEntry.completed;
				} else if (habit.habit_type === 'Counter') {
					todayValue = todayEntry.value;
					completedToday = habit.target_value ?
						todayEntry.value >= habit.target_value :
						todayEntry.value > 0;
				}
			}

			const result: HabitWithEntry = {
				habit,
				today_entry: todayEntry,
				completed_today: completedToday,
				today_value: todayValue
			};

			return result;
		});
	}
);

/** Store dla statystyk nawyków */
export const habitStats = derived(habits, $habits => {
	const total = $habits.length;
	const withStreak = $habits.filter(h => h.current_streak > 0).length;
	const allStreaks = $habits.map(h => h.current_streak);
	const averageStreak = total > 0 ? allStreaks.reduce((a, b) => a + b, 0) / total : 0;
	const longestStreak = total > 0 ? Math.max(...allStreaks) : 0;

	const stats: HabitStats = {
		total,
		with_streak: withStreak,
		average_streak: Math.round(averageStreak * 100) / 100, // Round to 2 decimal places
		longest_streak: longestStreak
	};

	return stats;
});

/** Główny export store'a (tylko do odczytu) */
export default { subscribe: habitStore.subscribe }; 