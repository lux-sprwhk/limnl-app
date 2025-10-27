<script lang="ts">
	import { css } from '../../../styled-system/css';
	import { BookOpen, Lock, Moon, Sparkles } from 'lucide-svelte';

	const containerStyles = css({
		minHeight: '100vh',
		backgroundColor: 'bg.primary',
		padding: '2rem'
	});

	const headerStyles = css({
		maxWidth: '800px',
		margin: '0 auto 2rem'
	});

	const titleStyles = css({
		fontSize: '3xl',
		fontWeight: 'bold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const subtitleStyles = css({
		fontSize: 'md',
		color: 'text.secondary'
	});

	const contentStyles = css({
		maxWidth: '800px',
		margin: '0 auto'
	});

	const cardStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'lg',
		padding: '2rem',
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

	const textStyles = css({
		fontSize: 'md',
		color: 'text.secondary',
		lineHeight: '1.6',
		marginBottom: '1rem'
	});

	const listStyles = css({
		listStyle: 'disc',
		paddingLeft: '1.5rem',
		color: 'text.secondary',
		'& li': {
			marginBottom: '0.5rem'
		}
	});

	const featureIconStyles = css({
		color: 'breakthrough.400'
	});
</script>

<div class={containerStyles}>
	<div class={headerStyles}>
		<h1 class={titleStyles}>Help & Documentation</h1>
		<p class={subtitleStyles}>Learn how to use Limnl Dream Journal</p>
	</div>

	<div class={contentStyles}>
		<div class={cardStyles}>
			<h2 class={sectionTitleStyles}>
				<BookOpen size={24} class={featureIconStyles} />
				Getting Started
			</h2>
			<p class={textStyles}>
				Limnl is a privacy-first dream journal application that helps you capture and explore your
				dreams. All your entries are stored locally on your device with no cloud sync or telemetry.
			</p>
			<ul class={listStyles}>
				<li>Click "Dreams" in the sidebar to view all your dream entries</li>
				<li>Click "New Dream" to create a new dream entry</li>
				<li>Use the search bar to find specific dreams by keywords</li>
				<li>Click on any dream card to view its full details</li>
			</ul>
		</div>

		<div class={cardStyles}>
			<h2 class={sectionTitleStyles}>
				<Sparkles size={24} class={featureIconStyles} />
				AI Features
			</h2>
			<p class={textStyles}>
				Limnl supports optional AI features like automatic dream title generation. Configure your
				preferred LLM provider in Settings:
			</p>
			<ul class={listStyles}>
				<li><strong>Ollama</strong> - Run models locally for complete privacy (requires installation)</li>
				<li><strong>OpenAI</strong> - Use GPT models with your API key</li>
				<li><strong>Anthropic Claude</strong> - Use Claude models with your API key</li>
				<li><strong>Disabled</strong> - Turn off all AI features</li>
			</ul>
		</div>

		<div class={cardStyles}>
			<h2 class={sectionTitleStyles}>
				<Lock size={24} class={featureIconStyles} />
				Privacy & Data
			</h2>
			<p class={textStyles}>
				Your privacy is our top priority. Here's how we protect your data:
			</p>
			<ul class={listStyles}>
				<li>All dream entries are stored locally in a SQLite database on your device</li>
				<li>API keys are stored in your browser's local storage and never leave your device</li>
				<li>No telemetry, tracking, or analytics of any kind</li>
				<li>No account required - your data stays on your machine</li>
				<li>Open source - you can verify our privacy claims in the code</li>
			</ul>
		</div>

		<div class={cardStyles}>
			<h2 class={sectionTitleStyles}>
				<Moon size={24} class={featureIconStyles} />
				Tips for Dream Journaling
			</h2>
			<ul class={listStyles}>
				<li>Record your dreams as soon as you wake up, while details are still fresh</li>
				<li>Don't worry about writing perfect prose - capture the key moments and feelings</li>
				<li>Include sensory details: what did you see, hear, feel, smell, taste?</li>
				<li>Note any recurring symbols, people, or themes</li>
				<li>Track your sleep quality to see if it correlates with dream recall</li>
				<li>Review old entries periodically to notice patterns over time</li>
			</ul>
		</div>
	</div>
</div>
