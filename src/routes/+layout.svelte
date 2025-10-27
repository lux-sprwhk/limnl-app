<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from '$lib/components/layout/Navbar.svelte';
	import { css } from '../../styled-system/css';
	import { page } from '$app/stores';

	let { children } = $props();

	// Hide navbar on home page
	const showNavbar = $derived($page.url.pathname !== '/');

	const mainContentStyles = css({
		marginLeft: '240px',
		minHeight: '100vh',
		transition: 'margin-left 0.3s'
	});

	const mainContentFullStyles = css({
		marginLeft: 0
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{#if showNavbar}
	<Navbar />
{/if}

<main class={showNavbar ? mainContentStyles : mainContentFullStyles}>
	{@render children?.()}
</main>
