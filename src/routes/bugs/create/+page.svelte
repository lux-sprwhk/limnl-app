<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import type { Bug } from '$lib/types/bug';
	import { bugsApi } from '$lib/api/bugs';
	import { llmApi } from '$lib/api/llm';
	import { llmSettings } from '$lib/stores/llm-settings.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import { ArrowLeft, Save, Sparkles, Check } from 'lucide-svelte';

	let lifeArea = $state<'life' | 'work' | 'creative' | 'relationship' | ''>('');
	let description = $state('');
	let title = $state('');
	let isLoading = $state(false);
	let isOptimizing = $state(false);
	let isGeneratingTitle = $state(false);
	let error = $state('');

	const LIFE_AREAS = [
		{ id: 'life', label: 'Life', emoji: '🌱', description: 'Personal growth, relationships, and daily living' },
		{ id: 'work', label: 'Work', emoji: '💼', description: 'Career, projects, and professional challenges' },
		{ id: 'creative', label: 'Creative', emoji: '🎨', description: 'Artistic pursuits, expression, and innovation' },
		{ id: 'relationship', label: 'Relationship', emoji: '💝', description: 'Connections, communication, and intimacy' }
	] as const;

	// Styles
	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const contentStyles = css({
		maxWidth: '800px',
		width: '100%',
		margin: '0 auto'
	});

	const headerStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '1rem',
		marginBottom: '2rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary'
	});

	const formStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const formGroupStyles = css({
		marginBottom: '1.5rem',
		display: 'flex',
		flexDirection: 'column'
	});

	const labelStyles = css({
		fontSize: 'md',
		fontWeight: 'semibold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const inputStyles = css({
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		fontSize: 'md',
		transition: 'all 0.2s',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const textareaStyles = css({
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		fontSize: 'md',
		lineHeight: '1.6',
		resize: 'vertical',
		minHeight: '200px',
		transition: 'all 0.2s',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const buttonGroupStyles = css({
		display: 'flex',
		gap: '1rem',
		justifyContent: 'flex-end',
		marginTop: '2rem'
	});

	const errorStyles = css({
		padding: '1rem',
		borderRadius: 'md',
		backgroundColor: 'rgba(239, 68, 68, 0.1)',
		borderLeft: '4px solid rgb(239, 68, 68)',
		color: 'rgb(248, 113, 113)',
		marginBottom: '1rem'
	});


	const buttonRowStyles = css({
		display: 'flex',
		gap: '0.75rem',
		marginTop: '0.5rem'
	});


	async function optimizeDescription() {
		if (!description.trim()) {
			error = 'Please write a description first';
			return;
		}

		isOptimizing = true;
		error = '';

		try {
			const response = await llmApi.optimizeBugDescription({ content: description.trim() });
			description = response.optimized;
		} catch (err) {
			console.error('Failed to optimize description:', err);
			error = err instanceof Error ? err.message : 'Failed to optimize description. Please try again.';
		} finally {
			isOptimizing = false;
		}
	}

	async function generateTitle() {
		if (!description.trim()) {
			error = 'Please write a description first';
			return;
		}

		isGeneratingTitle = true;
		error = '';

		try {
			const response = await llmApi.generateBugTitle({ content: description.trim() });
			title = response.title;
		} catch (err) {
			console.error('Failed to generate title:', err);
			error = err instanceof Error ? err.message : 'Failed to generate title. Please try again.';
		} finally {
			isGeneratingTitle = false;
		}
	}

	async function handleSubmit() {
		if (lifeArea === '') {
			error = 'Please select a life area';
			return;
		}

		if (!title.trim() || !description.trim()) {
			error = 'Please fill in all fields';
			return;
		}

		isLoading = true;
		error = '';

		try {
			const bug = await bugsApi.create({
				title: title.trim(),
				description: description.trim()
			});

			// Navigate back to bug tracker
			window.location.href = '/bugs';
		} catch (err) {
			console.error('Failed to create bug:', err);
			error = 'Failed to create bug. Please try again.';
			isLoading = false;
		}
	}

	function goBack() {
		window.location.href = '/bugs';
	}
</script>

<div class={containerStyles}>
	<div class={contentStyles}>
		<div class={headerStyles}>
			<Button variant="outline" size="sm" onclick={goBack}>
				<ArrowLeft size={20} />
			</Button>
			<h1 class={titleStyles}>Create New Bug</h1>
		</div>

		<div class={formStyles}>
			{#if error}
				<div class={errorStyles}>{error}</div>
			{/if}

			<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				<!-- Life Area Selection -->
				<div class={formGroupStyles}>
					<label for="life-area" class={labelStyles}>Which life area is this bug in?</label>
					<Select
						id="life-area"
						bind:value={lifeArea}
						options={[
							{ value: '', label: 'Select a life area' },
							{ value: 'life', label: '🌱 Life' },
							{ value: 'work', label: '💼 Work' },
							{ value: 'creative', label: '🎨 Creative' },
							{ value: 'relationship', label: '💝 Relationship' }
						]}
						disabled={isLoading}
					/>
				</div>

				<!-- Description -->
				<div class={formGroupStyles}>
					<label for="description" class={labelStyles}>Describe the bug</label>
					<textarea
						id="description"
						class={textareaStyles}
						placeholder="Describe the bug in detail. What triggers it? How does it affect you? What have you already tried?"
						bind:value={description}
						disabled={isLoading || isOptimizing}
					></textarea>
					<div class={buttonRowStyles}>
						<Button
							variant="outline"
							size="sm"
							onclick={optimizeDescription}
							disabled={isLoading || isOptimizing || !description.trim() || !llmSettings.isConfigured}
							title={!llmSettings.isConfigured ? 'Configure LLM in settings first' : 'Optimize description for clarity and precision'}
						>
							<Sparkles size={16} />
							{isOptimizing ? 'Optimizing...' : 'Optimize with AI'}
						</Button>
					</div>
				</div>

				<!-- Title -->
				<div class={formGroupStyles}>
					<label for="title" class={labelStyles}>Bug title</label>
					<input
						id="title"
						type="text"
						class={inputStyles}
						placeholder="Give your bug a clear, concise title"
						bind:value={title}
						disabled={isLoading || isGeneratingTitle}
					/>
					<div class={buttonRowStyles}>
						<Button
							variant="outline"
							size="sm"
							onclick={generateTitle}
							disabled={isLoading || isGeneratingTitle || !description.trim() || !llmSettings.isConfigured}
							title={!llmSettings.isConfigured ? 'Configure LLM in settings first' : 'Generate title from description'}
						>
							<Sparkles size={16} />
							{isGeneratingTitle ? 'Generating...' : 'Generate from description'}
						</Button>
					</div>
				</div>

				<!-- Submit Buttons -->
				<div class={buttonGroupStyles}>
					<Button variant="outline" onclick={goBack} disabled={isLoading}>
						Cancel
					</Button>
					<Button variant="primary" onclick={handleSubmit} disabled={isLoading || isOptimizing || isGeneratingTitle}>
						<Save size={20} />
						{isLoading ? 'Creating...' : 'Create Bug'}
					</Button>
				</div>
			</form>
		</div>
	</div>
</div>
