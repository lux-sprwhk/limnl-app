<script lang="ts">
	import { css } from '../../styled-system/css';
	import Button from '$lib/components/ui/Button.svelte';
	import { Rocket, Palette, Package } from 'lucide-svelte';

	let count = $state(0);

	const handleClick = () => {
		count++;
	};

	const containerStyles = css({
		minHeight: '100vh',
		display: 'flex',
		flexDirection: 'column',
		alignItems: 'center',
		justifyContent: 'center',
		padding: '2rem',
		fontFamily: 'system-ui, sans-serif'
	});

	const headerStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		marginBottom: '1rem',
		color: 'gray.900'
	});

	const descriptionStyles = css({
		fontSize: 'lg',
		color: 'gray.600',
		marginBottom: '2rem',
		textAlign: 'center',
		maxWidth: '600px'
	});

	const featuresContainerStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(3, 1fr)',
		gap: '2rem',
		marginBottom: '3rem',
		maxWidth: '900px'
	});

	const featureCardStyles = css({
		padding: '1.5rem',
		borderRadius: 'lg',
		border: '1px solid',
		borderColor: 'gray.200',
		textAlign: 'center',
		'&:hover': {
			borderColor: 'blue.400',
			boxShadow: 'sm'
		}
	});

	const featureTitleStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		marginTop: '1rem',
		marginBottom: '0.5rem'
	});

	const featureDescStyles = css({
		fontSize: 'sm',
		color: 'gray.600'
	});

	const counterStyles = css({
		fontSize: '2xl',
		fontWeight: 'bold',
		color: 'blue.600',
		marginBottom: '1rem'
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '1rem',
		flexWrap: 'wrap',
		justifyContent: 'center'
	});
</script>

<div class={containerStyles}>
	<h1 class={headerStyles}>lmnl-app</h1>

	<p class={descriptionStyles}>
		A modern desktop application built with Tauri, SvelteKit, and Panda CSS
	</p>

	<div class={featuresContainerStyles}>
		<div class={featureCardStyles}>
			<Rocket size={32} class={css({ color: 'blue.600', margin: 'auto' })} />
			<h3 class={featureTitleStyles}>Tauri</h3>
			<p class={featureDescStyles}>Lightweight, secure desktop framework</p>
		</div>

		<div class={featureCardStyles}>
			<Palette size={32} class={css({ color: 'purple.600', margin: 'auto' })} />
			<h3 class={featureTitleStyles}>Panda CSS</h3>
			<p class={featureDescStyles}>Type-safe, build-time CSS-in-JS</p>
		</div>

		<div class={featureCardStyles}>
			<Package size={32} class={css({ color: 'green.600', margin: 'auto' })} />
			<h3 class={featureTitleStyles}>SvelteKit</h3>
			<p class={featureDescStyles}>Fast, reactive web framework</p>
		</div>
	</div>

	<div class={counterStyles}>Count: {count}</div>

	<div class={buttonGroupStyles}>
		<Button variant="primary" onclick={handleClick}>Increment</Button>
		<Button variant="secondary" onclick={() => (count = 0)}>Reset</Button>
		<Button variant="outline" onclick={() => (count--)}>Decrement</Button>
	</div>
</div>
