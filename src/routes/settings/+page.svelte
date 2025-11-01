<script lang="ts">
	import { css } from '../../../styled-system/css';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';
	import { userProfile } from '$lib/stores/user-profile.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import type { LLMProvider } from '$lib/types/llm';
	import { ZODIAC_SIGNS, MBTI_TYPES } from '$lib/types/user';
	import Button from '$lib/components/ui/Button.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import { Save, Lock, Unlock, User, Bot, Database } from 'lucide-svelte';
	import { Tabs } from 'bits-ui';
	import { databaseApi } from '$lib/api/database';

	let config = $state({ ...llmSettings.config });
	let profile = $state({ ...userProfile.profile });
	let saved = $state(false);
	let pinEnabled = $state(authStore.authState.requirePin);
	let showPinSection = $state(false);
	let currentPin = $state('');
	let newPin = $state('');
	let confirmPin = $state('');
	let pinError = $state('');
	let pinSuccess = $state('');
	let backupInProgress = $state(false);
	let backupSuccess = $state('');
	let backupError = $state('');

	// Dynamic styles for toggle button
	const getToggleButtonStyles = (enabled: boolean) => css({
		width: '3rem',
		height: '1.75rem',
		borderRadius: 'full',
		backgroundColor: enabled ? 'breakthrough.600' : 'void.700',
		border: '2px solid',
		borderColor: enabled ? 'breakthrough.500' : 'border.liminal',
		position: 'relative',
		cursor: 'pointer',
		transition: 'all 0.2s',
		'&:hover': {
			backgroundColor: enabled ? 'breakthrough.500' : 'void.600'
		}
	});

	const getToggleKnobStyles = (enabled: boolean) => css({
		position: 'absolute',
		width: '1.25rem',
		height: '1.25rem',
		borderRadius: 'full',
		backgroundColor: 'white',
		top: '0.125rem',
		left: enabled ? 'calc(100% - 1.375rem)' : '0.125rem',
		transition: 'left 0.2s'
	});

	function handleSave() {
		llmSettings.updateConfig(config);
		userProfile.updateProfile(profile);
		saved = true;
		setTimeout(() => (saved = false), 2000);
	}

	function handleReset() {
		llmSettings.resetConfig();
		userProfile.resetProfile();
		config = { ...llmSettings.config };
		profile = { ...userProfile.profile };
	}

	async function handleTogglePin() {
		pinError = '';
		pinSuccess = '';

		const newState = !pinEnabled;

		if (!newState && authStore.authState.hasPin) {
			// Disabling PIN - need to verify current PIN
			if (!currentPin) {
				pinError = 'Please enter your current PIN to disable';
				return;
			}

			const success = await authStore.togglePinRequirement(false, currentPin);
			if (!success) {
				pinError = 'Incorrect PIN';
				return;
			}

			pinSuccess = 'PIN authentication disabled';
			pinEnabled = false;
			currentPin = '';
		} else {
			// Enabling PIN
			const success = await authStore.togglePinRequirement(true);
			if (success) {
				pinEnabled = true;
				pinSuccess = 'PIN authentication enabled. You can set up your PIN on the home page.';
			} else {
				pinError = 'Failed to enable PIN authentication';
			}
		}

		setTimeout(() => {
			pinError = '';
			pinSuccess = '';
		}, 3000);
	}

	async function handleChangePin() {
		pinError = '';
		pinSuccess = '';

		if (!currentPin || !newPin || !confirmPin) {
			pinError = 'Please fill in all PIN fields';
			return;
		}

		if (newPin !== confirmPin) {
			pinError = 'New PINs do not match';
			return;
		}

		if (newPin.length < 4) {
			pinError = 'PIN must be at least 4 characters';
			return;
		}

		const success = await authStore.changePin(currentPin, newPin);
		if (success) {
			pinSuccess = 'PIN changed successfully';
			currentPin = '';
			newPin = '';
			confirmPin = '';
			showPinSection = false;
		} else {
			pinError = 'Incorrect current PIN';
		}

		setTimeout(() => {
			pinError = '';
			pinSuccess = '';
		}, 3000);
	}

	async function handleBackupDatabase() {
		backupInProgress = true;
		backupError = '';
		backupSuccess = '';

		try {
			const success = await databaseApi.backupDatabaseWithDialog();
			if (success) {
				backupSuccess = 'Database backed up successfully!';
				setTimeout(() => {
					backupSuccess = '';
				}, 3000);
			}
		} catch (error) {
			backupError = `Failed to backup database: ${error}`;
			setTimeout(() => {
				backupError = '';
			}, 5000);
		} finally {
			backupInProgress = false;
		}
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

	const tabsListStyles = css({
		display: 'flex',
		gap: '0.5rem',
		marginBottom: '2rem',
		borderBottom: '1px solid',
		borderColor: 'border.liminal'
	});

	const tabTriggerStyles = css({
		padding: '0.75rem 1.5rem',
		fontSize: 'md',
		fontWeight: 'semibold',
		color: 'text.secondary',
		backgroundColor: 'transparent',
		border: 'none',
		borderBottom: '2px solid transparent',
		cursor: 'pointer',
		transition: 'all 0.2s',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem',
		'&:hover': {
			color: 'text.primary',
			backgroundColor: 'void.800'
		},
		'&[data-state="active"]': {
			color: 'breakthrough.400',
			borderBottomColor: 'breakthrough.500'
		}
	});

	const tabContentStyles = css({
		outline: 'none'
	});

	const linkStyles = css({
		color: 'breakthrough.400',
		textDecoration: 'underline',
		'&:hover': {
			color: 'breakthrough.300'
		}
	});

	const errorStyles = css({
		padding: '1rem',
		backgroundColor: 'void.800',
		color: 'error.300',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'error.600',
		marginBottom: '1rem',
		fontSize: 'sm'
	});

	const pinToggleContainerStyles = css({
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'space-between',
		marginBottom: '0.5rem'
	});

	const toggleButtonStyles = css({
		width: '3rem',
		height: '1.75rem',
		borderRadius: 'full',
		border: '2px solid',
		position: 'relative',
		cursor: 'pointer',
		transition: 'all 0.2s'
	});

	const toggleKnobStyles = css({
		position: 'absolute',
		width: '1.25rem',
		height: '1.25rem',
		borderRadius: 'full',
		backgroundColor: 'white',
		top: '0.125rem',
		transition: 'left 0.2s'
	});

	const pinChangeSectionStyles = css({
		padding: '1.5rem',
		backgroundColor: 'void.800',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		marginTop: '1rem'
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

		<Tabs.Root value="profile">
			<Tabs.List class={tabsListStyles}>
				<Tabs.Trigger value="profile" class={tabTriggerStyles}>
					<User size={18} />
					Profile
				</Tabs.Trigger>
				<Tabs.Trigger value="llm" class={tabTriggerStyles}>
					<Bot size={18} />
					LLM Config
				</Tabs.Trigger>
				<Tabs.Trigger value="data" class={tabTriggerStyles}>
					<Database size={18} />
					Data & Export
				</Tabs.Trigger>
			</Tabs.List>

			<Tabs.Content value="profile" class={tabContentStyles}>
				<div class={sectionStyles}>
					<h2 class={sectionTitleStyles}>User Profile</h2>

					<div class={formGroupStyles}>
						<label for="name" class={labelStyles}>Name</label>
						<input
							type="text"
							id="name"
							class={inputStyles}
							bind:value={profile.name}
							placeholder="Your name"
						/>
						<p class={helpTextStyles}>How would you like to be addressed in your dream journal?</p>
					</div>

					<div class={formGroupStyles}>
						<label for="zodiac" class={labelStyles}>Zodiac Sign (Optional)</label>
						<Select
							id="zodiac"
							bind:value={profile.zodiacSign}
							options={[
								{ value: '', label: 'Not specified' },
								...ZODIAC_SIGNS
							]}
						/>
						<p class={helpTextStyles}>Your zodiac sign for enhanced dream insights</p>
					</div>

					<div class={formGroupStyles}>
						<label for="mbti" class={labelStyles}>Myers-Briggs Type (Optional)</label>
						<Select
							id="mbti"
							bind:value={profile.mbtiType}
							options={[
								{ value: '', label: 'Not specified' },
								...MBTI_TYPES
							]}
						/>
						<p class={helpTextStyles}>Your MBTI personality type for personalized dream analysis</p>
					</div>
				</div>

				<div class={sectionStyles}>
					<h2 class={sectionTitleStyles}>Security</h2>

			{#if pinError}
				<div class={errorStyles}>
					{pinError}
				</div>
			{/if}

			{#if pinSuccess}
				<div class={successStyles}>
					{pinSuccess}
				</div>
			{/if}

			<div class={formGroupStyles}>
				<div class={pinToggleContainerStyles}>
					<label for="pin-enabled" class={labelStyles} style="margin-bottom: 0;">
						{#if pinEnabled}
							<Lock size={16} style="display: inline; vertical-align: text-bottom;" />
						{:else}
							<Unlock size={16} style="display: inline; vertical-align: text-bottom;" />
						{/if}
						Require PIN on Launch
					</label>
					<button
						type="button"
						role="switch"
						aria-checked={pinEnabled}
						aria-label="Toggle PIN requirement"
						class={getToggleButtonStyles(pinEnabled)}
						onclick={handleTogglePin}
					>
						<span
							class={getToggleKnobStyles(pinEnabled)}
						></span>
					</button>
				</div>
				<p class={helpTextStyles}>
					When enabled, you'll need to enter a PIN to access your journal. You can bypass this by toggling it off.
				</p>
			</div>

			{#if pinEnabled && authStore.authState.hasPin}
				<div class={formGroupStyles}>
					<Button
						variant="outline"
						type="button"
						onclick={() => showPinSection = !showPinSection}
					>
						{showPinSection ? 'Cancel' : 'Change PIN'}
					</Button>
				</div>

				{#if showPinSection}
					<div class={pinChangeSectionStyles}>
						<div class={formGroupStyles}>
							<label for="current-pin" class={labelStyles}>Current PIN</label>
							<input
								type="password"
								id="current-pin"
								class={inputStyles}
								bind:value={currentPin}
								placeholder="Enter current PIN"
							/>
						</div>

						<div class={formGroupStyles}>
							<label for="new-pin" class={labelStyles}>New PIN</label>
							<input
								type="password"
								id="new-pin"
								class={inputStyles}
								bind:value={newPin}
								placeholder="Enter new PIN (min 4 characters)"
							/>
						</div>

						<div class={formGroupStyles}>
							<label for="confirm-pin" class={labelStyles}>Confirm New PIN</label>
							<input
								type="password"
								id="confirm-pin"
								class={inputStyles}
								bind:value={confirmPin}
								placeholder="Re-enter new PIN"
							/>
						</div>

						<Button variant="primary" type="button" onclick={handleChangePin}>
							Update PIN
						</Button>
					</div>
				{/if}
			{/if}

			{#if !pinEnabled && authStore.authState.hasPin}
				<div class={formGroupStyles}>
					<label for="disable-pin" class={labelStyles}>Enter PIN to Disable</label>
					<input
						type="password"
						id="disable-pin"
						class={inputStyles}
						bind:value={currentPin}
						placeholder="Enter your PIN"
					/>
					<p class={helpTextStyles}>
						Enter your current PIN to confirm disabling PIN authentication
					</p>
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
			</Tabs.Content>

			<Tabs.Content value="llm" class={tabContentStyles}>
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
									class={linkStyles}>platform.openai.com</a
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
									class={linkStyles}>console.anthropic.com</a
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
			</Tabs.Content>

			<Tabs.Content value="data" class={tabContentStyles}>
				<div class={sectionStyles}>
					<h2 class={sectionTitleStyles}>Database Backup</h2>

					<div class={infoBoxStyles}>
						Export a complete backup of your Limnl database. This backup includes all your dreams, bugs, mind dumps, and other data. Store it in a safe place to restore your journal if needed.
					</div>

					{#if backupError}
						<div class={errorStyles}>
							{backupError}
						</div>
					{/if}

					{#if backupSuccess}
						<div class={successStyles}>
							{backupSuccess}
						</div>
					{/if}

					<div class={formGroupStyles}>
						<Button
							variant="primary"
							type="button"
							onclick={handleBackupDatabase}
							disabled={backupInProgress}
						>
							<Database size={20} />
							{backupInProgress ? 'Backing up...' : 'Backup Database'}
						</Button>
						<p class={helpTextStyles}>
							Downloads a copy of your entire database file (dreams.db)
						</p>
					</div>
				</div>
			</Tabs.Content>
		</Tabs.Root>
	</div>
</div>
