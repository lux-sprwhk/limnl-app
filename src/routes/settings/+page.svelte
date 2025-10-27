<script lang="ts">
	import { css } from '../../../styled-system/css';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';
	import type { LLMProvider } from '$lib/types/llm';
	import Button from '$lib/components/ui/Button.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import { Save } from 'lucide-svelte';

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
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const headerStyles = css({
		maxWidth: '800px',
		margin: '0 auto 2rem'
	});


	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary'
	});

	const cardStyles = css({
		maxWidth: '800px',
		margin: '0 auto',
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const sectionStyles = css({
		marginBottom: '2rem'
	});

	const sectionTitleStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '1rem'
	});

	const formGroupStyles = css({
		marginBottom: '1.5rem'
	});

	const labelStyles = css({
		display: 'block',
		fontSize: 'sm',
		fontWeight: 'semibold',
		color: 'text.secondary',
		marginBottom: '0.5rem'
	});

	const inputStyles = css({
		width: '100%',
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		fontSize: 'md',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&:disabled': {
			backgroundColor: 'void.800',
			color: 'text.muted',
			cursor: 'not-allowed'
		},
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const helpTextStyles = css({
		fontSize: 'xs',
		color: 'text.muted',
		marginTop: '0.25rem'
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '1rem',
		justifyContent: 'flex-end'
	});

	const successStyles = css({
		padding: '1rem',
		backgroundColor: 'void.800',
		color: 'breakthrough.300',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'breakthrough.600',
		marginBottom: '1rem',
		fontSize: 'sm'
	});

	const infoBoxStyles = css({
		padding: '1rem',
		backgroundColor: 'void.800',
		color: 'text.secondary',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		fontSize: 'sm',
		marginBottom: '1.5rem'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
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
				<Select
					id="provider"
					bind:value={config.provider}
					options={[
						{ value: 'disabled', label: 'Disabled' },
						{ value: 'ollama', label: 'Ollama (Local)' },
						{ value: 'openai', label: 'OpenAI' },
						{ value: 'anthropic', label: 'Anthropic (Claude)' }
					]}
				/>
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
					<Select
						id="ollama-model"
						bind:value={config.ollamaModel}
						options={[
							{ value: 'llama', label: 'Llama' },
							{ value: 'mistral', label: 'Mistral' },
							{ value: 'phi', label: 'Phi' },
							{ value: 'deepseek', label: 'Deepseek' }
						]}
					/>
					<p class={helpTextStyles}>
						Select from popular Ollama models
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
							class={css({ color: 'breakthrough.400', textDecoration: 'underline', '&:hover': { color: 'breakthrough.300' } })}>platform.openai.com</a
						>
					</p>
				</div>

				<div class={formGroupStyles}>
					<label for="openai-model" class={labelStyles}>Model</label>
					<Select
						id="openai-model"
						bind:value={config.openaiModel}
						options={[
							{ value: 'gpt4-mini', label: 'GPT-4 Mini' },
							{ value: 'gpt4-turbo', label: 'GPT-4 Turbo' },
							{ value: 'gpt4', label: 'GPT-4' },
							{ value: 'gpt4o', label: 'GPT-4o' }
						]}
					/>
					<p class={helpTextStyles}>Select your preferred OpenAI model</p>
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
							class={css({ color: 'breakthrough.400', textDecoration: 'underline', '&:hover': { color: 'breakthrough.300' } })}>console.anthropic.com</a
						>
					</p>
				</div>

				<div class={formGroupStyles}>
					<label for="anthropic-model" class={labelStyles}>Model</label>
					<Select
						id="anthropic-model"
						bind:value={config.anthropicModel}
						options={[
							{ value: 'claude-haiku', label: 'Haiku (latest)' },
							{ value: 'claude-sonnet', label: 'Sonnet (latest)' }
						]}
					/>
					<p class={helpTextStyles}>Choose between Haiku (fast and affordable) or Sonnet (more capable)</p>
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
