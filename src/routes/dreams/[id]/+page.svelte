<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../../styled-system/css';
	import { dreamsApi } from '$lib/api/dreams';
	import type { Dream } from '$lib/types/dream';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Edit, Trash2, Calendar, Moon } from 'lucide-svelte';

	let { data } = $props();
	const id = data.id;

	let dream = $state<Dream | null>(null);
	let loading = $state(true);
	let deleting = $state(false);

	onMount(async () => {
		await loadDream();
	});

	async function loadDream() {
		try {
			loading = true;
			const dreamId = parseInt(id);
			dream = await dreamsApi.get(dreamId);
		} catch (error) {
			console.error('Failed to load dream:', error);
		} finally {
			loading = false;
		}
	}

	async function handleDelete() {
		if (!dream?.id) return;

		const confirmed = confirm('Are you sure you want to delete this dream? This cannot be undone.');
		if (!confirmed) return;

		try {
			deleting = true;
			await dreamsApi.delete(dream.id);
			window.location.href = '/dreams';
		} catch (error) {
			console.error('Failed to delete dream:', error);
			alert('Failed to delete dream. Please try again.');
		} finally {
			deleting = false;
		}
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function formatDateTime(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const headerStyles = css({
		maxWidth: '800px',
		margin: '0 auto 2rem'
	});

	const backButtonStyles = css({
		display: 'inline-flex',
		alignItems: 'center',
		gap: '0.5rem',
		color: 'text.secondary',
		fontSize: 'sm',
		marginBottom: '1rem',
		cursor: 'pointer',
		transition: 'color 0.2s',
		'&:hover': {
			color: 'text.primary'
		}
	});

	const contentStyles = css({
		maxWidth: '800px',
		margin: '0 auto',
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const titleContainerStyles = css({
		display: 'flex',
		alignItems: 'flex-start',
		justifyContent: 'space-between',
		marginBottom: '1.5rem',
		gap: '1rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		flex: '1'
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '0.5rem'
	});

	const metaContainerStyles = css({
		display: 'flex',
		flexWrap: 'wrap',
		gap: '1.5rem',
		padding: '1rem',
		backgroundColor: 'void.800',
		borderRadius: 'md',
		marginBottom: '2rem',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const metaItemStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem',
		fontSize: 'sm',
		color: 'text.secondary'
	});

	const contentTextStyles = css({
		fontSize: 'lg',
		lineHeight: '1.8',
		color: 'text.primary',
		whiteSpace: 'pre-wrap',
		marginBottom: '2rem'
	});

	const qualityBadgeStyles = css({
		display: 'inline-flex',
		alignItems: 'center',
		padding: '0.25rem 0.75rem',
		borderRadius: 'full',
		fontSize: 'xs',
		fontWeight: 'semibold'
	});

	const footerStyles = css({
		paddingTop: '1.5rem',
		borderTop: '1px solid',
		borderColor: 'border.liminal',
		fontSize: 'xs',
		color: 'text.muted'
	});

	const loadingStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});

	const notFoundStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.muted'
	});

	function getQualityColor(quality: number): string {
		if (quality >= 4) return 'breakthrough';
		if (quality >= 3) return 'liminal';
		return 'void';
	}

	function getQualityBgShade(quality: number): string {
		if (quality >= 4) return '900';
		if (quality >= 3) return '700';
		return '800';
	}

	function getQualityTextShade(quality: number): string {
		if (quality >= 4) return '200';
		if (quality >= 3) return '200';
		return '300';
	}

	function getQualityLabel(quality: number): string {
		const labels = ['Poor', 'Fair', 'Good', 'Very Good', 'Excellent'];
		return labels[quality - 1] || 'Unknown';
	}
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/dreams" class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Dreams
		</a>
	</div>

	{#if loading}
		<div class={loadingStyles}>Loading dream...</div>
	{:else if !dream}
		<div class={notFoundStyles}>
			<Moon size={64} class={css({ margin: '0 auto 1rem', color: 'text.muted' })} />
			<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem', color: 'text.secondary' })}>
				Dream not found
			</h2>
			<p>This dream may have been deleted or doesn't exist.</p>
		</div>
	{:else}
		<div class={contentStyles}>
			<div class={titleContainerStyles}>
				<h1 class={titleStyles}>{dream.title}</h1>
				<div class={buttonGroupStyles}>
					<Button
						variant="outline"
						size="sm"
						onclick={() => dream && (window.location.href = `/dreams/${dream.id}/edit`)}
					>
						<Edit size={16} />
						Edit
					</Button>
					<Button variant="outline" size="sm" onclick={handleDelete} disabled={deleting}>
						<Trash2 size={16} />
						{deleting ? 'Deleting...' : 'Delete'}
					</Button>
				</div>
			</div>

			<div class={metaContainerStyles}>
				<div class={metaItemStyles}>
					<Calendar size={16} />
					<span>Dream Date: {formatDate(dream.date_occurred)}</span>
				</div>

				{#if dream.sleep_quality}
					<div class={metaItemStyles}>
						<Moon size={16} />
						<span>Sleep Quality:</span>
						<span
							class={qualityBadgeStyles}
							style={`background-color: var(--colors-${getQualityColor(dream.sleep_quality)}-${getQualityBgShade(dream.sleep_quality)}); color: var(--colors-${getQualityColor(dream.sleep_quality)}-${getQualityTextShade(dream.sleep_quality)});`}
						>
							{getQualityLabel(dream.sleep_quality)} ({dream.sleep_quality}/5)
						</span>
					</div>
				{/if}
			</div>

			<div class={contentTextStyles}>{dream.content}</div>

			<div class={footerStyles}>
				<div>Recorded: {formatDateTime(dream.date_recorded)}</div>
				{#if dream.updated_at !== dream.created_at}
					<div>Last edited: {formatDateTime(dream.updated_at)}</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
