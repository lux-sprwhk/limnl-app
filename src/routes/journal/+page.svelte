<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../styled-system/css';
	import { journalApi } from '$lib/api/journal';
	import type { JournalEntry } from '$lib/types/journal';
	import Button from '$lib/components/ui/Button.svelte';
	import { Plus, Search, BookOpen } from 'lucide-svelte';

	let entries = $state<JournalEntry[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');

	onMount(async () => {
		await loadEntries();
	});

	async function loadEntries() {
		try {
			loading = true;
			entries = await journalApi.list();
		} catch (error) {
			console.error('Failed to load journal entries:', error);
		} finally {
			loading = false;
		}
	}

	async function handleSearch() {
		if (!searchQuery.trim()) {
			await loadEntries();
			return;
		}

		try {
			loading = true;
			entries = await journalApi.search(searchQuery);
		} catch (error) {
			console.error('Failed to search journal entries:', error);
		} finally {
			loading = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function truncateContent(content: string, maxLength: number = 150): string {
		if (content.length <= maxLength) return content;
		return content.substring(0, maxLength) + '...';
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
		maxWidth: '1200px',
		margin: '0 auto',
		marginBottom: '2rem'
	});

	const titleContainerStyles = css({
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'space-between',
		marginBottom: '1.5rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem'
	});

	const searchContainerStyles = css({
		display: 'flex',
		gap: '0.5rem',
		marginBottom: '1rem'
	});

	const searchInputStyles = css({
		flex: '1',
		padding: '0.75rem 1rem',
		borderRadius: 'lg',
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

	const contentStyles = css({
		maxWidth: '1200px',
		margin: '0 auto'
	});

	const entriesGridStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fill, minmax(350px, 1fr))',
		gap: '1.5rem'
	});

	const entryCardStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '1.5rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		cursor: 'pointer',
		transition: 'all 0.2s',
		'&:hover': {
			boxShadow: 'breakthrough',
			borderColor: 'border.hover',
			transform: 'translateY(-2px)',
			backgroundColor: 'void.800'
		}
	});

	const entryHeaderStyles = css({
		display: 'flex',
		justifyContent: 'space-between',
		alignItems: 'start',
		marginBottom: '1rem'
	});

	const entryTitleStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const wordCountStyles = css({
		fontSize: 'sm',
		color: 'text.muted',
		backgroundColor: 'void.800',
		padding: '0.25rem 0.5rem',
		borderRadius: 'md'
	});

	const entryDateStyles = css({
		fontSize: 'sm',
		color: 'text.muted',
		marginBottom: '1rem'
	});

	const entryContentStyles = css({
		fontSize: 'md',
		color: 'text.secondary',
		lineHeight: '1.6'
	});

	const emptyStateStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});

	const emptyIconStyles = css({
		margin: '0 auto 1rem',
		color: 'text.muted'
	});

	const loadingStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<div class={titleContainerStyles}>
			<h1 class={titleStyles}>
				<BookOpen size={32} />
				Journal
			</h1>
			<Button variant="primary" onclick={() => (window.location.href = '/journal/new')}>
				<Plus size={20} />
				New Entry
			</Button>
		</div>

		<div class={searchContainerStyles}>
			<input
				type="text"
				placeholder="Search your journal..."
				class={searchInputStyles}
				bind:value={searchQuery}
				onkeydown={(e) => e.key === 'Enter' && handleSearch()}
			/>
			<Button variant="outline" onclick={handleSearch}>
				<Search size={20} />
				Search
			</Button>
		</div>
	</div>

	<div class={contentStyles}>
		{#if loading}
			<div class={loadingStyles}>Loading your journal...</div>
		{:else if entries.length === 0}
			<div class={emptyStateStyles}>
				<BookOpen size={64} class={emptyIconStyles} />
				<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem' })}>
					No entries yet
				</h2>
				<p class={css({ marginBottom: '1.5rem' })}>
					Start your daily writing practice. Aim for 750 words to clear your mind.
				</p>
				<Button variant="primary" onclick={() => (window.location.href = '/journal/new')}>
					<Plus size={20} />
					Write Your First Entry
				</Button>
			</div>
		{:else}
			<div class={entriesGridStyles}>
				{#each entries as entry}
					<a href={`/journal/${entry.id}`} class={entryCardStyles}>
						<div class={entryHeaderStyles}>
							<div>
								<h2 class={entryTitleStyles}>{getEntryTitle(entry)}</h2>
								<div class={entryDateStyles}>{formatDate(entry.created_at)}</div>
							</div>
							<div class={wordCountStyles}>{entry.word_count} words</div>
						</div>
						<p class={entryContentStyles}>{truncateContent(entry.content)}</p>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</div>
