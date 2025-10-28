<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../../styled-system/css';
	import { journalApi } from '$lib/api/journal';
	import type { JournalEntry } from '$lib/types/journal';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Trash2, Calendar, BookOpen } from 'lucide-svelte';

	let { data } = $props();
	const id = data.id;

	let entry = $state<JournalEntry | null>(null);
	let loading = $state(true);
	let deleting = $state(false);

	onMount(async () => {
		await loadEntry();
	});

	async function loadEntry() {
		try {
			loading = true;
			const entryId = parseInt(id);
			entry = await journalApi.get(entryId);
		} catch (error) {
			console.error('Failed to load journal entry:', error);
		} finally {
			loading = false;
		}
	}

	async function handleDelete() {
		if (!entry?.id) return;

		const confirmed = confirm('Are you sure you want to delete this entry? This cannot be undone.');
		if (!confirmed) return;

		try {
			deleting = true;
			await journalApi.delete(entry.id);
			window.location.href = '/journal';
		} catch (error) {
			console.error('Failed to delete entry:', error);
			alert('Failed to delete entry. Please try again.');
		} finally {
			deleting = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function formatDateTime(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getEntryTitle(entry: JournalEntry): string {
		if (entry.title) return entry.title;
		return formatDate(entry.created_at);
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
		transition: 'color 0.2s',
		'&:hover': {
			color: 'text.primary'
		}
	});

	const contentStyles = css({
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

	const titleContainerStyles = css({
		display: 'flex',
		alignItems: 'flex-start',
		justifyContent: 'space-between',
		marginBottom: '1.5rem',
		gap: '1rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		flex: '1'
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '0.5rem'
	});

	const metaContainerStyles = css({
		display: 'flex',
		flexWrap: 'wrap',
		gap: '1.5rem',
		padding: '1rem',
		backgroundColor: 'void.800',
		borderRadius: 'md',
		marginBottom: '2rem',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const metaItemStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem',
		fontSize: 'sm',
		color: 'text.secondary'
	});

	const wordCountBadgeStyles = css({
		display: 'inline-flex',
		alignItems: 'center',
		padding: '0.25rem 0.75rem',
		borderRadius: 'full',
		fontSize: 'xs',
		fontWeight: 'semibold',
		backgroundColor: 'breakthrough.900',
		color: 'breakthrough.200'
	});

	const contentTextStyles = css({
		fontSize: 'lg',
		lineHeight: '1.8',
		color: 'text.primary',
		whiteSpace: 'pre-wrap',
		marginBottom: '2rem'
	});

	const footerStyles = css({
		paddingTop: '1.5rem',
		borderTop: '1px solid',
		borderColor: 'border.liminal',
		fontSize: 'xs',
		color: 'text.muted'
	});

	const loadingStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});

	const notFoundStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.muted'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/journal" class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Journal
		</a>
	</div>

	{#if loading}
		<div class={loadingStyles}>Loading entry...</div>
	{:else if !entry}
		<div class={notFoundStyles}>
			<BookOpen size={64} class={css({ margin: '0 auto 1rem', color: 'text.muted' })} />
			<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem', color: 'text.secondary' })}>
				Entry not found
			</h2>
			<p>This entry may have been deleted or doesn't exist.</p>
		</div>
	{:else}
		<div class={contentStyles}>
			<div class={titleContainerStyles}>
				<h1 class={titleStyles}>{getEntryTitle(entry)}</h1>
				<div class={buttonGroupStyles}>
					<Button variant="outline" size="sm" onclick={handleDelete} disabled={deleting}>
						<Trash2 size={16} />
						{deleting ? 'Deleting...' : 'Delete'}
					</Button>
				</div>
			</div>

			<div class={metaContainerStyles}>
				<div class={metaItemStyles}>
					<Calendar size={16} />
					<span>{formatDateTime(entry.created_at)}</span>
				</div>

				<div class={metaItemStyles}>
					<BookOpen size={16} />
					<span class={wordCountBadgeStyles}>{entry.word_count} words</span>
				</div>
			</div>

			<div class={contentTextStyles}>{entry.content}</div>

			<div class={footerStyles}>
				<div>Created: {formatDateTime(entry.created_at)}</div>
				{#if entry.updated_at !== entry.created_at}
					<div>Last edited: {formatDateTime(entry.updated_at)}</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
