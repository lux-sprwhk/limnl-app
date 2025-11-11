<script lang="ts">
	import { onMount } from 'svelte';
	import { css } from '../../../../styled-system/css';
	import { dreamsApi } from '$lib/api/dreams';
	import type { Dream, DreamAnalysisWithCards } from '$lib/types/dream';
	import Button from '$lib/components/ui/Button.svelte';
	import { ArrowLeft, Edit, Trash2, Calendar, Moon, Sparkles, Palette, Music, BookOpen } from 'lucide-svelte';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';

	let { data } = $props();
	const id = data.id;

	let dream = $state<Dream | null>(null);
	let loading = $state(true);
	let deleting = $state(false);
	let activeTab = $state<'overview' | 'analysis' | 'creative'>('overview');
	let analysis = $state<DreamAnalysisWithCards | null>(null);
	let analysisLoading = $state(false);
	let analysisError = $state<string | null>(null);

	onMount(async () => {
		await loadDream();
		await loadAnalysis();
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

	async function loadAnalysis() {
		if (!llmSettings.isConfigured) return;

		try {
			analysisLoading = true;
			analysisError = null;
			const dreamId = parseInt(id);
			analysis = await dreamsApi.getAnalysisWithCards(dreamId);

			// Auto-generate analysis if it doesn't exist
			if (!analysis && dream) {
				await generateAnalysis();
			}
		} catch (error) {
			console.error('Failed to load analysis:', error);
			analysisError = 'Failed to load analysis';
		} finally {
			analysisLoading = false;
		}
	}

	async function generateAnalysis() {
		if (!dream || !llmSettings.isConfigured) return;

		try {
			analysisLoading = true;
			analysisError = null;
			const dreamId = parseInt(id);

			analysis = await dreamsApi.generateAnalysis({
				dream_id: dreamId,
				dream_title: dream.title,
				dream_content: dream.content,
				sleep_quality: dream.sleep_quality,
				config: llmSettings.config
			});
		} catch (error) {
			console.error('Failed to generate analysis:', error);
			analysisError =
				error instanceof Error ? error.message : 'Failed to generate analysis. Please try again.';
		} finally {
			analysisLoading = false;
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

	const tabsContainerStyles = css({
		display: 'flex',
		gap: '0.5rem',
		marginBottom: '1.5rem',
		borderBottom: '1px solid',
		borderColor: 'border.liminal'
	});

	const tabButtonStyles = css({
		padding: '0.75rem 1rem',
		fontSize: 'sm',
		fontWeight: 'medium',
		color: 'text.secondary',
		backgroundColor: 'transparent',
		border: 'none',
		borderBottom: '2px solid transparent',
		cursor: 'pointer',
		transition: 'all 0.2s',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem',
		'&:hover': {
			color: 'text.primary',
			backgroundColor: 'void.800'
		}
	});

	const tabButtonActiveStyles = css({
		color: 'breakthrough.400',
		borderBottomColor: 'breakthrough.400'
	});

	const analysisSectionStyles = css({
		marginBottom: '2rem'
	});

	const analysisTitleStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		color: 'breakthrough.300',
		marginBottom: '0.75rem',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem'
	});

	const analysisTextStyles = css({
		fontSize: 'md',
		lineHeight: '1.7',
		color: 'text.primary',
		marginBottom: '1rem'
	});

	const cardsGridStyles = css({
		display: 'grid',
		gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
		gap: '1rem',
		marginTop: '1rem'
	});

	const cardItemStyles = css({
		padding: '1rem',
		backgroundColor: 'void.800',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		transition: 'all 0.2s',
		'&:hover': {
			borderColor: 'breakthrough.500',
			boxShadow: '0 4px 12px rgba(0,0,0,0.2)'
		}
	});

	const cardNameStyles = css({
		fontSize: 'md',
		fontWeight: 'semibold',
		color: 'breakthrough.300',
		marginBottom: '0.5rem'
	});

	const cardRelevanceStyles = css({
		fontSize: 'sm',
		lineHeight: '1.6',
		color: 'text.secondary'
	});

	const promptSectionStyles = css({
		marginBottom: '2rem'
	});

	const promptSectionTitleStyles = css({
		fontSize: 'lg',
		fontWeight: 'semibold',
		color: 'breakthrough.300',
		marginBottom: '1rem',
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem'
	});

	const promptListStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '0.75rem'
	});

	const promptItemStyles = css({
		padding: '1rem',
		backgroundColor: 'void.800',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		fontSize: 'md',
		lineHeight: '1.6',
		color: 'text.primary',
		transition: 'all 0.2s',
		cursor: 'pointer',
		'&:hover': {
			borderColor: 'breakthrough.500',
			backgroundColor: 'void.700'
		}
	});

	const emptyStateStyles = css({
		textAlign: 'center',
		padding: '3rem 1rem',
		color: 'text.muted'
	});

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

			<!-- Tabs -->
			<div class={tabsContainerStyles}>
				<button
					class={`${tabButtonStyles} ${activeTab === 'overview' ? tabButtonActiveStyles : ''}`}
					onclick={() => (activeTab = 'overview')}
				>
					<Moon size={16} />
					Overview
				</button>
				<button
					class={`${tabButtonStyles} ${activeTab === 'analysis' ? tabButtonActiveStyles : ''}`}
					onclick={() => (activeTab = 'analysis')}
				>
					<Sparkles size={16} />
					Analysis
				</button>
				<button
					class={`${tabButtonStyles} ${activeTab === 'creative' ? tabButtonActiveStyles : ''}`}
					onclick={() => (activeTab = 'creative')}
				>
					<Palette size={16} />
					Creative
				</button>
			</div>

			<!-- Overview Tab -->
			{#if activeTab === 'overview'}
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
			{/if}

			<!-- Analysis Tab -->
			{#if activeTab === 'analysis'}
				{#if !llmSettings.isConfigured}
					<div class={emptyStateStyles}>
						<Sparkles size={48} class={css({ margin: '0 auto 1rem', color: 'text.muted' })} />
						<h3 class={css({ fontSize: 'lg', fontWeight: 'semibold', marginBottom: '0.5rem', color: 'text.secondary' })}>
							LLM Not Configured
						</h3>
						<p class={css({ marginBottom: '1rem' })}>
							Configure an LLM provider in Settings to generate dream analysis.
						</p>
						<Button variant="primary" onclick={() => (window.location.href = '/settings')}>
							Go to Settings
						</Button>
					</div>
				{:else if analysisLoading}
					<div class={emptyStateStyles}>
						<div class={css({ fontSize: 'lg', color: 'text.secondary' })}>Generating analysis...</div>
					</div>
				{:else if analysisError}
					<div class={emptyStateStyles}>
						<div class={css({ fontSize: 'md', color: 'red.400', marginBottom: '1rem' })}>
							{analysisError}
						</div>
						<Button variant="primary" onclick={generateAnalysis}>
							<Sparkles size={16} />
							Retry Analysis
						</Button>
					</div>
				{:else if !analysis}
					<div class={emptyStateStyles}>
						<Sparkles size={48} class={css({ margin: '0 auto 1rem', color: 'text.muted' })} />
						<h3 class={css({ fontSize: 'lg', fontWeight: 'semibold', marginBottom: '0.5rem', color: 'text.secondary' })}>
							No Analysis Yet
						</h3>
						<p class={css({ marginBottom: '1rem' })}>
							Generate an AI-powered analysis of this dream including themes, emotions, and symbolic cards.
						</p>
						<Button variant="primary" onclick={generateAnalysis}>
							<Sparkles size={16} />
							Generate Analysis
						</Button>
					</div>
				{:else}
					<!-- Themes & Patterns -->
					<div class={analysisSectionStyles}>
						<h3 class={analysisTitleStyles}>
							<Sparkles size={20} />
							Themes & Patterns
						</h3>
						<p class={analysisTextStyles}>{analysis.analysis.themes_patterns}</p>
					</div>

					<!-- Emotional Analysis -->
					<div class={analysisSectionStyles}>
						<h3 class={analysisTitleStyles}>
							<Moon size={20} />
							Emotional Analysis
						</h3>
						<p class={analysisTextStyles}>{analysis.analysis.emotional_analysis}</p>
					</div>

					<!-- Narrative Summary -->
					<div class={analysisSectionStyles}>
						<h3 class={analysisTitleStyles}>
							<Calendar size={20} />
							Narrative Summary
						</h3>
						<p class={analysisTextStyles}>{analysis.analysis.narrative_summary}</p>
					</div>

					<!-- Symbolic Cards -->
					{#if analysis.cards && analysis.cards.length > 0}
						<div class={analysisSectionStyles}>
							<h3 class={analysisTitleStyles}>
								🎴 Symbolic Cards
							</h3>
							<div class={cardsGridStyles}>
								{#each analysis.cards as card}
									<div class={cardItemStyles}>
										<div class={cardNameStyles}>{card.card_name}</div>
										{#if card.relevance_note}
											<p class={cardRelevanceStyles}>{card.relevance_note}</p>
										{/if}
									</div>
								{/each}
							</div>
						</div>
					{/if}

					<!-- Regenerate Button -->
					<div class={css({ marginTop: '2rem', textAlign: 'center' })}>
						<Button variant="outline" onclick={generateAnalysis} disabled={analysisLoading}>
							<Sparkles size={16} />
							Regenerate Analysis
						</Button>
					</div>
				{/if}
			{/if}

			<!-- Creative Tab -->
			{#if activeTab === 'creative'}
				<!-- Image Prompts Section -->
				<div class={promptSectionStyles}>
					<h3 class={promptSectionTitleStyles}>
						<Palette size={20} />
						Image Prompts
					</h3>
					<div class={promptListStyles}>
						<div class={promptItemStyles}>
							[Placeholder] A surreal dreamscape depicting {dream.title.toLowerCase()}...
						</div>
						<div class={promptItemStyles}>
							[Placeholder] An ethereal visualization of the key moments from this dream...
						</div>
						<div class={promptItemStyles}>
							[Placeholder] Abstract representation of the emotions and atmosphere in this dream...
						</div>
					</div>
				</div>

				<!-- Music Prompts Section -->
				<div class={promptSectionStyles}>
					<h3 class={promptSectionTitleStyles}>
						<Music size={20} />
						Music Prompts
					</h3>
					<div class={promptListStyles}>
						<div class={promptItemStyles}>
							[Placeholder] An ambient soundscape that captures the mood of {dream.title.toLowerCase()}...
						</div>
						<div class={promptItemStyles}>
							[Placeholder] A melodic piece reflecting the emotional journey of this dream...
						</div>
						<div class={promptItemStyles}>
							[Placeholder] Atmospheric music inspired by the themes and imagery in this dream...
						</div>
					</div>
				</div>

				<!-- Story Prompts Section -->
				<div class={promptSectionStyles}>
					<h3 class={promptSectionTitleStyles}>
						<BookOpen size={20} />
						Story Prompts
					</h3>
					<div class={promptListStyles}>
						<div class={promptItemStyles}>
							[Placeholder] Expand this dream into a short story exploring {dream.title.toLowerCase()}...
						</div>
						<div class={promptItemStyles}>
							[Placeholder] Write a narrative that continues where this dream left off...
						</div>
						<div class={promptItemStyles}>
							[Placeholder] Create a poetic interpretation of the symbols and themes in this dream...
						</div>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>
