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
	},

	optimizeBugDescription: async (request: { content: string }) => {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM provider is disabled. Please configure an LLM provider in settings.');
		}

		if (!request.content || request.content.trim().length === 0) {
			throw new Error('Bug description is required to optimize.');
		}

		try {
			const response = await invoke<{ optimized: string }>('optimize_bug_description', {
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
			throw new Error('Failed to optimize bug description');
		}
	},

	generateBugTitle: async (request: { content: string }) => {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM provider is disabled. Please configure an LLM provider in settings.');
		}

		if (!request.content || request.content.trim().length === 0) {
			throw new Error('Bug description is required to generate a title.');
		}

		try {
			const response = await invoke<GenerateTitleResponse>('generate_bug_title', {
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
			throw new Error('Failed to generate bug title');
		}
	},

	commentOnCard: async (request: { cardName: string; cardQuestion: string; cardMeaning: string; lifeArea: string }) => {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM provider is disabled. Please configure an LLM provider in settings.');
		}

		try {
			const response = await invoke<{ commentary: string }>('comment_on_card', {
				request: {
					card_name: request.cardName,
					card_question: request.cardQuestion,
					card_meaning: request.cardMeaning,
					life_area: request.lifeArea,
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
			throw new Error('Failed to generate card commentary');
		}
	},

	commentOnMultipleCards: async (request: { cards: Array<{ id: number; name: string; question: string; meaning: string }>; lifeArea: string }) => {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM provider is disabled. Please configure an LLM provider in settings.');
		}

		try {
			const response = await invoke<{ commentaries: Record<string, string> }>('comment_on_multiple_cards', {
				request: {
					cards: request.cards,
					life_area: request.lifeArea,
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
			throw new Error('Failed to generate card commentaries');
		}
	},

	chatWithHistory: async (request: { userMessage: string; messages: Array<{ role: 'user' | 'assistant'; content: string }>; cardName: string; cardQuestion: string; cardMeaning: string; lifeArea: string }) => {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM provider is disabled. Please configure an LLM provider in settings.');
		}

		try {
			const response = await invoke<{ response: string }>('chat_with_history', {
				request: {
					user_message: request.userMessage,
					messages: request.messages,
					card_name: request.cardName,
					card_question: request.cardQuestion,
					card_meaning: request.cardMeaning,
					life_area: request.lifeArea,
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
			throw new Error('Failed to get chat response');
		}
	}
};
