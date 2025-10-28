<script lang="ts">
	import { Loader2 } from 'lucide-svelte';

	interface Props {
		size?: number;
		text?: string;
		class?: string;
	}

	let { size = 20, text = undefined, class: className = '' } = $props();
</script>

<div class={`flex items-center gap-2 ${className}`}>
	<Loader2 {size} class="animate-spin" />
	{#if text}
		<span>{text}</span>
	{/if}
</div>

<style>
	:global(.animate-spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
