<script lang="ts">
	import { css } from '../../../styled-system/css';
	import { onMount } from 'svelte';
	import type { Bug } from '$lib/types/bug';
	import { bugsApi } from '$lib/api/bugs';
	import Button from '$lib/components/ui/Button.svelte';
	import { Sparkles, Plus, CheckCircle, Archive, Trash2 } from 'lucide-svelte';

	let bugs = $state<Bug[]>([]);
	let isLoading = $state(true);
	let filter = $state<'all' | 'active' | 'resolved' | 'archived'>('active');

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
		display: 'flex',
		justifyContent: 'space-between',
		alignItems: 'center',
		marginBottom: '2rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		display: 'flex',
		alignItems: 'center',
		gap: '0.75rem'
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '1rem',
		alignItems: 'center'
	});

	const filterStyles = css({
		display: 'flex',
		gap: '0.5rem'
	});

	const filterButtonStyles = css({
		padding: '0.5rem 1rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.800',
		color: 'text.secondary',
		cursor: 'pointer',
		transition: 'all 0.2s',
		'&:hover': {
			borderColor: 'border.hover',
			backgroundColor: 'liminal.hover'
		}
	});

	const filterButtonActiveStyles = css({
		borderColor: 'border.active',
		backgroundColor: 'void.700',
		color: 'text.accent'
	});

	const bugsListStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '1rem'
	});

	const bugCardStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '1.5rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		transition: 'all 0.2s',
		'&:hover': {
			borderColor: 'border.hover',
			boxShadow: 'glow'
		}
	});

	const bugHeaderStyles = css({
		display: 'flex',
		justifyContent: 'space-between',
		alignItems: 'flex-start',
		marginBottom: '0.75rem'
	});

	const bugTitleStyles = css({
		fontSize: 'xl',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.25rem'
	});

	const bugMetaStyles = css({
		fontSize: 'sm',
		color: 'text.muted',
		display: 'flex',
		gap: '1rem'
	});

	const bugDescriptionStyles = css({
		color: 'text.secondary',
		lineHeight: '1.6',
		marginBottom: '1rem'
	});

	const bugActionsStyles = css({
		display: 'flex',
		gap: '0.5rem',
		justifyContent: 'flex-end'
	});

	const statusBadgeStyles = css({
		padding: '0.25rem 0.75rem',
		borderRadius: 'md',
		fontSize: 'sm',
		fontWeight: 'medium'
	});

	const emptyStateStyles = css({
		textAlign: 'center',
		padding: '4rem 2rem',
		color: 'text.muted'
	});

	const sectionTitleStyles = css({
		fontSize: '1.5rem',
		fontWeight: '600',
		color: 'text.primary',
		marginBottom: '1rem',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem'
	});

	onMount(async () => {
		await loadBugs();
	});

	async function loadBugs() {
		isLoading = true;
		try {
			const statusFilter = filter === 'all' ? undefined : filter;
			bugs = await bugsApi.list(statusFilter);
		} catch (error) {
			console.error('Failed to load bugs:', error);
		} finally {
			isLoading = false;
		}
	}


	async function updateBugStatus(bugId: number, status: 'active' | 'resolved' | 'archived') {
		try {
			await bugsApi.update({
				id: bugId,
				status,
				resolved_at: status === 'resolved' ? new Date().toISOString() : undefined
			});
			await loadBugs();
		} catch (error) {
			console.error('Failed to update bug status:', error);
		}
	}

	async function deleteBug(bugId: number) {
		if (!confirm('Are you sure you want to delete this bug?')) return;

		try {
			await bugsApi.delete(bugId);
			await loadBugs();
		} catch (error) {
			console.error('Failed to delete bug:', error);
		}
	}

	function getStatusBadgeStyle(status: string) {
		const base = statusBadgeStyles;
		switch (status) {
			case 'active':
				return `${base} background-color: rgba(59, 130, 246, 0.1); color: rgb(96, 165, 250);`;
			case 'resolved':
				return `${base} background-color: rgba(34, 197, 94, 0.1); color: rgb(74, 222, 128);`;
			case 'archived':
				return `${base} background-color: rgba(107, 114, 128, 0.1); color: rgb(156, 163, 175);`;
			default:
				return base;
		}
	}

	function formatDate(dateString: string) {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	$effect(() => {
		loadBugs();
	});
</script>

<div class={containerStyles}>
	<div class={contentStyles}>
		<div class={headerStyles}>
			<h1 class={titleStyles}>
				Bug Tracker
			</h1>
			<div class={buttonGroupStyles}>
				<Button variant="primary" onclick={() => window.location.href = '/bugs/create'}>
					<Plus size={20} />
					Create New Bug
				</Button>
				<Button variant="primary" onclick={() => window.location.href = '/bugs/discover'}>
					<Sparkles size={20} />
					Discover New Bug
				</Button>
			</div>
		</div>

		<!-- Bugs List -->
		<h2 class={sectionTitleStyles}>
			Your Bugs
		</h2>

		<div class={filterStyles}>
			<button
				class={`${filterButtonStyles} ${filter === 'all' ? filterButtonActiveStyles : ''}`}
				onclick={() => (filter = 'all')}
			>
				All
			</button>
			<button
				class={`${filterButtonStyles} ${filter === 'active' ? filterButtonActiveStyles : ''}`}
				onclick={() => (filter = 'active')}
			>
				Active
			</button>
			<button
				class={`${filterButtonStyles} ${filter === 'resolved' ? filterButtonActiveStyles : ''}`}
				onclick={() => (filter = 'resolved')}
			>
				Resolved
			</button>
			<button
				class={`${filterButtonStyles} ${filter === 'archived' ? filterButtonActiveStyles : ''}`}
				onclick={() => (filter = 'archived')}
			>
				Archived
			</button>
		</div>

		<div class={bugsListStyles}>
			{#if isLoading}
				<div class={emptyStateStyles}>Loading bugs...</div>
			{:else if bugs.length === 0}
				<div class={emptyStateStyles}>
					<p style="font-size: 1.25rem; margin-bottom: 1rem;">No bugs found</p>
					<p>Start by discovering what's bugging you using the card-based discovery tool.</p>
				</div>
			{:else}
				{#each bugs as bug (bug.id)}
					<a href={`/bugs/${bug.id}`} class={bugCardStyles}>
						<div class={bugHeaderStyles}>
							<div style="flex: 1;">
								<h2 class={bugTitleStyles}>{bug.title}</h2>
								<div class={bugMetaStyles}>
									<span>Created {formatDate(bug.created_at)}</span>
									{#if bug.resolved_at}
										<span>• Resolved {formatDate(bug.resolved_at)}</span>
									{/if}
								</div>
							</div>
							<div style={getStatusBadgeStyle(bug.status)}>
								{bug.status}
							</div>
						</div>

						<!-- <p class={bugDescriptionStyles}>{bug.description}</p> -->

						<div class={bugActionsStyles}>
							{#if bug.status === 'active'}
								<Button variant="outline" size="sm" onclick={() => updateBugStatus(bug.id!, 'resolved')}>
									<CheckCircle size={16} />
									Mark Resolved
								</Button>
							{/if}
							{#if bug.status !== 'archived'}
								<Button variant="outline" size="sm" onclick={() => updateBugStatus(bug.id!, 'archived')}>
									<Archive size={16} />
									Archive
								</Button>
							{/if}
							<Button variant="outline" size="sm" onclick={() => deleteBug(bug.id!)}>
								<Trash2 size={16} />
								Delete
							</Button>
						</div>
					</a>
				{/each}
			{/if}
		</div>
	</div>
</div>
