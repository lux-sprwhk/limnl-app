import { invoke } from '@tauri-apps/api/core';
import type { JournalEntry, CreateJournalInput, UpdateJournalInput } from '$lib/types/journal';

export const journalApi = {
	async create(input: CreateJournalInput): Promise<JournalEntry> {
		return await invoke<JournalEntry>('create_journal_entry', { input });
	},

	async get(id: number): Promise<JournalEntry | null> {
		return await invoke<JournalEntry | null>('get_journal_entry', { id });
	},

	async list(limit?: number, offset?: number): Promise<JournalEntry[]> {
		return await invoke<JournalEntry[]>('list_journal_entries', { limit, offset });
	},

	async update(input: UpdateJournalInput): Promise<JournalEntry | null> {
		return await invoke<JournalEntry | null>('update_journal_entry', { input });
	},

	async delete(id: number): Promise<boolean> {
		return await invoke<boolean>('delete_journal_entry', { id });
	},

	async search(query: string): Promise<JournalEntry[]> {
		return await invoke<JournalEntry[]>('search_journal_entries', { query });
	}
};
