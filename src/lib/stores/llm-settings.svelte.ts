import { browser } from '$app/environment';
import type { LLMConfig } from '$lib/types/llm';
import { DEFAULT_LLM_CONFIG } from '$lib/types/llm';

const STORAGE_KEY = 'limnl-llm-config';

function loadConfig(): LLMConfig {
	if (!browser) return DEFAULT_LLM_CONFIG;

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			return { ...DEFAULT_LLM_CONFIG, ...JSON.parse(stored) };
		}
	} catch (error) {
		console.error('Failed to load LLM config:', error);
	}

	return DEFAULT_LLM_CONFIG;
}

function saveConfig(config: LLMConfig): void {
	if (!browser) return;

	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
	} catch (error) {
		console.error('Failed to save LLM config:', error);
	}
}

class LLMSettingsStore {
	config = $state<LLMConfig>(loadConfig());

	updateConfig(updates: Partial<LLMConfig>) {
		this.config = { ...this.config, ...updates };
		saveConfig(this.config);
	}

	resetConfig() {
		this.config = DEFAULT_LLM_CONFIG;
		saveConfig(this.config);
	}

	get isConfigured(): boolean {
		if (this.config.provider === 'disabled') return false;
		if (this.config.provider === 'ollama') return true; // Assumes local Ollama is running
		if (this.config.provider === 'openai') return this.config.openaiApiKey.length > 0;
		if (this.config.provider === 'anthropic') return this.config.anthropicApiKey.length > 0;
		return false;
	}
}

export const llmSettings = new LLMSettingsStore();
