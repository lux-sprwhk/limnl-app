<script lang="ts">
	import { css } from '../../styled-system/css';
	import Button from '$lib/components/ui/Button.svelte';
	import { Moon, BookOpen, Settings } from 'lucide-svelte';

	const containerStyles = css({
		minHeight: '100vh',
		display: 'flex',
		flexDirection: 'column',
		alignItems: 'center',
		justifyContent: 'center',
		padding: '2rem',
		fontFamily: 'system-ui, sans-serif',
		backgroundColor: 'gray.50'
	});

	const titleStyles = css({
		fontSize: '4xl',
		fontWeight: 'bold',
		marginBottom: '0.5rem',
		color: 'gray.900',
		textAlign: 'center'
	});

	const subtitleStyles = css({
		fontSize: 'xl',
		color: 'gray.600',
		marginBottom: '3rem',
		textAlign: 'center'
	});

	const cardStyles = css({
		backgroundColor: 'white',
		borderRadius: 'xl',
		padding: '3rem',
		boxShadow: 'lg',
		maxWidth: '500px',
		width: '100%',
		textAlign: 'center'
	});

	const iconWrapperStyles = css({
		margin: '0 auto 1.5rem',
		width: '80px',
		height: '80px',
		borderRadius: 'full',
		backgroundColor: 'blue.100',
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center'
	});

	const descriptionStyles = css({
		fontSize: 'md',
		color: 'gray.600',
		marginBottom: '2rem',
		lineHeight: '1.6'
	});

	const settingsLinkStyles = css({
		position: 'fixed',
		top: '1.5rem',
		right: '1.5rem',
		display: 'inline-flex',
		alignItems: 'center',
		gap: '0.5rem',
		padding: '0.75rem 1rem',
		backgroundColor: 'white',
		borderRadius: 'lg',
		boxShadow: 'sm',
		border: '1px solid',
		borderColor: 'gray.200',
		color: 'gray.700',
		fontSize: 'sm',
		fontWeight: 'medium',
		cursor: 'pointer',
		transition: 'all 0.2s',
		'&:hover': {
			backgroundColor: 'gray.50',
			borderColor: 'gray.300'
		}
	});
</script>

<a href="/settings" class={settingsLinkStyles}>
	<Settings size={20} />
	Settings
</a>

<div class={containerStyles}>
	<h1 class={titleStyles}>Lmnl Journal</h1>
	<p class={subtitleStyles}>Your private space for self-reflection</p>

	<div class={cardStyles}>
		<div class={iconWrapperStyles}>
			<Moon size={48} class={css({ color: 'blue.600' })} />
		</div>

		<h2 class={css({ fontSize: '2xl', fontWeight: 'semibold', marginBottom: '1rem' })}>
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
