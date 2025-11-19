<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../../styled-system/css';
	import { mindDumpApi } from '$lib/api/mind-dumps';
	import type { CreateMindDumpInput } from '$lib/types/mind-dump';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Clock } from 'lucide-svelte';

	let content = $state('');
	let saving = $state(false);
	let error = $state('');
	let timeRemaining = $state(60);
	let timerStarted = $state(false);
	let intervalId: number | null = null;

	const MAX_CHARACTERS = 2000;
	const TIMER_DURATION = 60; // seconds

	// Start timer when user begins typing
	$effect(() => {
		if (content.length > 0 && !timerStarted && !saving) {
			startTimer();
		}
	});

	// Enforce character limit
	$effect(() => {
		if (content.length > MAX_CHARACTERS) {
			content = content.slice(0, MAX_CHARACTERS);
		}
	});

	function startTimer() {
		timerStarted = true;
		timeRemaining = TIMER_DURATION;

		intervalId = window.setInterval(() => {
			timeRemaining--;
			if (timeRemaining <= 0) {
				stopTimer();
				autoSave();
			}
		}, 1000);
	}

	function stopTimer() {
		if (intervalId !== null) {
			clearInterval(intervalId);
			intervalId = null;
		}
	}

	async function autoSave() {
		if (!content.trim()) {
			error = 'Cannot save empty mind dump';
			return;
		}

		await saveMindDump();
	}

	async function handleManualSave() {
		stopTimer();
		await saveMindDump();
	}

	async function saveMindDump() {
		if (saving) return;

		try {
			saving = true;
			error = '';

			const trimmedContent = content.trim();
			const input: CreateMindDumpInput = {
				content: trimmedContent,
				character_count: trimmedContent.length
			};

			await mindDumpApi.create(input);
			window.location.href = '/mind-dumps';
		} catch (err) {
			console.error('Failed to create mind dump:', err);
			error = 'Failed to save entry. Please try again.';
		} finally {
			saving = false;
		}
	}

	onMount(() => {
		// Cleanup timer on unmount
		return () => {
			stopTimer();
		};
	});

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

	const timerCircleStyles = css({
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center',
		width: '64px',
		height: '64px',
		borderRadius: '50%',
		border: '2px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.800',
		fontSize: 'xl',
		fontWeight: 'bold',
		marginBottom: '1.5rem',
		marginLeft: 'auto',
		marginRight: 'auto'
	});

	const timerActiveStyles = css({
		color: 'breakthrough.200',
		borderColor: 'breakthrough.500'
	});

	const timerWarningStyles = css({
		color: 'void.200',
		borderColor: 'void.400'
	});

</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/mind-dumps" class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Mind Dump
		</a>
		<h1 class={titleStyles}>New Mind Dump Entry</h1>
	</div>

	<div class={formContainerStyles}>
		{#if error}
			<div class={errorStyles}>{error}</div>
		{/if}

		<!-- Timer Display -->
		{#if timerStarted}
			<div class={`${timerCircleStyles} ${timeRemaining <= 10 ? timerWarningStyles : timerActiveStyles}`}>
				{timeRemaining}
			</div>
		{/if}

		<div class={formGroupStyles}>
			<label for="content" class={labelStyles}>What's on your mind?</label>
			<textarea
				id="content"
				class={textareaStyles}
				bind:value={content}
				placeholder="Start typing... you have 60 seconds and 2000 characters to dump your thoughts."
				maxlength={MAX_CHARACTERS}
				autofocus
			></textarea>
			<p class={helpTextStyles}>
				{content.length}/{MAX_CHARACTERS} characters · Write without stopping. Don't worry about grammar or structure - just let it out.
			</p>
		</div>

		<div class={buttonGroupStyles}>
			<Button variant="outline" type="button" onclick={() => (window.location.href = '/mind-dumps')}>
				Cancel
			</Button>
			<Button variant="primary" type="button" onclick={handleManualSave} disabled={saving || !content.trim()}>
				<Clock size={20} />
				{saving ? 'Saving...' : 'Save Now'}
			</Button>
		</div>
	</div>
</div>
