<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import { onMount } from 'svelte';
	import type { Card } from '$lib/types/card';
	import type { ConversationMessage } from '$lib/types/bug';
	import cardsData from '../../../cards.json';
	import Button from '$lib/components/ui/Button.svelte';
	import { Sparkles, Send, CheckCircle } from 'lucide-svelte';
	import { llmSettings } from '$lib/stores/llm-settings';
	import { bugsApi } from '$lib/api/bugs';

	let drawnCards = $state<Card[]>([]);
	let selectedCard = $state<Card | null>(null);
	let conversationMessages = $state<ConversationMessage[]>([]);
	let userMessage = $state('');
	let isLoading = $state(false);
	let bugDiscovered = $state(false);
	let discoveredBugTitle = $state('');
	let discoveredBugDescription = $state('');

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

	// Draw 3 random cards on mount
	onMount(() => {
		const allCards = cardsData.cards;
		const shuffled = [...allCards].sort(() => Math.random() - 0.5);
		drawnCards = shuffled.slice(0, 3);
	});

	function selectCard(card: Card) {
		selectedCard = card;
		// Start the conversation with an initial message from the assistant
		conversationMessages = [
			{
				role: 'assistant',
				content: `You've selected "${card.name}" ${card.emoji}. ${card.question}\n\nTell me, what's been on your mind lately? What feels stuck or unclear?`,
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
				{#if !selectedCard}
					Choose a card that resonates with you to begin uncovering what's bugging you.
				{:else if !bugDiscovered}
					Let's explore what's blocking you using the "{selectedCard.name}" card as our guide.
				{:else}
					Bug discovered! Review and save it to your tracker.
				{/if}
			</p>
		</div>

		{#if !selectedCard}
			<div class={cardsContainerStyles}>
				{#each drawnCards as card (card.id)}
					<button
						class={cardStyles}
						onclick={() => selectCard(card)}
					>
						<div class={cardEmojiStyles}>{card.emoji}</div>
						<div class={cardNameStyles}>{card.name}</div>
						<div class={cardQuestionStyles}>{card.question}</div>
						<div class={cardMeaningStyles}>{card.core_meaning}</div>
					</button>
				{/each}
			</div>
		{:else if !bugDiscovered}
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

				<div style="margin-top: 1rem; text-align: center;">
					<Button
						variant="outline"
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
					<label style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--colors-text-secondary); margin-bottom: 0.5rem;">
						Title
					</label>
					<input
						type="text"
						bind:value={discoveredBugTitle}
						class={textareaStyles}
						placeholder="Give your bug a name..."
						style="min-height: auto; height: auto;"
					/>
				</div>

				<div style="margin-bottom: 1.5rem;">
					<label style="display: block; font-size: 0.875rem; font-weight: 500; color: var(--colors-text-secondary); margin-bottom: 0.5rem;">
						Description
					</label>
					<textarea
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
