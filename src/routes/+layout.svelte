<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Navbar from '$lib/components/layout/Navbar.svelte';
	import { css } from '../../styled-system/css';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth.svelte';

	let { children } = $props();

	// Hide navbar on home page
	const showNavbar = $derived($page.url.pathname !== '/');

	// Protected routes that require authentication
	const protectedRoutes = ['/dreams', '/bugs', '/mind-dumps', '/cards'];

	// Check authentication for protected routes
	$effect(() => {
		if (browser) {
			const currentPath = $page.url.pathname;
			const isProtectedRoute = protectedRoutes.some((route) => currentPath.startsWith(route));

			if (isProtectedRoute && authStore.authState.requirePin && !authStore.authState.isAuthenticated) {
				// User needs to authenticate, redirect to home
				goto('/');
			}
		}
	});

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
