import { invoke } from '@tauri-apps/api/core';
import type { MindDump, CreateMindDumpInput, UpdateMindDumpInput } from '$lib/types/mind-dump';

export const mindDumpApi = {
	async create(input: CreateMindDumpInput): Promise<MindDump> {
		return await invoke<MindDump>('create_mind_dump', { input });
	},

	async get(id: number): Promise<MindDump | null> {
		return await invoke<MindDump | null>('get_mind_dump', { id });
	},

	async list(limit?: number, offset?: number): Promise<MindDump[]> {
		return await invoke<MindDump[]>('list_mind_dumps', { limit, offset });
	},

	async update(input: UpdateMindDumpInput): Promise<MindDump | null> {
		return await invoke<MindDump | null>('update_mind_dump', { input });
	},

	async delete(id: number): Promise<boolean> {
		return await invoke<boolean>('delete_mind_dump', { id });
	},

	async search(query: string): Promise<MindDump[]> {
		return await invoke<MindDump[]>('search_mind_dumps', { query });
	}
};
