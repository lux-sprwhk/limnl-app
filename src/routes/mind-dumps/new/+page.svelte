<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import { mindDumpApi } from '$lib/api/mind-dumps';
	import type { CreateMindDumpInput } from '$lib/types/mind-dumps';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Save } from 'lucide-svelte';

	let title = $state('');
	let content = $state('');
	let saving = $state(false);
	let error = $state('');


	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!content.trim()) {
			error = 'Content is required';
			return;
		}

		try {
			saving = true;
			error = '';

			const input: CreateMindDumpInput = {
				title: title.trim() || undefined,
				content: content.trim()
			};

			await mindDumpApi.create(input);
			window.location.href = '/mind-dumps';
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

		<form onsubmit={handleSubmit}>
			<div class={formGroupStyles}>
				<label for="content" class={labelStyles}>What's on your mind?</label>
				<textarea
					id="content"
					class={textareaStyles}
					bind:value={content}
					placeholder="Let your thoughts flow freely... aim for a few sentences or go for 750 words to clear your mind."
					required
				></textarea>
				<p class={helpTextStyles}>
					Write without stopping. Don't worry about grammar or structure - just let it out.
				</p>
			</div>

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

			<div class={buttonGroupStyles}>
				<Button variant="outline" type="button" onclick={() => (window.location.href = '/mind-dumps')}>
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
