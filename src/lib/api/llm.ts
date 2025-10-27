import type { LLMConfig, GenerateTitleRequest, GenerateTitleResponse } from '$lib/types/llm';
import { llmSettings } from '$lib/stores/llm-settings.svelte';

const TITLE_GENERATION_PROMPT = `You are a helpful assistant that generates concise, evocative titles for dream journal entries.

Given the dream content below, generate a short title (2-6 words) that captures the essence of the dream. The title should be memorable and help the dreamer recall the dream later.

Rules:
- Keep it brief (2-6 words)
- Capture the main theme or memorable element
- Don't use quotes around the title
- Respond with ONLY the title, nothing else

Dream content:`;

const DESCRIPTION_OPTIMIZATION_PROMPT = `You are a helpful assistant that optimizes dream journal descriptions for clarity, analysis, and data mining.

Given the raw dream description below, improve it by:
- Organizing the narrative into a clear, coherent structure
- Clarifying ambiguous descriptions
- Highlighting key symbols, emotions, and themes
- Maintaining the dreamer's original voice and perspective
- Making it more suitable for analysis while preserving authenticity

Respond with ONLY the optimized description, nothing else.

Raw dream description:`;

function mapOllamaModel(modelName: string): string {
	const modelMap: Record<string, string> = {
		'llama': 'llama3.2',
		'mistral': 'mistral',
		'phi': 'phi3',
		'deepseek': 'deepseek-coder'
	};
	return modelMap[modelName] || modelName;
}

function mapOpenAIModel(modelName: string): string {
	const modelMap: Record<string, string> = {
		'gpt4-mini': 'gpt-4o-mini',
		'gpt4-turbo': 'gpt-4-turbo',
		'gpt4': 'gpt-4',
		'gpt4o': 'gpt-4o'
	};
	return modelMap[modelName] || modelName;
}

function mapAnthropicModel(modelName: string): string {
	const modelMap: Record<string, string> = {
		'claude-haiku': 'claude-haiku-4.5',
		'claude-sonnet': 'claude-sonnet-4.5'
	};
	return modelMap[modelName] || modelName;
}

async function generateTitleWithOllama(
	content: string,
	config: LLMConfig
): Promise<string> {
	const response = await fetch(`${config.ollamaUrl}/api/generate`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			model: mapOllamaModel(config.ollamaModel),
			prompt: `${TITLE_GENERATION_PROMPT}\n\n${content}`,
			stream: false
		})
	});

	if (!response.ok) {
		throw new Error(`Ollama API error: ${response.statusText}`);
	}

	const data = await response.json();
	return data.response.trim();
}

async function generateTitleWithOpenAI(
	content: string,
	config: LLMConfig
): Promise<string> {
	const response = await fetch('https://api.openai.com/v1/chat/completions', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${config.openaiApiKey}`
		},
		body: JSON.stringify({
			model: mapOpenAIModel(config.openaiModel),
			messages: [
				{
					role: 'system',
					content: TITLE_GENERATION_PROMPT
				},
				{
					role: 'user',
					content: content
				}
			],
			temperature: 0.7,
			max_tokens: 20
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(`OpenAI API error: ${error.error?.message || response.statusText}`);
	}

	const data = await response.json();
	return data.choices[0].message.content.trim();
}

async function optimizeDescriptionWithOllama(
	content: string,
	config: LLMConfig
): Promise<string> {
	const response = await fetch(`${config.ollamaUrl}/api/generate`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			model: mapOllamaModel(config.ollamaModel),
			prompt: `${DESCRIPTION_OPTIMIZATION_PROMPT}\n\n${content}`,
			stream: false
		})
	});

	if (!response.ok) {
		throw new Error(`Ollama API error: ${response.statusText}`);
	}

	const data = await response.json();
	return data.response.trim();
}

async function optimizeDescriptionWithOpenAI(
	content: string,
	config: LLMConfig
): Promise<string> {
	const response = await fetch('https://api.openai.com/v1/chat/completions', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${config.openaiApiKey}`
		},
		body: JSON.stringify({
			model: mapOpenAIModel(config.openaiModel),
			messages: [
				{
					role: 'system',
					content: DESCRIPTION_OPTIMIZATION_PROMPT
				},
				{
					role: 'user',
					content: content
				}
			],
			temperature: 0.7,
			max_tokens: 2000
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(`OpenAI API error: ${error.error?.message || response.statusText}`);
	}

	const data = await response.json();
	return data.choices[0].message.content.trim();
}

async function optimizeDescriptionWithAnthropic(
	content: string,
	config: LLMConfig
): Promise<string> {
	const response = await fetch('https://api.anthropic.com/v1/messages', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			'x-api-key': config.anthropicApiKey,
			'anthropic-version': '2023-06-01'
		},
		body: JSON.stringify({
			model: mapAnthropicModel(config.anthropicModel),
			max_tokens: 2000,
			messages: [
				{
					role: 'user',
					content: `${DESCRIPTION_OPTIMIZATION_PROMPT}\n\n${content}`
				}
			]
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(`Anthropic API error: ${error.error?.message || response.statusText}`);
	}

	const data = await response.json();
	return data.content[0].text.trim();
}

async function generateTitleWithAnthropic(
	content: string,
	config: LLMConfig
): Promise<string> {
	const response = await fetch('https://api.anthropic.com/v1/messages', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			'x-api-key': config.anthropicApiKey,
			'anthropic-version': '2023-06-01'
		},
		body: JSON.stringify({
			model: mapAnthropicModel(config.anthropicModel),
			max_tokens: 20,
			messages: [
				{
					role: 'user',
					content: `${TITLE_GENERATION_PROMPT}\n\n${content}`
				}
			]
		})
	});

	if (!response.ok) {
		const error = await response.json();
		throw new Error(`Anthropic API error: ${error.error?.message || response.statusText}`);
	}

	const data = await response.json();
	return data.content[0].text.trim();
}

export const llmApi = {
	async generateTitle(request: GenerateTitleRequest): Promise<GenerateTitleResponse> {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM is disabled. Please configure an LLM provider in settings.');
		}

		if (!request.content || request.content.trim().length === 0) {
			throw new Error('Dream content is required to generate a title.');
		}

		let title: string;

		try {
			switch (config.provider) {
				case 'ollama':
					title = await generateTitleWithOllama(request.content, config);
					break;
				case 'openai':
					if (!config.openaiApiKey) {
						throw new Error('OpenAI API key not configured.');
					}
					title = await generateTitleWithOpenAI(request.content, config);
					break;
				case 'anthropic':
					if (!config.anthropicApiKey) {
						throw new Error('Anthropic API key not configured.');
					}
					title = await generateTitleWithAnthropic(request.content, config);
					break;
				default:
					throw new Error(`Unknown LLM provider: ${config.provider}`);
			}

			return { title };
		} catch (error) {
			if (error instanceof Error) {
				throw error;
			}
			throw new Error('Failed to generate title');
		}
	},

	optimizeDescription: async (request: { content: string }) => {
		const config = llmSettings.config;

		if (config.provider === 'disabled') {
			throw new Error('LLM provider is disabled. Please configure an LLM provider in settings.');
		}

		let optimized = '';

		try {
			switch (config.provider) {
				case 'ollama':
					optimized = await optimizeDescriptionWithOllama(request.content, config);
					break;
				case 'openai':
					if (!config.openaiApiKey) {
						throw new Error('OpenAI API key not configured.');
					}
					optimized = await optimizeDescriptionWithOpenAI(request.content, config);
					break;
				case 'anthropic':
					if (!config.anthropicApiKey) {
						throw new Error('Anthropic API key not configured.');
					}
					optimized = await optimizeDescriptionWithAnthropic(request.content, config);
					break;
				default:
					throw new Error(`Unknown LLM provider: ${config.provider}`);
			}

			return { optimized };
		} catch (error) {
			if (error instanceof Error) {
				throw error;
			}
			throw new Error('Failed to optimize description');
		}
	}
};
