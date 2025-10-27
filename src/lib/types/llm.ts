export type LLMProvider = 'disabled' | 'ollama' | 'openai' | 'anthropic';

export interface LLMConfig {
	provider: LLMProvider;
	ollamaUrl: string; // Default: http://localhost:11434
	ollamaModel: string; // Default: llama3.2
	openaiApiKey: string;
	openaiModel: string; // Default: gpt-4o-mini
	anthropicApiKey: string;
	anthropicModel: string; // Default: claude-3-5-haiku-20241022
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
	ollamaModel: 'llama3.2',
	openaiApiKey: '',
	openaiModel: 'gpt-4o-mini',
	anthropicApiKey: '',
	anthropicModel: 'claude-3-5-haiku-20241022'
};
