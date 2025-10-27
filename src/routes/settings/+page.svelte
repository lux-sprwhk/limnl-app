<script lang="ts">
	import { css } from '../../../styled-system/css';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';
	import type { LLMProvider } from '$lib/types/llm';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Save } from 'lucide-svelte';

	let config = $state({ ...llmSettings.config });
	let saved = $state(false);

	function handleSave() {
		llmSettings.updateConfig(config);
		saved = true;
		setTimeout(() => (saved = false), 2000);
	}

	function handleReset() {
		llmSettings.resetConfig();
		config = { ...llmSettings.config };
	}

	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'gray.50',
		padding: '2rem'
	});

	const headerStyles = css({
		maxWidth: '800px',
		margin: '0 auto 2rem'
	});

	const backButtonStyles = css({
		display: 'inline-flex',
		alignItems: 'center',
		gap: '0.5rem',
		color: 'gray.600',
		fontSize: 'sm',
		marginBottom: '1rem',
		cursor: 'pointer',
		'&:hover': {
			color: 'gray.900'
		}
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'gray.900'
	});

	const cardStyles = css({
		maxWidth: '800px',
		margin: '0 auto',
		backgroundColor: 'white',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'sm',
		border: '1px solid',
		borderColor: 'gray.200'
	});

	const sectionStyles = css({
		marginBottom: '2rem'
	});

	const sectionTitleStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'gray.900',
		marginBottom: '1rem'
	});

	const formGroupStyles = css({
		marginBottom: '1.5rem'
	});

	const labelStyles = css({
		display: 'block',
		fontSize: 'sm',
		fontWeight: 'semibold',
		color: 'gray.700',
		marginBottom: '0.5rem'
	});

	const inputStyles = css({
		width: '100%',
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'gray.300',
		fontSize: 'md',
		'&:focus': {
			outline: 'none',
			borderColor: 'blue.500',
			boxShadow: '0 0 0 3px rgba(59, 130, 246, 0.1)'
		},
		'&:disabled': {
			backgroundColor: 'gray.100',
			cursor: 'not-allowed'
		}
	});

	const selectStyles = css({
		width: '100%',
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'gray.300',
		fontSize: 'md',
		backgroundColor: 'white',
		cursor: 'pointer',
		'&:focus': {
			outline: 'none',
			borderColor: 'blue.500',
			boxShadow: '0 0 0 3px rgba(59, 130, 246, 0.1)'
		}
	});

	const helpTextStyles = css({
		fontSize: 'xs',
		color: 'gray.500',
		marginTop: '0.25rem'
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '1rem',
		justifyContent: 'flex-end'
	});

	const successStyles = css({
		padding: '1rem',
		backgroundColor: 'green.50',
		color: 'green.700',
		borderRadius: 'md',
		marginBottom: '1rem',
		fontSize: 'sm'
	});

	const infoBoxStyles = css({
		padding: '1rem',
		backgroundColor: 'blue.50',
		color: 'blue.700',
		borderRadius: 'md',
		fontSize: 'sm',
		marginBottom: '1.5rem'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/" class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Home
		</a>
		<h1 class={titleStyles}>Settings</h1>
	</div>

	<div class={cardStyles}>
		{#if saved}
			<div class={successStyles}>Settings saved successfully!</div>
		{/if}

		<div class={sectionStyles}>
			<h2 class={sectionTitleStyles}>LLM Configuration</h2>

			<div class={infoBoxStyles}>
				Configure an LLM provider to enable AI features like automatic dream title generation. Your
				API keys are stored locally and never leave your device.
			</div>

			<div class={formGroupStyles}>
				<label for="provider" class={labelStyles}>Provider</label>
				<select id="provider" class={selectStyles} bind:value={config.provider}>
					<option value="disabled">Disabled</option>
					<option value="ollama">Ollama (Local)</option>
					<option value="openai">OpenAI</option>
					<option value="anthropic">Anthropic (Claude)</option>
				</select>
				<p class={helpTextStyles}>Choose your preferred LLM provider</p>
			</div>

			{#if config.provider === 'ollama'}
				<div class={formGroupStyles}>
					<label for="ollama-url" class={labelStyles}>Ollama URL</label>
					<input
						type="text"
						id="ollama-url"
						class={inputStyles}
						bind:value={config.ollamaUrl}
						placeholder="http://localhost:11434"
					/>
					<p class={helpTextStyles}>
						The URL where Ollama is running. Make sure Ollama is installed and running.
					</p>
				</div>

				<div class={formGroupStyles}>
					<label for="ollama-model" class={labelStyles}>Model</label>
					<input
						type="text"
						id="ollama-model"
						class={inputStyles}
						bind:value={config.ollamaModel}
						placeholder="llama3.2"
					/>
					<p class={helpTextStyles}>
						The Ollama model to use. Popular choices: llama3.2, mistral, phi3
					</p>
				</div>
			{/if}

			{#if config.provider === 'openai'}
				<div class={formGroupStyles}>
					<label for="openai-key" class={labelStyles}>OpenAI API Key</label>
					<input
						type="password"
						id="openai-key"
						class={inputStyles}
						bind:value={config.openaiApiKey}
						placeholder="sk-..."
					/>
					<p class={helpTextStyles}>
						Get your API key from <a
							href="https://platform.openai.com/api-keys"
							target="_blank"
							class={css({ color: 'blue.600', textDecoration: 'underline' })}>platform.openai.com</a
						>
					</p>
				</div>

				<div class={formGroupStyles}>
					<label for="openai-model" class={labelStyles}>Model</label>
					<input
						type="text"
						id="openai-model"
						class={inputStyles}
						bind:value={config.openaiModel}
						placeholder="gpt-4o-mini"
					/>
					<p class={helpTextStyles}>Default: gpt-4o-mini (fast and affordable)</p>
				</div>
			{/if}

			{#if config.provider === 'anthropic'}
				<div class={formGroupStyles}>
					<label for="anthropic-key" class={labelStyles}>Anthropic API Key</label>
					<input
						type="password"
						id="anthropic-key"
						class={inputStyles}
						bind:value={config.anthropicApiKey}
						placeholder="sk-ant-..."
					/>
					<p class={helpTextStyles}>
						Get your API key from <a
							href="https://console.anthropic.com/settings/keys"
							target="_blank"
							class={css({ color: 'blue.600', textDecoration: 'underline' })}>console.anthropic.com</a
						>
					</p>
				</div>

				<div class={formGroupStyles}>
					<label for="anthropic-model" class={labelStyles}>Model</label>
					<input
						type="text"
						id="anthropic-model"
						class={inputStyles}
						bind:value={config.anthropicModel}
						placeholder="claude-3-5-haiku-20241022"
					/>
					<p class={helpTextStyles}>Default: claude-3-5-haiku-20241022 (fast and affordable)</p>
				</div>
			{/if}
		</div>

		<div class={buttonGroupStyles}>
			<Button variant="outline" type="button" onclick={handleReset}> Reset to Defaults </Button>
			<Button variant="primary" type="button" onclick={handleSave}>
				<Save size={20} />
				Save Settings
			</Button>
		</div>
	</div>
</div>
