import { invoke } from '@tauri-apps/api/core';
import type {
	Dream,
	CreateDreamInput,
	UpdateDreamInput,
	DreamAnalysisWithCards,
	GenerateDreamAnalysisRequest,
	DreamCreativePrompts,
	DreamCreativePromptsData,
	GenerateCreativePromptsRequest
} from '$lib/types/dream';

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
	},

	// Dream Analysis
	async generateAnalysis(request: GenerateDreamAnalysisRequest): Promise<DreamAnalysisWithCards> {
		return await invoke<DreamAnalysisWithCards>('generate_dream_analysis', { request });
	},

	async getAnalysisWithCards(dreamId: number): Promise<DreamAnalysisWithCards | null> {
		return await invoke<DreamAnalysisWithCards | null>('get_dream_analysis_with_cards', {
			dreamId
		});
	},

	// Dream Creative Prompts
	async generateCreativePrompts(request: GenerateCreativePromptsRequest): Promise<DreamCreativePrompts> {
		return await invoke<DreamCreativePrompts>('generate_dream_creative_prompts', { request });
	},

	async getCreativePrompts(dreamAnalysisId: number): Promise<DreamCreativePromptsData | null> {
		const result = await invoke<DreamCreativePrompts | null>('get_dream_creative_prompts', {
			dreamAnalysisId
		});

		if (!result) return null;

		// Parse JSON strings to arrays
		return {
			image_prompts: JSON.parse(result.image_prompts),
			music_prompts: JSON.parse(result.music_prompts),
			story_prompts: JSON.parse(result.story_prompts)
		};
	}
};
