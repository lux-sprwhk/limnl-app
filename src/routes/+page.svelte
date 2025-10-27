<script lang="ts">
	import { css } from '../../styled-system/css';
	import Button from '$lib/components/ui/Button.svelte';
	import { Moon, BookOpen } from 'lucide-svelte';

	const containerStyles = css({
		minHeight: '100vh',
		display: 'flex',
		flexDirection: 'column',
		alignItems: 'center',
		justifyContent: 'center',
		padding: '2rem',
		fontFamily: 'system-ui, sans-serif',
		backgroundColor: 'bg.primary'
	});

	const titleStyles = css({
		fontSize: '4xl',
		fontWeight: 'bold',
		marginBottom: '0.5rem',
		color: 'text.primary',
		textAlign: 'center'
	});

	const subtitleStyles = css({
		fontSize: 'xl',
		color: 'text.secondary',
		marginBottom: '3rem',
		textAlign: 'center'
	});

	const cardStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'xl',
		padding: '3rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		maxWidth: '500px',
		width: '100%',
		textAlign: 'center'
	});

	const iconWrapperStyles = css({
		margin: '0 auto 1.5rem',
		width: '80px',
		height: '80px',
		borderRadius: 'full',
		backgroundColor: 'void.800',
		border: '2px solid',
		borderColor: 'breakthrough.500',
		boxShadow: 'glow',
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center'
	});

	const descriptionStyles = css({
		fontSize: 'md',
		color: 'text.secondary',
		marginBottom: '2rem',
		lineHeight: '1.6'
	});

</script>

<div class={containerStyles}>
	<h1 class={titleStyles}>Lmnl Journal</h1>
	<p class={subtitleStyles}>Your private space for self-reflection</p>

	<div class={cardStyles}>
		<div class={iconWrapperStyles}>
			<Moon size={48} class={css({ color: 'breakthrough.400' })} />
		</div>

		<h2 class={css({ fontSize: '2xl', fontWeight: 'semibold', marginBottom: '1rem', color: 'text.primary' })}>
			Dream Journal
		</h2>

		<p class={descriptionStyles}>
			Capture your dreams, track patterns, and explore the symbols of your subconscious. All your
			entries are stored locally and privately on your device.
		</p>

		<Button variant="primary" size="lg" onclick={() => (window.location.href = '/dreams')}>
			<BookOpen size={20} />
			Open Dream Journal
		</Button>
	</div>
</div>
