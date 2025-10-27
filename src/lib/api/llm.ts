import type { GenerateTitleRequest, GenerateTitleResponse } from '$lib/types/llm';
import { llmSettings } from '$lib/stores/llm-settings.svelte';
import { invoke } from '@tauri-apps/api/core';

export const llmApi = {
	async generateTitle(request: GenerateTitleRequest): Promise<GenerateTitleResponse> {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM is disabled. Please configure an LLM provider in settings.');
		}

		if (!request.content || request.content.trim().length === 0) {
			throw new Error('Dream content is required to generate a title.');
		}

		try {
			const response = await invoke<GenerateTitleResponse>('generate_dream_title', {
				request: {
					content: request.content,
					config: config
				}
			});
			return response;
		} catch (error) {
			if (error instanceof Error) {
				throw error;
			}
			if (typeof error === 'string') {
				throw new Error(error);
			}
			throw new Error('Failed to generate title');
		}
	},

	optimizeDescription: async (request: { content: string }) => {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM provider is disabled. Please configure an LLM provider in settings.');
		}

		if (!request.content || request.content.trim().length === 0) {
			throw new Error('Dream content is required to optimize description.');
		}

		try {
			const response = await invoke<{ optimized: string }>('optimize_dream_description', {
				request: {
					content: request.content,
					config: config
				}
			});
			return response;
		} catch (error) {
			if (error instanceof Error) {
				throw error;
			}
			if (typeof error === 'string') {
				throw new Error(error);
			}
			throw new Error('Failed to optimize description');
		}
	}
};
