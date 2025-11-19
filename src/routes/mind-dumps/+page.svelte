<script lang="ts">
	import { onMount } from 'svelte';
	import { formatDistanceToNow } from 'date-fns';
	import { css } from '../../../styled-system/css';
	import { mindDumpApi } from '$lib/api/mind-dumps';
	import type { MindDump } from '$lib/types/mind-dump';
	import Button from '$lib/components/ui/Button.svelte';
	import { Plus, Search, Brain } from 'lucide-svelte';

	let entries = $state<MindDump[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');

	onMount(async () => {
		await loadEntries();
	});

	async function loadEntries() {
		try {
			loading = true;
			entries = await mindDumpApi.list();
		} catch (error) {
			console.error('Failed to load mind dumps:', error);
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
			entries = await mindDumpApi.search(searchQuery);
		} catch (error) {
			console.error('Failed to search mind dumps:', error);
		} finally {
			loading = false;
		}
	}

	function formatRelativeTime(dateStr: string): string {
		const date = new Date(dateStr);
		return formatDistanceToNow(date, { addSuffix: true });
	}

	function truncateContent(content: string, maxLength: number = 150): string {
		if (content.length <= maxLength) return content;
		return content.substring(0, maxLength) + '...';
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

	const characterCountStyles = css({
		fontSize: 'sm',
		color: 'text.muted',
		backgroundColor: 'void.800',
		padding: '0.25rem 0.5rem',
		borderRadius: 'md'
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
				<Brain size={32} />
				Mind Dumps
			</h1>
			<Button variant="primary" onclick={() => (window.location.href = '/mind-dumps/new')}>
				<Plus size={20} />
				New Mind Dump
			</Button>
		</div>

		<div class={searchContainerStyles}>
			<input
				type="text"
				placeholder="Search your mind dumps..."
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
			<div class={loadingStyles}>Loading your mind dumps...</div>
		{:else if entries.length === 0}
			<div class={emptyStateStyles}>
				<Brain size={64} class={emptyIconStyles} />
				<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem' })}>
					No mind dumps yet
				</h2>
				<p class={css({ marginBottom: '1.5rem' })}>
					Capture your raw thoughts whenever inspiration strikes. No structure, no pressure.
				</p>
				<Button variant="primary" onclick={() => (window.location.href = '/mind-dumps/new')}>
					<Plus size={20} />
					Dump Your First Thoughts
				</Button>
			</div>
		{:else}
			<div class={entriesGridStyles}>
				{#each entries as entry}
					<a href={`/mind-dumps/${entry.id}`} class={entryCardStyles}>
						<div class={entryHeaderStyles}>
							<div>
								<h2 class={entryTitleStyles}>{formatRelativeTime(entry.created_at)}</h2>
							</div>
							<div class={characterCountStyles}>{entry.character_count} characters</div>
						</div>
						<p class={entryContentStyles}>{truncateContent(entry.content)}</p>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</div>
