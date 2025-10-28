<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import { journalApi } from '$lib/api/journal';
	import type { CreateJournalInput } from '$lib/types/journal';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Save } from 'lucide-svelte';

	let title = $state('');
	let content = $state('');
	let saving = $state(false);
	let error = $state('');

	// Derived word count
	let wordCount = $derived.by(() => {
		if (!content.trim()) return 0;
		return content.trim().split(/\s+/).filter(word => word.length > 0).length;
	});

	// Progress toward 750 words
	let progress = $derived.by(() => {
		return Math.min((wordCount / 750) * 100, 100);
	});

	// Progress color based on word count
	let progressColor = $derived.by(() => {
		if (wordCount < 250) return 'void.500';
		if (wordCount < 500) return 'liminal.500';
		if (wordCount < 750) return 'breakthrough.400';
		return 'breakthrough.500';
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!content.trim()) {
			error = 'Content is required';
			return;
		}

		try {
			saving = true;
			error = '';

			const input: CreateJournalInput = {
				title: title.trim() || undefined,
				content: content.trim(),
				word_count: wordCount
			};

			await journalApi.create(input);
			window.location.href = '/journal';
		} catch (err) {
			console.error('Failed to create journal entry:', err);
			error = 'Failed to save entry. Please try again.';
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
		maxWidth: '900px',
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
		maxWidth: '900px',
		margin: '0 auto',
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const wordCountContainerStyles = css({
		display: 'flex',
		justifyContent: 'space-between',
		alignItems: 'center',
		marginBottom: '1rem',
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const wordCountTextStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		color: 'text.primary'
	});

	const progressBarContainerStyles = css({
		width: '100%',
		height: '8px',
		backgroundColor: 'void.800',
		borderRadius: 'full',
		overflow: 'hidden',
		marginTop: '0.5rem'
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
		minHeight: '400px',
		fontFamily: 'inherit',
		resize: 'vertical',
		lineHeight: '1.6',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&::placeholder': {
			color: 'text.muted'
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

	const milestoneTextStyles = css({
		fontSize: 'sm',
		color: 'text.muted'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/journal" class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Journal
		</a>
		<h1 class={titleStyles}>New Journal Entry</h1>
	</div>

	<div class={formContainerStyles}>
		{#if error}
			<div class={errorStyles}>{error}</div>
		{/if}

		<div class={wordCountContainerStyles}>
			<div>
				<div class={wordCountTextStyles}>
					{wordCount} / 750 words
				</div>
				<div class={milestoneTextStyles}>
					{#if wordCount === 0}
						Start writing to track your progress
					{:else if wordCount < 250}
						Keep going! You're just getting started.
					{:else if wordCount < 500}
						Nice work! You're building momentum.
					{:else if wordCount < 750}
						Excellent! Almost at 750 words.
					{:else}
						Great job! You've reached 750 words!
					{/if}
				</div>
			</div>
		</div>

		<div class={progressBarContainerStyles}>
			<div style={`width: ${progress}%; height: 100%; background-color: var(--colors-${progressColor}); transition: width 0.3s ease, background-color 0.3s ease;`}></div>
		</div>

		<form onsubmit={handleSubmit}>
			<div class={formGroupStyles}>
				<label for="title" class={labelStyles}>Title (Optional)</label>
				<input
					type="text"
					id="title"
					class={inputStyles}
					bind:value={title}
					placeholder="Give your entry a title..."
				/>
				<p class={helpTextStyles}>Leave blank to use the date as the title</p>
			</div>

			<div class={formGroupStyles}>
				<label for="content" class={labelStyles}>What's on your mind?</label>
				<textarea
					id="content"
					class={textareaStyles}
					bind:value={content}
					placeholder="Let your thoughts flow freely... aim for 750 words to clear your mind."
					required
				></textarea>
				<p class={helpTextStyles}>
					Write without stopping. Don't worry about grammar or structure - just let it out.
				</p>
			</div>

			<div class={buttonGroupStyles}>
				<Button variant="outline" type="button" onclick={() => (window.location.href = '/journal')}>
					Cancel
				</Button>
				<Button variant="primary" type="submit" disabled={saving}>
					<Save size={20} />
					{saving ? 'Saving...' : 'Save Entry'}
				</Button>
			</div>
		</form>
	</div>
</div>
