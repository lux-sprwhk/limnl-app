<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../../styled-system/css';
	import type { Card } from '$lib/types/card';
	import Button from '$lib/components/ui/Button.svelte';
	import { Sparkles, Search } from 'lucide-svelte';
	import cardsData from '../../../cards.json';

	let cards = $state<Card[]>([]);
	let filteredCards = $state<Card[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');

	onMount(async () => {
		loadCards();
	});

	function loadCards() {
		try {
			loading = true;
			// Load and normalize card data
			cards = cardsData.cards.map((card) => ({
				...card,
				// Normalize the question field - some cards use "question", others use "card_question"
				card_question: card.card_question || (card as any).question || '',
				// Normalize the wisdom field - some use duck_wisdom, wisdom, or fortune_cookie
				fortune_cookie:
					card.fortune_cookie || (card as any).duck_wisdom || (card as any).wisdom || ''
			}));
			filteredCards = cards;
		} catch (error) {
			console.error('Failed to load cards:', error);
		} finally {
			loading = false;
		}
	}

	function handleSearch() {
		if (!searchQuery.trim()) {
			filteredCards = cards;
			return;
		}

		const query = searchQuery.toLowerCase();
		filteredCards = cards.filter(
			(card) =>
				card.name.toLowerCase().includes(query) ||
				card.core_meaning.toLowerCase().includes(query) ||
				card.tags.some((tag) => tag.toLowerCase().includes(query))
		);
	}

	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const headerStyles = css({
		maxWidth: '1400px',
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
		maxWidth: '1400px',
		margin: '0 auto'
	});

	const cardsGridStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
		gap: '1.5rem'
	});

	const cardStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '1.5rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		cursor: 'pointer',
		transition: 'all 0.2s',
		display: 'flex',
		flexDirection: 'column',
		gap: '1rem',
		'&:hover': {
			boxShadow: 'breakthrough',
			borderColor: 'border.hover',
			transform: 'translateY(-2px)',
			backgroundColor: 'void.800'
		}
	});

	const cardEmojiStyles = css({
		fontSize: '4xl',
		textAlign: 'center',
		lineHeight: '1'
	});

	const cardNameStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'text.primary',
		textAlign: 'center',
		marginBottom: '0.25rem'
	});

	const cardMeaningStyles = css({
		fontSize: 'sm',
		color: 'text.secondary',
		textAlign: 'center',
		lineHeight: '1.4'
	});

	const cardTraditionalStyles = css({
		fontSize: 'xs',
		color: 'text.muted',
		textAlign: 'center',
		fontStyle: 'italic'
	});

	const tagsContainerStyles = css({
		display: 'flex',
		flexWrap: 'wrap',
		gap: '0.5rem',
		marginTop: 'auto',
		paddingTop: '1rem',
		justifyContent: 'center'
	});

	const tagStyles = css({
		fontSize: 'xs',
		color: 'text.muted',
		backgroundColor: 'void.800',
		padding: '0.25rem 0.5rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal'
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

	const resultCountStyles = css({
		fontSize: 'sm',
		color: 'text.muted',
		marginBottom: '1rem'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<div class={titleContainerStyles}>
			<h1 class={titleStyles}>
				<Sparkles size={32} />
				Card Deck
			</h1>
			<Button variant="primary" onclick={() => (window.location.href = '/cards/draw')}>
				<Sparkles size={20} />
				Draw Cards
			</Button>
		</div>

		<div class={searchContainerStyles}>
			<input
				type="text"
				placeholder="Search cards by name, meaning, or tags..."
				class={searchInputStyles}
				bind:value={searchQuery}
				onkeydown={(e) => e.key === 'Enter' && handleSearch()}
				oninput={handleSearch}
			/>
			<Button variant="outline" onclick={handleSearch}>
				<Search size={20} />
				Search
			</Button>
		</div>

		{#if searchQuery}
			<div class={resultCountStyles}>
				Found {filteredCards.length} of {cards.length} cards
			</div>
		{/if}
	</div>

	<div class={contentStyles}>
		{#if loading}
			<div class={loadingStyles}>Loading cards...</div>
		{:else if filteredCards.length === 0}
			<div class={emptyStateStyles}>
				<Sparkles size={64} class={emptyIconStyles} />
				<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem' })}>
					No cards found
				</h2>
				<p class={css({ marginBottom: '1.5rem' })}>Try adjusting your search terms.</p>
				<Button variant="outline" onclick={() => { searchQuery = ''; handleSearch(); }}>
					Clear Search
				</Button>
			</div>
		{:else}
			<div class={cardsGridStyles}>
				{#each filteredCards as card}
					<a href={`/cards/${card.id}`} class={cardStyles}>
						<div class={cardEmojiStyles}>{card.emoji}</div>
						<div>
							<h2 class={cardNameStyles}>{card.name}</h2>
							<div class={cardTraditionalStyles}>({card.traditional_equivalent})</div>
						</div>
						<p class={cardMeaningStyles}>{card.core_meaning}</p>
						<div class={tagsContainerStyles}>
							{#each card.tags.slice(0, 3) as tag}
								<span class={tagStyles}>{tag}</span>
							{/each}
						</div>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</div>
