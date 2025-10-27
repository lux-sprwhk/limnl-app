<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../../../styled-system/css';
	import { dreamsApi } from '$lib/api/dreams';
	import type { Dream, UpdateDreamInput } from '$lib/types/dream';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Save } from 'lucide-svelte';

	let { id } = $props();

	let dream = $state<Dream | null>(null);
	let title = $state('');
	let content = $state('');
	let dateOccurred = $state('');
	let sleepQuality = $state<number | undefined>(undefined);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');

	onMount(async () => {
		await loadDream();
	});

	async function loadDream() {
		try {
			loading = true;
			const dreamId = parseInt(id);
			dream = await dreamsApi.get(dreamId);

			if (dream) {
				title = dream.title;
				content = dream.content;
				dateOccurred = new Date(dream.date_occurred).toISOString().split('T')[0];
				sleepQuality = dream.sleep_quality;
			}
		} catch (err) {
			console.error('Failed to load dream:', err);
			error = 'Failed to load dream';
		} finally {
			loading = false;
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!dream?.id || !title.trim() || !content.trim()) {
			error = 'Title and content are required';
			return;
		}

		try {
			saving = true;
			error = '';

			const input: UpdateDreamInput = {
				id: dream.id,
				title: title.trim(),
				content: content.trim(),
				date_occurred: new Date(dateOccurred).toISOString(),
				sleep_quality: sleepQuality
			};

			await dreamsApi.update(input);
			window.location.href = `/dreams/${dream.id}`;
		} catch (err) {
			console.error('Failed to update dream:', err);
			error = 'Failed to save changes. Please try again.';
		} finally {
			saving = false;
		}
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

	const formContainerStyles = css({
		maxWidth: '800px',
		margin: '0 auto',
		backgroundColor: 'white',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'sm',
		border: '1px solid',
		borderColor: 'gray.200'
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
		}
	});

	const textareaStyles = css({
		width: '100%',
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'gray.300',
		fontSize: 'md',
		minHeight: '200px',
		fontFamily: 'inherit',
		resize: 'vertical',
		'&:focus': {
			outline: 'none',
			borderColor: 'blue.500',
			boxShadow: '0 0 0 3px rgba(59, 130, 246, 0.1)'
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
		borderColor: 'gray.300',
		backgroundColor: 'white',
		cursor: 'pointer',
		transition: 'all 0.2s',
		'&:hover': {
			borderColor: 'blue.400'
		}
	});

	const qualityButtonActiveStyles = css({
		backgroundColor: 'blue.500',
		color: 'white',
		borderColor: 'blue.500',
		'&:hover': {
			borderColor: 'blue.600',
			backgroundColor: 'blue.600'
		}
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '1rem',
		justifyContent: 'flex-end'
	});

	const errorStyles = css({
		padding: '1rem',
		backgroundColor: 'red.50',
		color: 'red.700',
		borderRadius: 'md',
		marginBottom: '1rem',
		fontSize: 'sm'
	});

	const loadingStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'gray.600'
	});

	const helpTextStyles = css({
		fontSize: 'xs',
		color: 'gray.500',
		marginTop: '0.25rem'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href={`/dreams/${id}`} class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Dream
		</a>
		<h1 class={titleStyles}>Edit Dream</h1>
	</div>

	{#if loading}
		<div class={loadingStyles}>Loading dream...</div>
	{:else if !dream}
		<div class={loadingStyles}>Dream not found</div>
	{:else}
		<div class={formContainerStyles}>
			{#if error}
				<div class={errorStyles}>{error}</div>
			{/if}

			<form onsubmit={handleSubmit}>
				<div class={formGroupStyles}>
					<label for="date" class={labelStyles}>Dream Date</label>
					<input type="date" id="date" class={inputStyles} bind:value={dateOccurred} required />
				</div>

				<div class={formGroupStyles}>
					<label for="title" class={labelStyles}>Title</label>
					<input
						type="text"
						id="title"
						class={inputStyles}
						bind:value={title}
						placeholder="Give your dream a title..."
						required
					/>
				</div>

				<div class={formGroupStyles}>
					<label for="content" class={labelStyles}>Dream Description</label>
					<textarea
						id="content"
						class={textareaStyles}
						bind:value={content}
						placeholder="Describe your dream in as much detail as you remember..."
						required
					></textarea>
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

				<div class={buttonGroupStyles}>
					<Button
						variant="outline"
						type="button"
						onclick={() => (window.location.href = `/dreams/${id}`)}
					>
						Cancel
					</Button>
					<Button variant="primary" type="submit" disabled={saving}>
						<Save size={20} />
						{saving ? 'Saving...' : 'Save Changes'}
					</Button>
				</div>
			</form>
		</div>
	{/if}
</div>
