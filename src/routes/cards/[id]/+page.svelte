<script lang="ts">
	import { page } from '$app/stores';
	import { css } from '../../../../styled-system/css';
	import type { Card } from '$lib/types/card';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Sparkles } from 'lucide-svelte';
	import cardsData from '../../../cards.json';

	let card = $state<Card | null>(null);
	let loading = $state(true);
	let notFound = $state(false);

	$effect(() => {
		if ($page.params.id) {
			loadCard($page.params.id);
		}
	});

	function loadCard(idParam: string) {
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

	const questionStyles = css({
		fontSize: 'md',
		color: 'text.secondary',
		fontStyle: 'italic',
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		borderLeft: '3px solid',
		borderColor: 'border.active'
	});

	const promptsListStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '1rem'
	});

	const promptItemStyles = css({
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		borderLeft: '2px solid',
		borderColor: 'border.liminal',
		fontSize: 'sm',
		color: 'text.secondary',
		lineHeight: '1.6'
	});

	const lifeAreaGridStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
		gap: '1rem'
	});

	const lifeAreaCardStyles = css({
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const lifeAreaLabelStyles = css({
		fontSize: 'sm',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem',
		textTransform: 'capitalize'
	});

	const lifeAreaTextStyles = css({
		fontSize: 'sm',
		color: 'text.secondary',
		lineHeight: '1.5'
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

	const fortuneCookieStyles = css({
		fontSize: 'md',
		color: 'text.secondary',
		lineHeight: '1.7',
		fontStyle: 'italic',
		padding: '1.5rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		borderLeft: '3px solid',
		borderColor: 'border.active'
	});

	const reversedMeaningStyles = css({
		fontSize: 'md',
		color: 'text.secondary',
		lineHeight: '1.6',
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md'
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

			<!-- Card Question -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					The Question
				</div>
				<div class={questionStyles}>
					{card.card_question}
				</div>
			</div>

			<!-- Perspective Prompts -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					Perspective Prompts
				</div>
				<div class={promptsListStyles}>
					{#each card.perspective_prompts as prompt}
						<div class={promptItemStyles}>
							{prompt}
						</div>
					{/each}
				</div>
			</div>

			<!-- Life Area Insights -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					Life Area Insights
				</div>
				<div class={lifeAreaGridStyles}>
					{#each Object.entries(card.life_area_insights) as [area, insight]}
						<div class={lifeAreaCardStyles}>
							<div class={lifeAreaLabelStyles}>{area}</div>
							<div class={lifeAreaTextStyles}>{insight}</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Fortune Cookie -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					Fortune Cookie
				</div>
				<div class={fortuneCookieStyles}>
					{card.fortune_cookie}
				</div>
			</div>

			<!-- Reversed Meaning -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					Reversed Meaning
				</div>
				<div class={reversedMeaningStyles}>
					{card.reversed_meaning}
				</div>
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
		{/if}
	</div>
</div>
