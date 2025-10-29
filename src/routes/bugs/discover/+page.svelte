<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import { onMount } from 'svelte';
	import type { Card } from '$lib/types/card';
	import type { ConversationMessage } from '$lib/types/bug';
	import cardsData from '../../../cards.json';
	import Button from '$lib/components/ui/Button.svelte';
	import Loader from '$lib/components/ui/Loader.svelte';
	import { Sparkles, Send, CheckCircle, ChevronDown } from 'lucide-svelte';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';
	import { userProfile } from '$lib/stores/user-profile.svelte';
	import { bugsApi } from '$lib/api/bugs';
	import { llmApi } from '$lib/api/llm';
	import { Accordion } from 'bits-ui';

	let selectedBlock = $state<'life' | 'work' | 'creative' | 'relationship' | null>(null);
	let drawnCards = $state<Card[]>([]);
	let selectedCards = $state<Card[]>([]);
	let selectedCard = $state<Card | null>(null);
	let selectedPrompt = $state<string>('');
	let cardCommentaries = $state<Record<string, string>>({});
	let loadingCommentaries = $state<Record<string, boolean>>({});
	let conversationMessages = $state<ConversationMessage[]>([]);
	let userMessage = $state('');
	let isLoading = $state(false);
	let bugDiscovered = $state(false);
	let discoveredBugTitle = $state('');
	let discoveredBugDescription = $state('');
	let isGeneratingWithAI = $state(false);
	let userMessageCount = $state(0);
	let showDrawnCards = $state(false);
	let isSelectingCards = $state(false);
	const MAX_SELECTED_CARDS = 3;
	const NUDGE_THRESHOLD = 5;

	function getRandomPrompt(prompts: string[]): string {
		return prompts[Math.floor(Math.random() * prompts.length)];
	}

	const BLOCKS = [
		{ id: 'life', label: 'Life', emoji: '🌱', description: 'Personal growth, relationships, and daily living' },
		{ id: 'work', label: 'Work', emoji: '💼', description: 'Career, projects, and professional challenges' },
		{ id: 'creative', label: 'Creative', emoji: '🎨', description: 'Artistic pursuits, expression, and innovation' },
		{ id: 'relationship', label: 'Relationship', emoji: '💝', description: 'Connections, communication, and intimacy' }
	] as const;

	// Styles
	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const contentStyles = css({
		maxWidth: '1200px',
		width: '100%',
		margin: '0 auto'
	});

	const headerStyles = css({
		textAlign: 'center',
		marginBottom: '2rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		marginBottom: '1rem',
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center',
		gap: '0.75rem'
	});

	const subtitleStyles = css({
		fontSize: 'md',
		color: 'text.muted',
		lineHeight: '1.6'
	});

	const cardsContainerStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fit, minmax(250px, 1fr))',
		gap: '1.5rem',
		marginBottom: '2rem'
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
		'&:hover': {
			borderColor: 'border.hover',
			boxShadow: 'glow',
			transform: 'translateY(-2px)'
		}
	});

	const cardSelectedStyles = css({
		borderColor: 'border.active',
		backgroundColor: 'void.700',
		boxShadow: 'glow'
	});

	const cardEmojiStyles = css({
		fontSize: '3xl',
		marginBottom: '1rem',
		textAlign: 'center'
	});

	const cardNameStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const cardQuestionStyles = css({
		fontSize: 'sm',
		color: 'text.accent',
		marginBottom: '0.5rem',
		fontStyle: 'italic'
	});

	const cardMeaningStyles = css({
		fontSize: 'sm',
		color: 'text.secondary',
		lineHeight: '1.5'
	});

	const twoColumnLayoutStyles = css({
		display: 'grid',
		gridTemplateColumns: '1fr 2fr',
		gap: '2rem',
		marginTop: '2rem',
		'@media (max-width: 1024px)': {
			gridTemplateColumns: '1fr',
			gap: '1.5rem'
		}
	});

	const leftColumnStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '1.5rem'
	});

	const rightColumnStyles = css({
		display: 'flex',
		flexDirection: 'column'
	});

	const chatContainerStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		display: 'flex',
		flexDirection: 'column',
		maxHeight: '600px'
	});

	const messagesContainerStyles = css({
		flex: 1,
		overflowY: 'auto',
		marginBottom: '1.5rem',
		display: 'flex',
		flexDirection: 'column',
		gap: '1rem',
		minHeight: '300px'
	});

	const messageStyles = css({
		padding: '1rem',
		borderRadius: 'md',
		maxWidth: '80%'
	});

	const userMessageStyles = css({
		backgroundColor: 'void.700',
		color: 'text.primary',
		marginLeft: 'auto'
	});

	const assistantMessageStyles = css({
		backgroundColor: 'void.800',
		color: 'text.secondary',
		marginRight: 'auto'
	});

	const inputContainerStyles = css({
		display: 'flex',
		gap: '1rem'
	});

	const textareaStyles = css({
		flex: 1,
		padding: '1rem',
		borderRadius: 'lg',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		fontSize: 'md',
		lineHeight: '1.6',
		resize: 'vertical',
		minHeight: '60px',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const bugSummaryStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.active',
		marginTop: '2rem'
	});

	const selectedCardDisplayStyles = css({
		backgroundColor: 'void.800',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '1.5rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		marginBottom: '1.5rem',
		display: 'flex',
		alignItems: 'center',
		gap: '1.5rem'
	});

	const selectedCardEmojiStyles = css({
		fontSize: '4xl',
		minWidth: '80px',
		textAlign: 'center'
	});

	const selectedCardInfoStyles = css({
		flex: 1
	});

	const selectedCardNameStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const selectedCardQuestionStyles = css({
		fontSize: 'md',
		color: 'text.accent',
		fontStyle: 'italic',
		lineHeight: '1.5'
	});

	const accordionStyles = css({
		marginTop: '1.5rem'
	});

	const accordionItemStyles = css({
		borderBottom: '1px solid',
		borderColor: 'border.liminal'
	});

	const accordionTriggerStyles = css({
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'space-between',
		width: '100%',
		padding: '1rem',
		backgroundColor: 'transparent',
		border: 'none',
		cursor: 'pointer',
		fontSize: 'sm',
		fontWeight: 'semibold',
		color: 'text.primary',
		transition: 'all 0.2s',
		'&:hover': {
			backgroundColor: 'void.900'
		}
	});

	const accordionContentStyles = css({
		padding: '1rem',
		backgroundColor: 'void.900',
		fontSize: 'sm',
		color: 'text.secondary',
		lineHeight: '1.6'
	});

	const blockContainerStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))',
		gap: '1.5rem',
		marginBottom: '2rem'
	});

	const blockButtonStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '2px solid',
		borderColor: 'border.liminal',
		cursor: 'pointer',
		transition: 'all 0.3s',
		textAlign: 'center',
		'&:hover': {
			borderColor: 'border.hover',
			boxShadow: 'glow',
			transform: 'translateY(-4px)'
		}
	});

	const blockButtonSelectedStyles = css({
		borderColor: 'border.active',
		backgroundColor: 'void.700',
		boxShadow: 'glow'
	});

	const blockEmojiStyles = css({
		fontSize: '4xl',
		marginBottom: '1rem'
	});

	const blockLabelStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const blockDescriptionStyles = css({
		fontSize: 'sm',
		color: 'text.secondary',
		lineHeight: '1.5'
	});

	const selectedBlockBadgeStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.75rem',
		padding: '0.5rem 1rem',
		backgroundColor: 'void.800',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.active',
		marginBottom: '1.5rem',
		width: 'fit-content',
		fontSize: 'sm'
	});

	const selectedBlockBadgeEmojiStyles = css({
		fontSize: '1.25rem'
	});

	const selectedBlockBadgeTextStyles = css({
		color: 'text.primary',
		fontWeight: 'semibold'
	});

	const selectedBlockBadgeCloseStyles = css({
		cursor: 'pointer',
		color: 'text.muted',
		transition: 'color 0.2s',
		'&:hover': {
			color: 'text.primary'
		}
	});

	function drawCards() {
		const allCards = cardsData.cards;
		const shuffled = [...allCards].sort(() => Math.random() - 0.5);
		drawnCards = shuffled.slice(0, 3);
		
		// Generate commentaries for all cards in one call if LLM is configured
		if (selectedBlock && llmSettings.config.provider !== 'disabled') {
			generateMultipleCardCommentaries(drawnCards);
		}
	}

	async function generateCardCommentary(card: Card) {
		if (!selectedBlock) return;
		
		const cardId = String(card.id);
		loadingCommentaries[cardId] = true;
		
		try {
			const response = await llmApi.commentOnCard({
				cardName: card.name,
				cardQuestion: card.card_question,
				cardMeaning: card.core_meaning,
				lifeArea: selectedBlock,
				selectedCards: selectedCards.map(c => ({
					name: c.name,
					question: c.card_question,
					meaning: c.core_meaning
				}))
			});
			cardCommentaries[cardId] = response.commentary;
		} catch (error) {
			console.error('Failed to generate card commentary:', error);
		} finally {
			delete loadingCommentaries[cardId];
		}
	}

	async function generateMultipleCardCommentaries(cards: Card[]) {
		if (!selectedBlock) return;

		// Set all cards as loading
		cards.forEach(card => {
			loadingCommentaries[String(card.id)] = true;
		});

		try {
			const response = await llmApi.commentOnMultipleCards({
				cards: cards.map(card => ({
					id: card.id,
					name: card.name,
					question: card.card_question,
					meaning: card.core_meaning
				})),
				lifeArea: selectedBlock,
				selectedCards: selectedCards.map(c => ({
					name: c.name,
					question: c.card_question,
					meaning: c.core_meaning
				}))
			});
			
			// Update commentaries for all cards
			Object.entries(response.commentaries).forEach(([cardId, commentary]) => {
				cardCommentaries[cardId] = commentary as string;
			});
		} catch (error) {
			console.error('Failed to generate card commentaries:', error);
		} finally {
			// Clear loading state for all cards
			cards.forEach(card => {
				delete loadingCommentaries[String(card.id)];
			});
		}
	}

	function selectBlock(block: 'life' | 'work' | 'creative' | 'relationship') {
		selectedBlock = block;
		drawCards();
	}

	function resetToBlockSelection() {
		selectedBlock = null;
		drawnCards = [];
		selectedCard = null;
		selectedCards = [];
		conversationMessages = [];
		cardCommentaries = {};
		loadingCommentaries = {};
		userMessageCount = 0;
		showDrawnCards = false;
	}

	function drawNewCards() {
		drawCards();
		showDrawnCards = true;
		isSelectingCards = true;
		// Only reset if we haven't started a conversation yet
		if (selectedCards.length === 0) {
			selectedCard = null;
			conversationMessages = [];
			userMessageCount = 0;
		}
	}

	function toggleCardSelection(card: Card) {
		const isSelected = selectedCards.some(c => c.id === card.id);
		const isNewCard = showDrawnCards; // True if selecting from the "Draw New Cards" section
		
		if (isSelected) {
			// Remove card if already selected
			selectedCards = selectedCards.filter(c => c.id !== card.id);
			// If we removed the active card, switch to the first remaining card
			if (selectedCard?.id === card.id && selectedCards.length > 0) {
				selectedCard = selectedCards[0];
				selectedPrompt = getRandomPrompt(selectedCard.perspective_prompts);
			}
		} else if (selectedCards.length < MAX_SELECTED_CARDS) {
			// Add card if under limit
			selectedCards = [...selectedCards, card];
			// If this is the first card, start the conversation
			if (selectedCards.length === 1) {
				selectedCard = card;
				selectedPrompt = getRandomPrompt(card.perspective_prompts);
				conversationMessages = [
					{
						role: 'assistant',
						content: `You've selected "${card.name}" ${card.emoji}. What about the card's insight resonates with you?`,
						timestamp: new Date().toISOString()
					}
				];
				showDrawnCards = false;
				isSelectingCards = false;
			} else if (isNewCard) {
				// If selecting a new card from the "Draw New Cards" section, switch to it and get LLM response
				selectedCard = card;
				selectedPrompt = getRandomPrompt(card.perspective_prompts);
				showDrawnCards = false;
				isSelectingCards = false;
				// Automatically get LLM response for the new card
				getNewCardResponse(card);
			}
		}
	}

	async function getNewCardResponse(card: Card) {
		if (!selectedBlock) return;

		isLoading = true;
		try {
			const cardId = String(card.id);
			const cardInsights = cardCommentaries[cardId] || '';
			
			// Build selected cards array with commentaries
			const selectedCardsWithCommentary = selectedCards
				.filter(c => c.id !== card.id) // Exclude the current card
				.map(c => ({
					id: c.id,
					name: c.name,
					card_question: c.card_question,
					core_meaning: c.core_meaning,
					emoji: c.emoji,
					commentary: cardCommentaries[String(c.id)] || ''
				}));
			
			const response = await llmApi.chatWithHistory({
				userMessage: `I've selected a new card: "${card.name}" ${card.emoji}. How does this card's insight relate to what we've been exploring?`,
				messages: conversationMessages.map(msg => ({
					role: msg.role,
					content: msg.content
				})),
				cardName: card.name,
				cardQuestion: card.card_question,
				cardMeaning: card.core_meaning,
				cardInsights: cardInsights,
				lifeArea: selectedBlock,
				userName: userProfile.profile.name,
				zodiacSign: userProfile.profile.zodiacSign,
				mbtiType: userProfile.profile.mbtiType,
				selectedCards: selectedCardsWithCommentary
			});

			const assistantMessage: ConversationMessage = {
				role: 'assistant',
				content: response.response,
				timestamp: new Date().toISOString()
			};
			conversationMessages = [...conversationMessages, assistantMessage];
		} catch (error) {
			console.error('Failed to get response for new card:', error);
			const errorMessage: ConversationMessage = {
				role: 'assistant',
				content: 'Sorry, I encountered an error. Please try again.',
				timestamp: new Date().toISOString()
			};
			conversationMessages = [...conversationMessages, errorMessage];
		} finally {
			isLoading = false;
		}
	}

	async function sendMessage() {
		if (!userMessage.trim() || isLoading || !selectedCard || !selectedBlock) return;

		const newMessage: ConversationMessage = {
			role: 'user',
			content: userMessage,
			timestamp: new Date().toISOString()
		};

		conversationMessages = [...conversationMessages, newMessage];
		userMessageCount++;
		const currentMessage = userMessage;
		userMessage = '';
		isLoading = true;

		try {
			const cardId = String(selectedCard.id);
			const cardInsights = cardCommentaries[cardId] || '';
			
			// Build selected cards array with commentaries
			const selectedCardsWithCommentary = selectedCards
				.filter(c => c.id !== selectedCard.id) // Exclude the current card
				.map(c => ({
					id: c.id,
					name: c.name,
					card_question: c.card_question,
					core_meaning: c.core_meaning,
					emoji: c.emoji,
					commentary: cardCommentaries[String(c.id)] || ''
				}));
			
			const response = await llmApi.chatWithHistory({
				userMessage: currentMessage,
				messages: conversationMessages.map(msg => ({
					role: msg.role,
					content: msg.content
				})),
				cardName: selectedCard.name,
				cardQuestion: selectedCard.card_question,
				cardMeaning: selectedCard.core_meaning,
				cardInsights: cardInsights,
				lifeArea: selectedBlock,
				userName: userProfile.profile.name,
				zodiacSign: userProfile.profile.zodiacSign,
				mbtiType: userProfile.profile.mbtiType,
				selectedCards: selectedCardsWithCommentary
			});

			const assistantMessage: ConversationMessage = {
				role: 'assistant',
				content: response.response,
				timestamp: new Date().toISOString()
			};
			conversationMessages = [...conversationMessages, assistantMessage];

			// Check if we should show the nudge
			if (userMessageCount === NUDGE_THRESHOLD) {
				const nudgeMessage: ConversationMessage = {
					role: 'assistant',
					content: `It seems we're exploring this card deeply! If you haven't found the bug yet, you might want to draw new cards and select a different one that resonates with you. Sometimes a fresh perspective helps uncover what's really blocking you.`,
					timestamp: new Date().toISOString()
				};
				conversationMessages = [...conversationMessages, nudgeMessage];
			}
		} catch (error) {
			console.error('Failed to get chat response:', error);
			const errorMessage: ConversationMessage = {
				role: 'assistant',
				content: 'Sorry, I encountered an error. Please try again.',
				timestamp: new Date().toISOString()
			};
			conversationMessages = [...conversationMessages, errorMessage];
		} finally {
			isLoading = false;
		}
	}

	async function generateWithAI() {
		if (!discoveredBugDescription.trim()) return;

		isGeneratingWithAI = true;
		try {
			// Generate title from description
			const titleResponse = await llmApi.generateBugTitle({
				content: discoveredBugDescription
			});
			discoveredBugTitle = titleResponse.title;
		} catch (error) {
			console.error('Failed to generate title:', error);
		} finally {
			isGeneratingWithAI = false;
		}
	}

	async function saveBug() {
		if (!discoveredBugTitle || !discoveredBugDescription) return;

		try {
			// Collect all cards that were selected during the discovery process
			const cardsUsed = selectedCards.map(card => card.id);
			
			const bug = await bugsApi.create({
				title: discoveredBugTitle,
				description: discoveredBugDescription,
				cards_drawn: JSON.stringify(cardsUsed),
				conversation_history: JSON.stringify(conversationMessages)
			});

			// Navigate to bug tracker
			window.location.href = '/bugs';
		} catch (error) {
			console.error('Failed to save bug:', error);
		}
	}
</script>

<div class={containerStyles}>
	<div class={contentStyles}>
		<div class={headerStyles}>
			<h1 class={titleStyles}>
				<Sparkles size={32} />
				Bug Discovery
			</h1>
			<p class={subtitleStyles}>
				{#if !selectedBlock}
					Which area of your life is the bug in?
				{:else if !selectedCard}
					Choose a card that resonates with you to begin uncovering what's bugging you.
				{:else if !bugDiscovered}
					Let's explore what's blocking you using the "{selectedCard.name}" card as our guide.
				{:else}
					Bug discovered! Review and save it to your tracker.
				{/if}
			</p>
		</div>

		{#if selectedBlock}
			{@const selectedBlockData = BLOCKS.find(b => b.id === selectedBlock)}
			<div class={selectedBlockBadgeStyles}>
				<span class={selectedBlockBadgeEmojiStyles}>{selectedBlockData?.emoji}</span>
				<span class={selectedBlockBadgeTextStyles}>{selectedBlockData?.label}</span>
				<button
					class={selectedBlockBadgeCloseStyles}
					onclick={resetToBlockSelection}
					title="Change life area"
				>
					✕
				</button>
			</div>
		{/if}

		{#if !selectedBlock}
			<div class={blockContainerStyles}>
				{#each BLOCKS as block (block.id)}
					<button
						class={`${blockButtonStyles} ${selectedBlock === block.id ? blockButtonSelectedStyles : ''}`}
						onclick={() => selectBlock(block.id as 'life' | 'work' | 'creative' | 'relationship')}
					>
						<div class={blockEmojiStyles}>{block.emoji}</div>
						<div class={blockLabelStyles}>{block.label}</div>
						<div class={blockDescriptionStyles}>{block.description}</div>
					</button>
				{/each}
			</div>
		{:else if !selectedCard}
			{#if !isSelectingCards}
				<div class={css({ marginBottom: '1.5rem' })}>
					<div class={css({ fontSize: 'sm', color: 'text.secondary', marginBottom: '1rem' })}>
						Select a card to begin. You can add more cards while chatting.
					</div>
				</div>
			{/if}

			<div class={cardsContainerStyles}>
				{#each drawnCards as card (card.id)}
					{@const cardId = String(card.id)}
					{@const commentary = cardCommentaries[cardId]}
					{@const isLoading = loadingCommentaries[cardId]}
					{@const isCardSelected = selectedCards.some(c => c.id === card.id)}
					<button
						class={`${cardStyles} ${isCardSelected ? cardSelectedStyles : ''}`}
						onclick={() => toggleCardSelection(card)}
						disabled={!isCardSelected && selectedCards.length >= MAX_SELECTED_CARDS}
						style={!isCardSelected && selectedCards.length >= MAX_SELECTED_CARDS ? 'opacity: 0.5; cursor: not-allowed;' : ''}
					>
						<div class={cardEmojiStyles}>{card.emoji}</div>
						<div class={cardNameStyles}>{card.name}</div>
						<div class={cardQuestionStyles}>{card.card_question}</div>
						<div class={cardMeaningStyles}>{card.core_meaning}</div>
						{#if isLoading}
							<div style="margin-top: 1rem;">
								<Loader size={16} />
							</div>
						{:else if commentary}
							<div style="margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--colors-border-liminal); font-size: 0.875rem; color: var(--colors-text-accent); line-height: 1.5;">
								{commentary}
							</div>
						{/if}
					</button>
				{/each}
			</div>

			{#if !isSelectingCards}
				<div class={css({ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '1.5rem', marginTop: '2rem', flexWrap: 'wrap' })}>
					<Button onclick={drawNewCards} variant="secondary">
						Redraw Cards
					</Button>
				</div>
			{/if}
		{:else if !bugDiscovered && !isSelectingCards}
			<div class={twoColumnLayoutStyles}>
				<!-- Left Column: Selected Cards -->
				<div class={leftColumnStyles}>
					<div>
						<h3 class={css({ fontSize: 'md', fontWeight: 'semibold', color: 'text.primary', marginBottom: '1rem' })}>
							Selected Cards ({selectedCards.length} / {MAX_SELECTED_CARDS})
						</h3>
						<div class={css({ display: 'flex', flexDirection: 'column', gap: '1rem' })}>
							{#each selectedCards as card (card.id)}
								{@const cardId = String(card.id)}
								{@const commentary = cardCommentaries[cardId]}
								{@const isLoading = loadingCommentaries[cardId]}
								<div class={css({ backgroundColor: 'void.800', borderRadius: 'lg', padding: '1rem', border: '1px solid', borderColor: 'border.liminal' })}>
									<div class={css({ fontSize: '2xl', marginBottom: '0.5rem' })}>{card.emoji}</div>
									<div class={css({ fontSize: 'sm', fontWeight: 'semibold', color: 'text.primary', marginBottom: '0.5rem' })}>{card.name}</div>
									<div class={css({ fontSize: 'xs', color: 'text.secondary', lineHeight: '1.4' })}>{card.card_question}</div>
									{#if isLoading}
										<div style="margin-top: 0.75rem;">
											<Loader size={12} />
										</div>
									{:else if commentary}
										<div style="margin-top: 0.75rem; padding-top: 0.75rem; border-top: 1px solid var(--colors-border-liminal); font-size: 0.75rem; color: var(--colors-text-accent); line-height: 1.4;">
											{commentary}
										</div>
									{/if}
								</div>
							{/each}
						</div>
					</div>

					{#if showDrawnCards}
						<div>
							<h3 class={css({ fontSize: 'md', fontWeight: 'semibold', color: 'text.primary', marginBottom: '1rem' })}>
								Draw New Cards
							</h3>
							<div class={css({ display: 'flex', flexDirection: 'column', gap: '1rem' })}>
								{#each drawnCards as card (card.id)}
									{@const cardId = String(card.id)}
									{@const commentary = cardCommentaries[cardId]}
									{@const isLoading = loadingCommentaries[cardId]}
									{@const isCardSelected = selectedCards.some(c => c.id === card.id)}
									<button
										class={`${cardStyles} ${isCardSelected ? cardSelectedStyles : ''}`}
										onclick={() => toggleCardSelection(card)}
										disabled={!isCardSelected && selectedCards.length >= MAX_SELECTED_CARDS}
										style={!isCardSelected && selectedCards.length >= MAX_SELECTED_CARDS ? 'opacity: 0.5; cursor: not-allowed;' : ''}
									>
										<div class={cardEmojiStyles}>{card.emoji}</div>
										<div class={cardNameStyles}>{card.name}</div>
										<div class={cardQuestionStyles}>{card.card_question}</div>
										<div class={cardMeaningStyles}>{card.core_meaning}</div>
										{#if isLoading}
											<div style="margin-top: 1rem;">
												<Loader size={16} />
											</div>
										{:else if commentary}
											<div style="margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--colors-border-liminal); font-size: 0.875rem; color: var(--colors-text-accent); line-height: 1.5;">
												{commentary}
											</div>
										{/if}
									</button>
								{/each}
							</div>
							<div style="margin-top: 1rem; text-align: center;">
								<Button onclick={drawNewCards} variant="secondary">
									Redraw Cards
								</Button>
							</div>
						</div>
					{/if}

					<div style="display: flex; gap: 1rem; justify-content: center; align-items: center; flex-wrap: wrap;">
						<Button
							variant="outline"
							onclick={resetToBlockSelection}
						>
							← Back to Blocks
						</Button>
						<Button
							variant="outline"
							onclick={drawNewCards}
						>
							Draw New Card
						</Button>
					</div>
				</div>

				<!-- Right Column: Chat -->
				<div class={rightColumnStyles}>
					<div class={chatContainerStyles}>
				<div class={messagesContainerStyles}>
					{#each conversationMessages as message}
						<div
							class={`${messageStyles} ${message.role === 'user' ? userMessageStyles : assistantMessageStyles}`}
						>
							{message.content}
						</div>
					{/each}
				</div>

						<div class={inputContainerStyles}>
							<textarea
								bind:value={userMessage}
								class={textareaStyles}
								placeholder="Share what's on your mind..."
								onkeydown={(e) => {
									if (e.key === 'Enter' && !e.shiftKey) {
										e.preventDefault();
										sendMessage();
									}
								}}
							></textarea>
							<Button
								variant="primary"
								onclick={sendMessage}
								disabled={!userMessage.trim() || isLoading}
							>
								<Send size={20} />
							</Button>
						</div>

						<div style="margin-top: 1rem; display: flex; gap: 1rem; justify-content: flex-end; align-items: center; flex-wrap: wrap;">
							<Button
								variant="primary"
								onclick={() => {
									bugDiscovered = true;
									// Use conversation content as placeholder for description
									if (conversationMessages.length > 0) {
										const userMessages = conversationMessages
											.filter(m => m.role === 'user')
											.map(m => m.content)
											.join('\n\n');
										discoveredBugDescription = userMessages || 'Describe what you\'ve uncovered...';
									}
								}}
							>
								<CheckCircle size={20} />
								I've Found It!
							</Button>
						</div>
					</div>
				</div>
			</div>
		{:else if isSelectingCards}
			<!-- Card Selection Mode: Show only drawn cards -->
			<div class={css({ marginBottom: '1.5rem' })}>
				<div class={css({ fontSize: 'sm', color: 'text.secondary', marginBottom: '1rem', textAlign: 'center' })}>
					Select a card to explore
				</div>
			</div>

			<div class={cardsContainerStyles}>
				{#each drawnCards as card (card.id)}
					{@const cardId = String(card.id)}
					{@const commentary = cardCommentaries[cardId]}
					{@const isLoading = loadingCommentaries[cardId]}
					{@const isCardSelected = selectedCards.some(c => c.id === card.id)}
					<button
						class={`${cardStyles} ${isCardSelected ? cardSelectedStyles : ''}`}
						onclick={() => toggleCardSelection(card)}
						disabled={!isCardSelected && selectedCards.length >= MAX_SELECTED_CARDS}
						style={!isCardSelected && selectedCards.length >= MAX_SELECTED_CARDS ? 'opacity: 0.5; cursor: not-allowed;' : ''}
					>
						<div class={cardEmojiStyles}>{card.emoji}</div>
						<div class={cardNameStyles}>{card.name}</div>
						<div class={cardQuestionStyles}>{card.card_question}</div>
						<div class={cardMeaningStyles}>{card.core_meaning}</div>
						{#if isLoading}
							<div style="margin-top: 1rem;">
								<Loader size={16} />
							</div>
						{:else if commentary}
							<div style="margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--colors-border-liminal); font-size: 0.875rem; color: var(--colors-text-accent); line-height: 1.5;">
								{commentary}
							</div>
						{/if}
					</button>
				{/each}
			</div>

			<div style="margin-top: 1rem; text-align: center;">
				<Button onclick={drawNewCards} variant="secondary">
					Redraw Cards
				</Button>
			</div>
		{:else}
			<div class={bugSummaryStyles}>
				<h2 style="font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; color: var(--colors-text-primary);">
					Bug Discovered!
				</h2>

				<div style="margin-bottom: 1.5rem;">
					<label for="bug-description" style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--colors-text-secondary); margin-bottom: 0.5rem;">
						Description
					</label>
					<textarea
						id="bug-description"
						bind:value={discoveredBugDescription}
						class={textareaStyles}
						placeholder="Describe what you've uncovered..."
						rows="5"
					></textarea>
				</div>

				<div style="margin-bottom: 1.5rem;">
					<label for="bug-title" style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--colors-text-secondary); margin-bottom: 0.5rem;">
						Title
					</label>
					<div style="display: flex; gap: 1rem; align-items: flex-start;">
						<input
							id="bug-title"
							type="text"
							bind:value={discoveredBugTitle}
							class={textareaStyles}
							placeholder="Give your bug a name..."
							style="min-height: auto; height: auto; flex: 1;"
						/>
						<Button
							variant="outline"
							onclick={generateWithAI}
							disabled={!discoveredBugDescription.trim() || isGeneratingWithAI || llmSettings.config.provider === 'disabled'}
							title={llmSettings.config.provider === 'disabled' ? 'LLM provider is disabled' : 'Generate title using AI'}
						>
							<Sparkles size={18} />
							{#if isGeneratingWithAI}
								Generating...
							{:else}
								Generate
							{/if}
						</Button>
					</div>
				</div>

				<div style="display: flex; gap: 1rem; justify-content: flex-end;">
					<Button
						variant="outline"
						onclick={() => bugDiscovered = false}
					>
						Continue Exploring
					</Button>
					<Button
						variant="primary"
						onclick={saveBug}
						disabled={!discoveredBugTitle.trim() || !discoveredBugDescription.trim()}
					>
						Save to Tracker
					</Button>
				</div>
			</div>
		{/if}
	</div>
</div>
