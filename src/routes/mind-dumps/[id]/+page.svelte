<script lang="ts">
	import { onMount } from 'svelte';
	import { formatDistanceToNow, format } from 'date-fns';
	import { css } from '../../../../styled-system/css';
	import { mindDumpApi } from '$lib/api/mind-dumps';
	import type { MindDump, MindDumpAnalysisWithCardsAndTasks } from '$lib/types/mind-dump';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Trash2, Calendar, Brain, CheckSquare, Sparkles, Heart } from 'lucide-svelte';

	let { data } = $props();
	const id = data.id;

	let entry = $state<MindDump | null>(null);
	let analysis = $state<MindDumpAnalysisWithCardsAndTasks | null>(null);
	let loading = $state(true);
	let deleting = $state(false);

	onMount(async () => {
		await loadEntry();
	});

	async function loadEntry() {
		try {
			loading = true;
			const entryId = parseInt(id);
			entry = await mindDumpApi.get(entryId);
			if (entry?.id) {
				analysis = await mindDumpApi.getAnalysis(entry.id);
			}
		} catch (error) {
			console.error('Failed to load journal entry:', error);
		} finally {
			loading = false;
		}
	}

	async function handleDelete() {
		if (!entry?.id) return;

		const confirmed = confirm('Are you sure you want to delete this entry? This cannot be undone.');
		if (!confirmed) return;

		try {
			deleting = true;
			await mindDumpApi.delete(entry.id);
			window.location.href = '/mind-dumps';
		} catch (error) {
			console.error('Failed to delete entry:', error);
			alert('Failed to delete entry. Please try again.');
		} finally {
			deleting = false;
		}
	}

	function formatRelativeTime(dateStr: string): string {
		const date = new Date(dateStr);
		return formatDistanceToNow(date, { addSuffix: true });
	}

	function formatDateTime(dateStr: string): string {
		const date = new Date(dateStr);
		return format(date, 'PPpp'); // e.g., "Apr 29, 2021, 11:59:59 AM"
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

	const characterCountBadgeStyles = css({
		display: 'inline-flex',
		alignItems: 'center',
		padding: '0.25rem 0.75rem',
		borderRadius: 'full',
		fontSize: 'xs',
		fontWeight: 'semibold',
		backgroundColor: 'breakthrough.900',
		color: 'breakthrough.200'
	});

	const contentTextStyles = css({
		fontSize: 'lg',
		lineHeight: '1.8',
		color: 'text.primary',
		whiteSpace: 'pre-wrap',
		marginBottom: '2rem'
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
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<a href="/mind-dumps" class={backButtonStyles}>
			<ArrowLeft size={16} />
			Back to Journal
		</a>
	</div>

	{#if loading}
		<div class={loadingStyles}>Loading entry...</div>
	{:else if !entry}
		<div class={notFoundStyles}>
			<Brain size={64} class={css({ margin: '0 auto 1rem', color: 'text.muted' })} />
			<h2 class={css({ fontSize: 'xl', fontWeight: 'semibold', marginBottom: '0.5rem', color: 'text.secondary' })}>
				Mind dump not found
			</h2>
			<p>This mind dump may have been deleted or doesn't exist.</p>
		</div>
	{:else}
		<div class={contentStyles}>
			<div class={titleContainerStyles}>
				<h1 class={titleStyles}>Mind dump from {formatRelativeTime(entry.created_at)}</h1>
				<div class={buttonGroupStyles}>
					<Button variant="outline" size="sm" onclick={handleDelete} disabled={deleting}>
						<Trash2 size={16} />
						{deleting ? 'Deleting...' : 'Delete'}
					</Button>
				</div>
			</div>

			<div class={metaContainerStyles}>
				<div class={metaItemStyles}>
					<Calendar size={16} />
					<span>{formatDateTime(entry.created_at)}</span>
				</div>

				<div class={metaItemStyles}>
					<Brain size={16} />
					<span class={characterCountBadgeStyles}>{entry.character_count} characters</span>
				</div>

				{#if entry.mood_tags}
					{@const moodTags = (() => {
						try {
							return JSON.parse(entry.mood_tags) as string[];
						} catch {
							return [];
						}
					})()}
					{#if moodTags.length > 0}
						<div class={metaItemStyles}>
							<Heart size={16} />
							<div class={css({
								display: 'flex',
								flexWrap: 'wrap',
								gap: '0.5rem'
							})}>
								{#each moodTags as tag}
									<span class={css({
										display: 'inline-flex',
										alignItems: 'center',
										padding: '0.25rem 0.75rem',
										borderRadius: 'full',
										fontSize: 'xs',
										fontWeight: 'semibold',
										backgroundColor: 'breakthrough.900',
										color: 'breakthrough.200',
										textTransform: 'capitalize'
									})}>
										{tag}
									</span>
								{/each}
							</div>
						</div>
					{/if}
				{/if}
			</div>

			<div class={contentTextStyles}>{entry.content}</div>

			{#if analysis}
				{#if analysis.cards.length > 0 || analysis.tasks.length > 0}
					<div class={css({
						marginTop: '2rem',
						paddingTop: '2rem',
						borderTop: '1px solid',
						borderColor: 'border.liminal'
					})}>
						{#if analysis.cards.length > 0}
							<div class={css({ marginBottom: '2rem' })}>
								<h2 class={css({
									fontSize: 'xl',
									fontWeight: 'semibold',
									color: 'text.primary',
									marginBottom: '1rem',
									display: 'flex',
									alignItems: 'center',
									gap: '0.5rem'
								})}>
									<Sparkles size={20} />
									Relevant Cards
								</h2>
								<div class={css({
									display: 'flex',
									flexDirection: 'column',
									gap: '0.75rem'
								})}>
									{#each analysis.cards as card}
										<div class={css({
											padding: '1rem',
											backgroundColor: 'void.800',
											borderRadius: 'md',
											border: '1px solid',
											borderColor: 'border.liminal'
										})}>
											<div class={css({
												fontWeight: 'semibold',
												color: 'text.primary',
												marginBottom: '0.25rem'
											})}>
												{card.card_name}
											</div>
											{#if card.relevance_note}
												<div class={css({
													fontSize: 'sm',
													color: 'text.secondary',
													marginTop: '0.25rem'
												})}>
													{card.relevance_note}
												</div>
											{/if}
										</div>
									{/each}
								</div>
							</div>
						{/if}

						{#if analysis.tasks.length > 0}
							<div>
								<h2 class={css({
									fontSize: 'xl',
									fontWeight: 'semibold',
									color: 'text.primary',
									marginBottom: '1rem',
									display: 'flex',
									alignItems: 'center',
									gap: '0.5rem'
								})}>
									<CheckSquare size={20} />
									Actionable Tasks
								</h2>
								<div class={css({
									display: 'flex',
									flexDirection: 'column',
									gap: '0.75rem'
								})}>
									{#each analysis.tasks as task}
										<div class={css({
											padding: '1rem',
											backgroundColor: 'breakthrough.950',
											borderRadius: 'md',
											border: '1px solid',
											borderColor: 'breakthrough.800'
										})}>
											<div class={css({
												fontWeight: 'semibold',
												color: 'breakthrough.200',
												marginBottom: task.description ? '0.5rem' : '0'
											})}>
												{task.title}
											</div>
											{#if task.description}
												<div class={css({
													fontSize: 'sm',
													color: 'breakthrough.300',
													marginTop: '0.25rem'
												})}>
													{task.description}
												</div>
											{/if}
										</div>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				{/if}
			{/if}

			<div class={footerStyles}>
				<div>Created: {formatDateTime(entry.created_at)}</div>
				{#if entry.updated_at !== entry.created_at}
					<div>Last edited: {formatDateTime(entry.updated_at)}</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
