export type LLMProvider = 'disabled' | 'ollama' | 'openai' | 'anthropic';

export interface LLMConfig {
	provider: LLMProvider;
	ollamaUrl: string; // Default: http://localhost:11434
	ollamaModel: string; // Default: llama
	openaiApiKey: string;
	openaiModel: string; // Default: gpt4-mini
	anthropicApiKey: string;
	anthropicModel: string; // Default: claude-haiku
}

export interface GenerateTitleRequest {
	content: string;
}

export interface GenerateTitleResponse {
	title: string;
}

export const DEFAULT_LLM_CONFIG: LLMConfig = {
	provider: 'disabled',
	ollamaUrl: 'http://localhost:11434',
	ollamaModel: 'llama',
	openaiApiKey: '',
	openaiModel: 'gpt4-mini',
	anthropicApiKey: '',
	anthropicModel: 'claude-haiku'
};
