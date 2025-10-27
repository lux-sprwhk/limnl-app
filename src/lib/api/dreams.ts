import { invoke } from '@tauri-apps/api/core';
import type { Dream, CreateDreamInput, UpdateDreamInput } from '$lib/types/dream';

export const dreamsApi = {
	async create(input: CreateDreamInput): Promise<Dream> {
		return await invoke<Dream>('create_dream', { input });
	},

	async get(id: number): Promise<Dream | null> {
		return await invoke<Dream | null>('get_dream', { id });
	},

	async list(limit?: number, offset?: number): Promise<Dream[]> {
		return await invoke<Dream[]>('list_dreams', { limit, offset });
	},

	async update(input: UpdateDreamInput): Promise<Dream | null> {
		return await invoke<Dream | null>('update_dream', { input });
	},

	async delete(id: number): Promise<boolean> {
		return await invoke<boolean>('delete_dream', { id });
	},

	async search(query: string): Promise<Dream[]> {
		return await invoke<Dream[]>('search_dreams', { query });
	}
};
