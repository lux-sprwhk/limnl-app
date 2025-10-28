<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import { onMount } from 'svelte';
	import type { Card } from '$lib/types/card';
	import type { ConversationMessage } from '$lib/types/bug';
	import cardsData from '../../../cards.json';
	import Button from '$lib/components/ui/Button.svelte';
	import { Sparkles, Send, CheckCircle, ChevronDown } from 'lucide-svelte';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';
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
	let cardDrawCount = $state(0);
	const MAX_CARD_DRAWS = 3;

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

	const chatContainerStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		marginTop: '2rem'
	});

	const messagesContainerStyles = css({
		maxHeight: '400px',
		overflowY: 'auto',
		marginBottom: '1.5rem',
		display: 'flex',
		flexDirection: 'column',
		gap: '1rem'
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
		cardDrawCount += 1;
		
		// Generate commentaries for each card if LLM is configured
		if (selectedBlock && llmSettings.config.provider !== 'disabled') {
			drawnCards.forEach(card => {
				generateCardCommentary(card);
			});
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
				lifeArea: selectedBlock
			});
			cardCommentaries[cardId] = response.commentary;
		} catch (error) {
			console.error('Failed to generate card commentary:', error);
		} finally {
			delete loadingCommentaries[cardId];
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
		cardDrawCount = 0;
		cardCommentaries = {};
		loadingCommentaries = {};
	}

	function drawNewCards() {
		if (cardDrawCount < 3) {
			drawCards();
			selectedCard = null;
		}
	}

	function selectCard(card: Card) {
		selectedCard = card;
		selectedPrompt = getRandomPrompt(card.perspective_prompts);
		// Start the conversation with an initial message from the assistant
		conversationMessages = [
			{
				role: 'assistant',
				content: `You've selected "${card.name}" ${card.emoji}. ${card.card_question}\n\nTell me, what's been on your mind lately? What feels stuck or unclear?`,
				timestamp: new Date().toISOString()
			}
		];
	}

	async function sendMessage() {
		if (!userMessage.trim() || isLoading) return;

		const newMessage: ConversationMessage = {
			role: 'user',
			content: userMessage,
			timestamp: new Date().toISOString()
		};

		conversationMessages = [...conversationMessages, newMessage];
		const currentMessage = userMessage;
		userMessage = '';
		isLoading = true;

		// TODO: Integrate with LLM for actual conversation
		// For now, simulate a response
		setTimeout(() => {
			const assistantMessage: ConversationMessage = {
				role: 'assistant',
				content: `I understand. Let's explore this further. Can you tell me more about when you first noticed this issue?`,
				timestamp: new Date().toISOString()
			};
			conversationMessages = [...conversationMessages, assistantMessage];
			isLoading = false;
		}, 1000);
	}

	async function saveBug() {
		if (!discoveredBugTitle || !discoveredBugDescription) return;

		try {
			const bug = await bugsApi.create({
				title: discoveredBugTitle,
				description: discoveredBugDescription,
				cards_drawn: JSON.stringify([selectedCard?.id]),
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
			<div class={cardsContainerStyles}>
				{#each drawnCards as card (card.id)}
					{@const cardId = String(card.id)}
					{@const commentary = cardCommentaries[cardId]}
					{@const isLoading = loadingCommentaries[cardId]}
					<button
						class={cardStyles}
						onclick={() => selectCard(card)}
					>
						<div class={cardEmojiStyles}>{card.emoji}</div>
						<div class={cardNameStyles}>{card.name}</div>
						<div class={cardQuestionStyles}>{card.card_question}</div>
						<div class={cardMeaningStyles}>{card.core_meaning}</div>
						{#if isLoading}
							<div style="margin-top: 1rem; font-size: 0.875rem; color: var(--colors-text-muted); font-style: italic;">
								Loading insight...
							</div>
						{:else if commentary}
							<div style="margin-top: 1rem; padding-top: 1rem; border-top: 1px solid var(--colors-border-liminal); font-size: 0.875rem; color: var(--colors-text-accent); line-height: 1.5;">
								{commentary}
							</div>
						{/if}
					</button>
				{/each}
			</div>
		{:else if !bugDiscovered}
			<div class={selectedCardDisplayStyles}>
				<div class={selectedCardEmojiStyles}>{selectedCard.emoji}</div>
				<div class={selectedCardInfoStyles}>
					<div class={selectedCardNameStyles}>{selectedCard.name}</div>
					<div class={css({ fontSize: 'sm', color: 'text.accent', marginTop: '0.75rem', fontStyle: 'italic', lineHeight: '1.4' })}>
						{selectedPrompt}
					</div>
				</div>
			</div>

			{#if selectedCard}
				{@const cardId = String(selectedCard.id)}
				{@const commentary = cardCommentaries[cardId]}
				{@const isLoading = loadingCommentaries[cardId]}
				{#if commentary || isLoading}
					<div class={accordionStyles}>
						<Accordion.Root type="single">
							<Accordion.Item value="commentary" class={accordionItemStyles}>
								<Accordion.Trigger class={accordionTriggerStyles}>
									<span>Card Insight</span>
									<ChevronDown size={18} style="transition: transform 0.2s;" />
								</Accordion.Trigger>
								<Accordion.Content class={accordionContentStyles}>
									{#if isLoading}
										<div style="font-style: italic; color: var(--colors-text-muted);">
											Loading insight...
										</div>
									{:else}
										{commentary}
									{/if}
								</Accordion.Content>
							</Accordion.Item>
						</Accordion.Root>
					</div>
				{/if}
			{/if}

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

				<div style="margin-top: 1rem; display: flex; gap: 1rem; justify-content: center; align-items: center; flex-wrap: wrap;">
					<Button
						variant="outline"
						onclick={resetToBlockSelection}
					>
						← Back to Blocks
					</Button>
					<Button
						variant="outline"
						onclick={drawNewCards}
						disabled={cardDrawCount >= MAX_CARD_DRAWS}
					>
						Draw New Card
					</Button>
					<span style="font-size: 0.875rem; color: var(--colors-text-muted);">
						{cardDrawCount} / {MAX_CARD_DRAWS} draws
					</span>
					<Button
						variant="primary"
						onclick={() => {
							bugDiscovered = true;
							discoveredBugTitle = 'Sample Bug Title';
							discoveredBugDescription = 'Sample bug description based on conversation';
						}}
					>
						<CheckCircle size={20} />
						I've Found It!
					</Button>
				</div>
			</div>
		{:else}
			<div class={bugSummaryStyles}>
				<h2 style="font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; color: var(--colors-text-primary);">
					Bug Discovered!
				</h2>

				<div style="margin-bottom: 1.5rem;">
					<label for="bug-title" style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--colors-text-secondary); margin-bottom: 0.5rem;">
						Title
					</label>
					<input
						id="bug-title"
						type="text"
						bind:value={discoveredBugTitle}
						class={textareaStyles}
						placeholder="Give your bug a name..."
						style="min-height: auto; height: auto;"
					/>
				</div>

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
