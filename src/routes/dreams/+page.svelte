<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../styled-system/css';
	import { dreamsApi } from '$lib/api/dreams';
	import type { Dream } from '$lib/types/dream';
	import Button from '$lib/components/ui/Button.svelte';
	import { Plus, Search, Moon } from 'lucide-svelte';

	let dreams = $state<Dream[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');

	onMount(async () => {
		await loadDreams();
	});

	async function loadDreams() {
		try {
			loading = true;
			dreams = await dreamsApi.list();
		} catch (error) {
			console.error('Failed to load dreams:', error);
		} finally {
			loading = false;
		}
	}

	async function handleSearch() {
		if (!searchQuery.trim()) {
			await loadDreams();
			return;
		}

		try {
			loading = true;
			dreams = await dreamsApi.search(searchQuery);
		} catch (error) {
			console.error('Failed to search dreams:', error);
		} finally {
			loading = false;
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

	const dreamsGridStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fill, minmax(350px, 1fr))',
		gap: '1.5rem'
	});

	const dreamCardStyles = css({
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

	const dreamTitleStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const dreamDateStyles = css({
		fontSize: 'sm',
		color: 'text.muted',
		marginBottom: '1rem'
	});

	const dreamContentStyles = css({
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
				<Moon size={32} />
				Dream Journal
			</h1>
			<Button variant="primary" onclick={() => (window.location.href = '/dreams/new')}>
				<Plus size={20} />
				New Dream
			</Button>
		</div>

		<div class={searchContainerStyles}>
			<input
				type="text"
				placeholder="Search your dreams..."
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
			<div class={loadingStyles}>Loading your dreams...</div>
		{:else if dreams.length === 0}
			<div class={emptyStateStyles}>
				<Moon size={64} class={emptyIconStyles} />
				<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem' })}>
					No dreams yet
				</h2>
				<p class={css({ marginBottom: '1.5rem' })}>
					Start capturing your dreams to unlock patterns and insights.
				</p>
				<Button variant="primary" onclick={() => (window.location.href = '/dreams/new')}>
					<Plus size={20} />
					Record Your First Dream
				</Button>
			</div>
		{:else}
			<div class={dreamsGridStyles}>
				{#each dreams as dream}
					<a href={`/dreams/${dream.id}`} class={dreamCardStyles}>
						<h2 class={dreamTitleStyles}>{dream.title}</h2>
						<div class={dreamDateStyles}>{formatDate(dream.date_occurred)}</div>
						<p class={dreamContentStyles}>{truncateContent(dream.content)}</p>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</div>
