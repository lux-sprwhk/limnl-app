<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import Button from '$lib/components/ui/Button.svelte';
	import { Sparkles } from 'lucide-svelte';
	import type { Card } from '$lib/types/card';
	import cardsData from '../../../cards.json';

	let userInput = $state('');
	let selectedSpread = $state<'single' | 'three' | null>(null);

	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem',
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center'
	});

	const contentStyles = css({
		maxWidth: '800px',
		width: '100%',
		margin: '0 auto'
	});

	const headerStyles = css({
		textAlign: 'center',
		marginBottom: '3rem'
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

	const questionStyles = css({
		fontSize: 'xl',
		color: 'text.accent',
		fontWeight: 'medium',
		marginBottom: '0.5rem'
	});

	const subtitleStyles = css({
		fontSize: 'md',
		color: 'text.muted',
		lineHeight: '1.6'
	});

	const cardStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const inputGroupStyles = css({
		marginBottom: '2rem'
	});

	const labelStyles = css({
		display: 'block',
		fontSize: 'sm',
		fontWeight: 'medium',
		color: 'text.secondary',
		marginBottom: '0.5rem'
	});

	const textareaStyles = css({
		width: '100%',
		minHeight: '120px',
		padding: '1rem',
		borderRadius: 'lg',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		fontSize: 'md',
		lineHeight: '1.6',
		resize: 'vertical',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const spreadSelectorStyles = css({
		marginBottom: '2rem'
	});

	const spreadButtonsStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(2, 1fr)',
		gap: '1rem'
	});

	const spreadButtonStyles = css({
		padding: '1.5rem',
		borderRadius: 'lg',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.800',
		color: 'text.secondary',
		cursor: 'pointer',
		transition: 'all 0.2s',
		textAlign: 'center',
		'&:hover': {
			borderColor: 'border.hover',
			backgroundColor: 'liminal.hover',
			boxShadow: 'glow'
		}
	});

	const spreadButtonSelectedStyles = css({
		borderColor: 'border.active',
		backgroundColor: 'void.700',
		color: 'text.accent',
		boxShadow: 'glow'
	});

	const spreadTitleStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		marginBottom: '0.5rem'
	});

	const spreadDescStyles = css({
		fontSize: 'sm',
		color: 'text.muted'
	});

	const actionButtonStyles = css({
		width: '100%',
		display: 'flex',
		justifyContent: 'center',
		gap: '1rem'
	});

	const disabledOverlayStyles = css({
		position: 'relative',
		opacity: 0.5,
		pointerEvents: 'none'
	});

	const comingSoonBadgeStyles = css({
		position: 'absolute',
		top: '0.5rem',
		right: '0.5rem',
		fontSize: 'xs',
		padding: '0.25rem 0.5rem',
		borderRadius: 'md',
		backgroundColor: 'void.700',
		color: 'text.muted',
		border: '1px solid',
		borderColor: 'border.liminal'
	});
</script>

<div class={containerStyles}>
	<div class={contentStyles}>
		<div class={headerStyles}>
			<h1 class={titleStyles}>
				<Sparkles size={32} />
				Draw Cards
			</h1>
			<p class={questionStyles}>Ready to debug?</p>
			<p class={subtitleStyles}>
				Know your bug? Draw a single card for perspective. Not sure what's blocking you? Use Discover to reveal it.
			</p>
		</div>

		<div class={cardStyles}>
			<div class={inputGroupStyles}>
				<label for="user-input" class={labelStyles}>
					{selectedSpread === 'single' ? "What bug are you working on?" : "What's the vague sense?"} (optional)
				</label>
				<textarea
					id="user-input"
					class={textareaStyles}
					bind:value={userInput}
					placeholder={selectedSpread === 'single'
						? "I keep procrastinating on... I'm avoiding this conversation... This project feels stuck because..."
						: "Something feels off... I'm stuck but don't know why... Everything feels like friction lately..."}
				></textarea>
			</div>

			<div class={spreadSelectorStyles}>
				<div class={labelStyles}>Choose your spread</div>
				<div class={spreadButtonsStyles}>
					<button
						class={`${spreadButtonStyles} ${selectedSpread === 'single' ? spreadButtonSelectedStyles : ''}`}
						onclick={() => (selectedSpread = 'single')}
					>
						<div class={spreadTitleStyles}>Single Card</div>
						<div class={spreadDescStyles}>Debug a known bug</div>
					</button>

					<button
						class={`${spreadButtonStyles} ${selectedSpread === 'three' ? spreadButtonSelectedStyles : ''}`}
						onclick={() => (selectedSpread = 'three')}
					>
						<div class={spreadTitleStyles}>Discover</div>
						<div class={spreadDescStyles}>Discover hidden bugs</div>
					</button>
				</div>
			</div>

			<div class={actionButtonStyles}>
				<Button
					variant="primary"
					onclick={() => {
						if (selectedSpread === 'three') {
							window.location.href = '/bugs/discover';
						} else {
							// TODO: Handle single card drawing
							console.log('Drawing single card...', { userInput, selectedSpread });
						}
					}}
					disabled={!selectedSpread}
				>
					<Sparkles size={20} />
					{selectedSpread === 'single' ? 'Draw Card' : 'Start Discovery'}
				</Button>
			</div>
		</div>
	</div>
</div>
