<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import { dreamsApi } from '$lib/api/dreams';
	import { llmApi } from '$lib/api/llm';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';
	import type { CreateDreamInput } from '$lib/types/dream';
	import { RECURRENCE_TIME_PERIODS } from '$lib/types/dream';
	import Button from '$lib/components/ui/Button.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import { ArrowLeft, Save, Sparkles } from 'lucide-svelte';

	let title = $state('');
	let content = $state('');
	let dateOccurred = $state(new Date().toISOString().split('T')[0]);
	let sleepQuality = $state<number | undefined>(undefined);
	let isRecurring = $state(false);
	let lastOccurrencePeriod = $state('');
	let isLucid = $state(false);
	let saving = $state(false);
	let error = $state('');
	let generatingTitle = $state(false);
	let titleError = $state('');
	let optimizingDescription = $state(false);
	let optimizeError = $state('');

	async function handleGenerateTitle() {
		if (!content.trim()) {
			titleError = 'Please write your dream content first';
			return;
		}

		try {
			generatingTitle = true;
			titleError = '';

			const response = await llmApi.generateTitle({ content: content.trim() });
			title = response.title;
		} catch (err) {
			console.error('Failed to generate title:', err);
			titleError = err instanceof Error ? err.message : 'Failed to generate title';
		} finally {
			generatingTitle = false;
		}
	}

	async function handleOptimizeDescription() {
		if (!content.trim()) {
			optimizeError = 'Please write your dream content first';
			return;
		}

		try {
			optimizingDescription = true;
			optimizeError = '';

			const response = await llmApi.optimizeDescription({ content: content.trim() });
			content = response.optimized;
		} catch (err) {
			console.error('Failed to optimize description:', err);
			optimizeError = err instanceof Error ? err.message : 'Failed to optimize description';
		} finally {
			optimizingDescription = false;
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!title.trim() || !content.trim()) {
			error = 'Title and content are required';
			return;
		}

		try {
			saving = true;
			error = '';

			const input: CreateDreamInput = {
				title: title.trim(),
				content: content.trim(),
				date_occurred: new Date(dateOccurred).toISOString(),
				sleep_quality: sleepQuality,
				is_recurring: isRecurring,
				last_occurrence_period: isRecurring && lastOccurrencePeriod ? (lastOccurrencePeriod as any) : undefined,
				is_lucid: isLucid
			};

			const createdDream = await dreamsApi.create(input);

			// Redirect to the dream detail page where auto-analysis will be triggered
			if (createdDream.id) {
				window.location.href = `/dreams/${createdDream.id}`;
			} else {
				window.location.href = '/dreams';
			}
		} catch (err) {
			console.error('Failed to create dream:', err);
			error = 'Failed to save dream. Please try again.';
		} finally {
			saving = false;
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

	const backButtonStyles = css({
		display: 'inline-flex',
		alignItems: 'center',
		gap: '0.5rem',
		color: 'text.secondary',
		fontSize: 'sm',
		marginBottom: '1rem',
		cursor: 'pointer',
		'&:hover': {
			color: 'text.primary'
		}
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary'
	});

	const formContainerStyles = css({
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

	const formGroupStyles = css({
		marginBottom: '1.5rem'
	});

	const labelStyles = css({
		display: 'block',
		fontSize: 'sm',
		fontWeight: 'semibold',
		color: 'text.primary',
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
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const textareaStyles = css({
		width: '100%',
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		fontSize: 'md',
		minHeight: '200px',
		fontFamily: 'inherit',
		resize: 'vertical',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const qualityContainerStyles = css({
		display: 'flex',
		gap: '0.5rem'
	});

	const qualityButtonStyles = css({
		padding: '0.5rem 1rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		cursor: 'pointer',
		transition: 'all 0.2s',
		'&:hover': {
			borderColor: 'border.active'
		}
	});

	const qualityButtonActiveStyles = css({
		backgroundColor: 'breakthrough.500',
		color: 'white',
		borderColor: 'breakthrough.500',
		'&:hover': {
			borderColor: 'breakthrough.600',
			backgroundColor: 'breakthrough.600'
		}
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '1rem',
		justifyContent: 'flex-end'
	});

	const errorStyles = css({
		padding: '1rem',
		backgroundColor: 'void.800',
		color: 'void.100',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'void.700',
		marginBottom: '1rem',
		fontSize: 'sm'
	});

	const helpTextStyles = css({
		fontSize: 'xs',
		color: 'text.muted',
		marginTop: '0.25rem'
	});

	const titleFieldContainerStyles = css({
		display: 'flex',
		gap: '0.5rem',
		alignItems: 'flex-start'
	});

	const titleInputWrapperStyles = css({
		flex: '1'
	});

	const titleErrorStyles = css({
		fontSize: 'xs',
		color: 'void.100',
		marginTop: '0.25rem'
	});

	const checkboxContainerStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.75rem'
	});

	const checkboxStyles = css({
		width: '1.25rem',
		height: '1.25rem',
		cursor: 'pointer',
		accentColor: 'breakthrough.500'
	});

	const checkboxLabelStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.75rem',
		cursor: 'pointer',
		fontSize: 'sm',
		fontWeight: 'semibold',
		color: 'text.primary'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/dreams" class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Dreams
		</a>
		<h1 class={titleStyles}>Record a New Dream</h1>
	</div>

	<div class={formContainerStyles}>
		{#if error}
			<div class={errorStyles}>{error}</div>
		{/if}

		<form onsubmit={handleSubmit}>
			<div class={formGroupStyles}>
				<label for="date" class={labelStyles}>Dream Date</label>
				<input type="date" id="date" class={inputStyles} bind:value={dateOccurred} required />
				<p class={helpTextStyles}>When did you have this dream?</p>
			</div>

			<div class={formGroupStyles}>
				<div class={titleFieldContainerStyles}>
					<label for="content" class={labelStyles}>Dream Description</label>
					<Button
						variant="outline"
						type="button"
						onclick={handleOptimizeDescription}
						disabled={optimizingDescription || !llmSettings.isConfigured || !content.trim()}
						title={!llmSettings.isConfigured
							? 'Configure LLM in settings first'
							: !content.trim()
								? 'Write dream content first'
								: 'Optimize description for clarity and analysis'}
					>
						<Sparkles size={20} />
						{optimizingDescription ? 'Optimizing...' : 'Optimize'}
					</Button>
				</div>
				<textarea
					id="content"
					class={textareaStyles}
					bind:value={content}
					placeholder="Describe your dream in as much detail as you remember..."
					required
				></textarea>
				{#if optimizeError}
					<p class={titleErrorStyles}>{optimizeError}</p>
				{/if}
				<p class={helpTextStyles}>
					Capture as many details as you can remember - symbols, feelings, people, places.
				</p>
			</div>

			<div class={formGroupStyles}>
				<label for="title" class={labelStyles}>Title</label>
				<div class={titleFieldContainerStyles}>
					<div class={titleInputWrapperStyles}>
						<input
							type="text"
							id="title"
							class={inputStyles}
							bind:value={title}
							placeholder="Give your dream a title..."
							required
						/>
						{#if titleError}
							<p class={titleErrorStyles}>{titleError}</p>
						{/if}
					</div>
					<Button
						variant="outline"
						type="button"
						onclick={handleGenerateTitle}
						disabled={generatingTitle || !llmSettings.isConfigured || !content.trim()}
						title={!llmSettings.isConfigured
							? 'Configure LLM in settings first'
							: !content.trim()
								? 'Write dream content first'
								: 'Generate title from dream content'}
					>
						<Sparkles size={20} />
						{generatingTitle ? 'Generating...' : 'Generate'}
					</Button>
				</div>
			</div>

			<div class={formGroupStyles}>
				<label for="sleep-quality" class={labelStyles}>Sleep Quality</label>
				<div id="sleep-quality" role="group" class={qualityContainerStyles}>
					{#each [1, 2, 3, 4, 5] as quality}
						<button
							type="button"
							class={`${qualityButtonStyles} ${sleepQuality === quality ? qualityButtonActiveStyles : ''}`}
							onclick={() => (sleepQuality = sleepQuality === quality ? undefined : quality)}
						>
							{quality}
						</button>
					{/each}
				</div>
				<p class={helpTextStyles}>Rate your sleep quality (1 = Poor, 5 = Excellent)</p>
			</div>

			<div class={formGroupStyles}>
				<label class={checkboxLabelStyles}>
					<input
						type="checkbox"
						class={checkboxStyles}
						bind:checked={isRecurring}
					/>
					This is a recurring dream
				</label>
				<p class={helpTextStyles}>Have you had this dream before?</p>
			</div>

			{#if isRecurring}
				<div class={formGroupStyles}>
					<label for="last-occurrence" class={labelStyles}>When was the last time you had this dream?</label>
					<Select
						id="last-occurrence"
						bind:value={lastOccurrencePeriod}
						options={[
							{ value: '', label: 'Select a time period' },
							...RECURRENCE_TIME_PERIODS
						]}
					/>
					<p class={helpTextStyles}>Approximately when did you last have this dream?</p>
				</div>
			{/if}

			<div class={formGroupStyles}>
				<label class={checkboxLabelStyles}>
					<input
						type="checkbox"
						class={checkboxStyles}
						bind:checked={isLucid}
					/>
					This was a lucid dream
				</label>
				<p class={helpTextStyles}>Were you aware that you were dreaming?</p>
			</div>

			<div class={buttonGroupStyles}>
				<Button variant="outline" type="button" onclick={() => (window.location.href = '/dreams')}>
					Cancel
				</Button>
				<Button variant="primary" type="submit" disabled={saving}>
					<Save size={20} />
					{saving ? 'Saving...' : 'Save Dream'}
				</Button>
			</div>
		</form>
	</div>
</div>
