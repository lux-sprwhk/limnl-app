<script lang="ts">
	import { page } from '$app/stores';
	import { css } from '../../../../styled-system/css';
	import type { Card } from '$lib/types/card';
	import type { Bug } from '$lib/types/bug';
	import { cardsApi } from '$lib/api/cards';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Sparkles } from 'lucide-svelte';
	import cardsData from '../../../cards.json';

	let card = $state<Card | null>(null);
	let bugs = $state<Bug[]>([]);
	let loading = $state(true);
	let bugsLoading = $state(false);
	let notFound = $state(false);

	$effect(() => {
		if ($page.params.id) {
			loadCard($page.params.id);
		}
	});

	async function loadCard(idParam: string) {
		try {
			loading = true;
			notFound = false;
			const cardId = parseInt(idParam, 10);

			const foundCard = cardsData.cards.find((c) => c.id === cardId);
			if (foundCard) {
				// Normalize the card data
				card = {
					...foundCard,
					card_question: foundCard.card_question || (foundCard as any).question || '',
					fortune_cookie:
						foundCard.fortune_cookie ||
						(foundCard as any).duck_wisdom ||
						(foundCard as any).wisdom ||
						''
				};
				// Load bugs for this card
				bugsLoading = true;
				try {
					bugs = await cardsApi.getCardBugs(cardId);
				} catch (error) {
					console.error('Failed to load bugs for card:', error);
					bugs = [];
				} finally {
					bugsLoading = false;
				}
			} else {
				notFound = true;
				card = null;
			}
		} catch (error) {
			console.error('Failed to load card:', error);
			notFound = true;
		} finally {
			loading = false;
		}
	}

	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const headerStyles = css({
		maxWidth: '900px',
		margin: '0 auto',
		marginBottom: '2rem'
	});

	const backButtonStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem',
		color: 'text.secondary',
		fontSize: 'sm',
		cursor: 'pointer',
		transition: 'color 0.2s',
		'&:hover': {
			color: 'text.primary'
		}
	});

	const contentStyles = css({
		maxWidth: '900px',
		margin: '0 auto'
	});

	const cardHeaderStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		marginBottom: '2rem',
		textAlign: 'center'
	});

	const emojiStyles = css({
		fontSize: '6xl',
		marginBottom: '1rem',
		lineHeight: '1'
	});

	const nameStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const traditionalStyles = css({
		fontSize: 'sm',
		color: 'text.muted',
		fontStyle: 'italic',
		marginBottom: '1rem'
	});

	const coreMeaningStyles = css({
		fontSize: 'lg',
		color: 'text.secondary',
		marginBottom: '1rem'
	});

	const sectionStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '1.5rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		marginBottom: '1.5rem'
	});

	const sectionTitleStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '1rem',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem'
	});


	const tagsContainerStyles = css({
		display: 'flex',
		flexWrap: 'wrap',
		gap: '0.5rem'
	});

	const tagStyles = css({
		fontSize: 'xs',
		color: 'text.muted',
		backgroundColor: 'void.800',
		padding: '0.25rem 0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal'
	});


	const notFoundStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});

	const notFoundIconStyles = css({
		margin: '0 auto 1rem',
		color: 'text.muted'
	});

	const loadingStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});

	const bugsListStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '1rem'
	});

	const bugItemStyles = css({
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		cursor: 'pointer',
		transition: 'all 0.2s',
		'&:hover': {
			borderColor: 'border.hover',
			backgroundColor: 'void.800'
		}
	});

	const bugTitleStyles = css({
		fontSize: 'md',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const bugDescriptionStyles = css({
		fontSize: 'sm',
		color: 'text.secondary',
		marginBottom: '0.5rem',
		lineHeight: '1.5'
	});

	const bugStatusStyles = css({
		fontSize: 'xs',
		fontWeight: 'medium',
		padding: '0.25rem 0.5rem',
		borderRadius: 'md',
		display: 'inline-block'
	});

	const emptyBugsStyles = css({
		textAlign: 'center',
		padding: '2rem',
		color: 'text.muted',
		fontSize: 'sm'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/cards/list" class={backButtonStyles}>
			<ArrowLeft size={20} />
			Back to Deck
		</a>
	</div>

	<div class={contentStyles}>
		{#if loading}
			<div class={loadingStyles}>Loading card...</div>
		{:else if notFound || !card}
			<div class={notFoundStyles}>
				<Sparkles size={64} class={notFoundIconStyles} />
				<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem' })}>
					Card not found
				</h2>
				<p class={css({ marginBottom: '1.5rem' })}>
					The card you're looking for doesn't exist.
				</p>
				<Button variant="outline" onclick={() => window.location.href = '/cards/list'}>
					Return to Deck
				</Button>
			</div>
		{:else}
			<!-- Card Header -->
			<div class={cardHeaderStyles}>
				<div class={emojiStyles}>{card.emoji}</div>
				<h1 class={nameStyles}>{card.name}</h1>
				<p class={traditionalStyles}>Traditional: {card.traditional_equivalent}</p>
				<p class={coreMeaningStyles}>{card.core_meaning}</p>
			</div>

			<!-- Tags -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					Tags
				</div>
				<div class={tagsContainerStyles}>
					{#each card.tags as tag}
						<span class={tagStyles}>{tag}</span>
					{/each}
				</div>
			</div>

			<!-- Bugs -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					Related Bugs
				</div>
				{#if bugsLoading}
					<div class={emptyBugsStyles}>Loading bugs...</div>
				{:else if bugs.length === 0}
					<div class={emptyBugsStyles}>No bugs related to this card yet.</div>
				{:else}
					<div class={bugsListStyles}>
						{#each bugs as bug (bug.id)}
							<a href={`/bugs/${bug.id}`} class={bugItemStyles}>
								<div class={bugTitleStyles}>{bug.title}</div>
								<div class={bugStatusStyles} style={`background-color: ${bug.status === 'active' ? 'rgba(59, 130, 246, 0.1)' : bug.status === 'resolved' ? 'rgba(34, 197, 94, 0.1)' : 'rgba(107, 114, 128, 0.1)'}; color: ${bug.status === 'active' ? 'rgb(96, 165, 250)' : bug.status === 'resolved' ? 'rgb(74, 222, 128)' : 'rgb(156, 163, 175)'}`}>
									{bug.status}
								</div>
							</a>
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
