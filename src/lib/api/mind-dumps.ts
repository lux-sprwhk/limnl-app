import { invoke } from '@tauri-apps/api/core';
import type { MindDump, CreateMindDumpInput, UpdateMindDumpInput, MindDumpAnalysisWithCardsAndTasks } from '$lib/types/mind-dump';
import type { LLMConfig } from '$lib/types/llm';
import { llmSettings } from '$lib/stores/llm-settings.svelte';

export const mindDumpApi = {
	async create(input: CreateMindDumpInput): Promise<MindDump> {
		const config: LLMConfig = llmSettings.config;
		return await invoke<MindDump>('create_mind_dump', { input, config });
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
	},

	async getAnalysis(mindDumpId: number): Promise<MindDumpAnalysisWithCardsAndTasks | null> {
		return await invoke<MindDumpAnalysisWithCardsAndTasks | null>('get_mind_dump_analysis_with_cards_and_tasks', { mindDumpId });
	}
};
