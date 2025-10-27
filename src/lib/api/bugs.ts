import { invoke } from '@tauri-apps/api/core';
import type { Bug, CreateBugInput, UpdateBugInput } from '$lib/types/bug';

export const bugsApi = {
	async create(input: CreateBugInput): Promise<Bug> {
		return await invoke<Bug>('create_bug', { input });
	},

	async get(id: number): Promise<Bug | null> {
		return await invoke<Bug | null>('get_bug', { id });
	},

	async list(status?: string): Promise<Bug[]> {
		return await invoke<Bug[]>('list_bugs', { status });
	},

	async update(input: UpdateBugInput): Promise<Bug | null> {
		return await invoke<Bug | null>('update_bug', { input });
	},

	async delete(id: number): Promise<boolean> {
		return await invoke<boolean>('delete_bug', { id });
	}
};
