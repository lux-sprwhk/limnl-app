<script lang="ts">
	import { page } from '$app/stores';
	import { css } from '../../../../styled-system/css';
	import type { Bug } from '$lib/types/bug';
	import type { DbCard } from '$lib/types/card';
	import { bugsApi } from '$lib/api/bugs';
	import { cardsApi } from '$lib/api/cards';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Sparkles, CheckCircle, Archive, Trash2, Edit } from 'lucide-svelte';

	let bug = $state<Bug | null>(null);
	let cards = $state<DbCard[]>([]);
	let loading = $state(true);
	let cardsLoading = $state(false);
	let notFound = $state(false);

	$effect(() => {
		if ($page.params.id) {
			loadBug($page.params.id);
		}
	});

	async function loadBug(idParam: string) {
		try {
			loading = true;
			notFound = false;
			const bugId = parseInt(idParam, 10);

			const foundBug = await bugsApi.get(bugId);
			if (foundBug) {
				bug = foundBug;
				// Load cards for this bug
				cardsLoading = true;
				try {
					cards = await cardsApi.getBugCards(bugId);
				} catch (error) {
					console.error('Failed to load cards for bug:', error);
					cards = [];
				} finally {
					cardsLoading = false;
				}
			} else {
				notFound = true;
				bug = null;
			}
		} catch (error) {
			console.error('Failed to load bug:', error);
			notFound = true;
		} finally {
			loading = false;
		}
	}

	async function updateBugStatus(status: 'active' | 'resolved' | 'archived') {
		if (!bug?.id) return;

		try {
			const updated = await bugsApi.update({
				id: bug.id,
				status,
				resolved_at: status === 'resolved' ? new Date().toISOString() : undefined
			});
			if (updated) {
				bug = updated;
			}
		} catch (error) {
			console.error('Failed to update bug status:', error);
		}
	}

	async function deleteBug() {
		if (!bug?.id) return;
		if (!confirm('Are you sure you want to delete this bug?')) return;

		try {
			await bugsApi.delete(bug.id);
			window.location.href = '/bugs';
		} catch (error) {
			console.error('Failed to delete bug:', error);
		}
	}

	function formatDate(dateString: string) {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			month: 'long',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStatusColor(status: string) {
		switch (status) {
			case 'active':
				return { bg: 'rgba(59, 130, 246, 0.1)', color: 'rgb(96, 165, 250)' };
			case 'resolved':
				return { bg: 'rgba(34, 197, 94, 0.1)', color: 'rgb(74, 222, 128)' };
			case 'archived':
				return { bg: 'rgba(107, 114, 128, 0.1)', color: 'rgb(156, 163, 175)' };
			default:
				return { bg: 'rgba(107, 114, 128, 0.1)', color: 'rgb(156, 163, 175)' };
		}
	}

	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const headerStyles = css({
		maxWidth: '900px',
		margin: '0 auto',
		marginBottom: '2rem'
	});

	const backButtonStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem',
		color: 'text.secondary',
		fontSize: 'sm',
		cursor: 'pointer',
		transition: 'color 0.2s',
		'&:hover': {
			color: 'text.primary'
		}
	});

	const contentStyles = css({
		maxWidth: '900px',
		margin: '0 auto'
	});

	const bugHeaderStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		marginBottom: '2rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		marginBottom: '1rem'
	});

	const metaStyles = css({
		display: 'flex',
		gap: '1.5rem',
		fontSize: 'sm',
		color: 'text.muted',
		marginBottom: '1rem',
		flexWrap: 'wrap'
	});

	const statusBadgeStyles = css({
		fontSize: 'sm',
		fontWeight: 'medium',
		padding: '0.5rem 1rem',
		borderRadius: 'md',
		display: 'inline-block',
		marginBottom: '1rem'
	});

	const actionsStyles = css({
		display: 'flex',
		gap: '0.5rem',
		flexWrap: 'wrap'
	});

	const sectionStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '1.5rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		marginBottom: '1.5rem'
	});

	const sectionTitleStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '1rem',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem'
	});

	const descriptionStyles = css({
		color: 'text.secondary',
		lineHeight: '1.8',
		whiteSpace: 'pre-wrap'
	});

	const cardsGridStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))',
		gap: '1rem'
	});

	const cardItemStyles = css({
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		cursor: 'pointer',
		transition: 'all 0.2s',
		textAlign: 'center',
		'&:hover': {
			borderColor: 'border.hover',
			backgroundColor: 'void.800'
		}
	});

	const cardNameStyles = css({
		fontSize: 'md',
		fontWeight: 'semibold',
		color: 'text.primary'
	});

	const emptyCardsStyles = css({
		textAlign: 'center',
		padding: '2rem',
		color: 'text.muted',
		fontSize: 'sm'
	});

	const notFoundStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});

	const notFoundIconStyles = css({
		margin: '0 auto 1rem',
		color: 'text.muted'
	});

	const loadingStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.secondary'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/bugs" class={backButtonStyles}>
			<ArrowLeft size={20} />
			Back to Bugs
		</a>
	</div>

	<div class={contentStyles}>
		{#if loading}
			<div class={loadingStyles}>Loading bug...</div>
		{:else if notFound || !bug}
			<div class={notFoundStyles}>
				<Sparkles size={64} class={notFoundIconStyles} />
				<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem' })}>
					Bug not found
				</h2>
				<p class={css({ marginBottom: '1.5rem' })}>
					The bug you're looking for doesn't exist.
				</p>
				<Button variant="outline" onclick={() => window.location.href = '/bugs'}>
					Return to Bugs
				</Button>
			</div>
		{:else}
			<!-- Bug Header -->
			<div class={bugHeaderStyles}>
				<h1 class={titleStyles}>{bug.title}</h1>

				<div class={metaStyles}>
					<span>Created {formatDate(bug.created_at)}</span>
					<span>Updated {formatDate(bug.updated_at)}</span>
					{#if bug.resolved_at}
						<span>Resolved {formatDate(bug.resolved_at)}</span>
					{/if}
				</div>

				<div
					class={statusBadgeStyles}
					style={`background-color: ${getStatusColor(bug.status).bg}; color: ${getStatusColor(bug.status).color}`}
				>
					{bug.status.charAt(0).toUpperCase() + bug.status.slice(1)}
				</div>

				<div class={actionsStyles}>
					{#if bug.status === 'active'}
						<Button variant="outline" size="sm" onclick={() => updateBugStatus('resolved')}>
							<CheckCircle size={16} />
							Mark Resolved
						</Button>
					{/if}
					{#if bug.status !== 'archived'}
						<Button variant="outline" size="sm" onclick={() => updateBugStatus('archived')}>
							<Archive size={16} />
							Archive
						</Button>
					{/if}
					{#if bug.status === 'archived' || bug.status === 'resolved'}
						<Button variant="outline" size="sm" onclick={() => updateBugStatus('active')}>
							Reopen
						</Button>
					{/if}
					<Button variant="outline" size="sm" onclick={deleteBug}>
						<Trash2 size={16} />
						Delete
					</Button>
				</div>
			</div>

			<!-- Description -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					Description
				</div>
				<div class={descriptionStyles}>{bug.description}</div>
			</div>

			<!-- Associated Cards -->
			<div class={sectionStyles}>
				<div class={sectionTitleStyles}>
					<Sparkles size={20} />
					Associated Cards
				</div>
				{#if cardsLoading}
					<div class={emptyCardsStyles}>Loading cards...</div>
				{:else if cards.length === 0}
					<div class={emptyCardsStyles}>No cards associated with this bug yet.</div>
				{:else}
					<div class={cardsGridStyles}>
						{#each cards as card (card.id)}
							<a href={`/cards/${card.id}`} class={cardItemStyles}>
								<div class={cardNameStyles}>{card.name}</div>
							</a>
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
