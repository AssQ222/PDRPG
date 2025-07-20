import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Task, CreateTaskRequest, TaskLoadingState, TaskError } from '../types/task';

/**
 * Stan store'a dla zadań
 */
interface TaskStoreState {
	/** Lista wszystkich zadań */
	tasks: Task[];
	/** Stan ładowania */
	loading: TaskLoadingState;
	/** Błąd (jeśli wystąpił) */
	error: TaskError | null;
}

/** Początkowy stan store'a */
const initialState: TaskStoreState = {
	tasks: [],
	loading: 'idle',
	error: null
};

/** Główny store dla zadań */
const taskStore = writable<TaskStoreState>(initialState);

/**
 * Funkcje do zarządzania zadaniami
 */
export const taskActions = {
	/**
	 * Ładuje wszystkie zadania z backendu
	 */
	async loadTasks(): Promise<void> {
		taskStore.update(state => ({ ...state, loading: 'loading', error: null }));

		try {
			const tasks: Task[] = await invoke('get_all_tasks');
			taskStore.update(state => ({
				...state,
				tasks,
				loading: 'success',
				error: null
			}));
		} catch (error) {
			console.error('Failed to load tasks:', error);
			taskStore.update(state => ({
				...state,
				loading: 'error',
				error: {
					message: error instanceof Error ? error.message : 'Failed to load tasks',
					code: 'LOAD_TASKS_ERROR'
				}
			}));
		}
	},

	/**
	 * Dodaje nowe zadanie
	 */
	async addTask(title: string): Promise<void> {
		if (!title.trim()) {
			taskStore.update(state => ({
				...state,
				error: {
					message: 'Task title cannot be empty',
					code: 'EMPTY_TITLE'
				}
			}));
			return;
		}

		taskStore.update(state => ({ ...state, loading: 'loading', error: null }));

		try {
			const newTask: Task = await invoke('add_task', { title: title.trim() });
			taskStore.update(state => ({
				...state,
				tasks: [newTask, ...state.tasks], // Dodaj na początku (najnowsze pierwsze)
				loading: 'success',
				error: null
			}));
		} catch (error) {
			console.error('Failed to add task:', error);
			taskStore.update(state => ({
				...state,
				loading: 'error',
				error: {
					message: error instanceof Error ? error.message : 'Failed to add task',
					code: 'ADD_TASK_ERROR'
				}
			}));
		}
	},

	/**
	 * Przełącza status ukończenia zadania
	 */
	async toggleTaskStatus(taskId: number): Promise<void> {
		try {
			const updatedTask: Task = await invoke('toggle_task_status', { taskId });
			taskStore.update(state => ({
				...state,
				tasks: state.tasks.map(task => 
					task.id === taskId ? updatedTask : task
				),
				error: null
			}));
		} catch (error) {
			console.error('Failed to toggle task status:', error);
			taskStore.update(state => ({
				...state,
				error: {
					message: error instanceof Error ? error.message : 'Failed to toggle task status',
					code: 'TOGGLE_TASK_ERROR'
				}
			}));
		}
	},

	/**
	 * Usuwa zadanie
	 */
	async deleteTask(taskId: number): Promise<void> {
		try {
			await invoke('delete_task', { taskId });
			taskStore.update(state => ({
				...state,
				tasks: state.tasks.filter(task => task.id !== taskId),
				error: null
			}));
		} catch (error) {
			console.error('Failed to delete task:', error);
			taskStore.update(state => ({
				...state,
				error: {
					message: error instanceof Error ? error.message : 'Failed to delete task',
					code: 'DELETE_TASK_ERROR'
				}
			}));
		}
	},

	/**
	 * Czyści błędy
	 */
	clearError(): void {
		taskStore.update(state => ({ ...state, error: null }));
	}
};

/** Store do-tylko-do-odczytu dla zadań */
export const tasks = derived(taskStore, $store => $store.tasks);

/** Store dla stanu ładowania */
export const isLoading = derived(taskStore, $store => $store.loading === 'loading');

/** Store dla błędów */
export const taskError = derived(taskStore, $store => $store.error);

/** Store dla ukończonych zadań */
export const completedTasks = derived(taskStore, $store => 
	$store.tasks.filter(task => task.completed)
);

/** Store dla nieukończonych zadań */
export const pendingTasks = derived(taskStore, $store => 
	$store.tasks.filter(task => !task.completed)
);

/** Store dla liczby zadań */
export const taskStats = derived(taskStore, $store => ({
	total: $store.tasks.length,
	completed: $store.tasks.filter(task => task.completed).length,
	pending: $store.tasks.filter(task => !task.completed).length
}));

/** Główny export store'a (tylko do odczytu) */
export default { subscribe: taskStore.subscribe }; 