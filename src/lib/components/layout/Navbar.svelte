<script lang="ts">
	import { css } from '../../../../styled-system/css';
	import { Moon, Sparkles, Bug, Settings, HelpCircle } from 'lucide-svelte';
	import { page } from '$app/stores';

	// Check if a link is active
	function isActive(path: string): boolean {
		return $page.url.pathname === path || $page.url.pathname.startsWith(path + '/');
	}

	const navStyles = css({
		position: 'fixed',
		left: 0,
		top: 0,
		bottom: 0,
		width: '240px',
		backgroundColor: 'void.900',
		borderRight: '1px solid',
		borderColor: 'border.liminal',
		display: 'flex',
		flexDirection: 'column',
		padding: '1.5rem',
		zIndex: 100
	});

	const logoSectionStyles = css({
		marginBottom: '2rem'
	});

	const logoStyles = css({
		fontSize: '2xl',
		fontWeight: 'bold',
		color: 'text.primary',
		marginBottom: '0.5rem'
	});

	const logoSubtitleStyles = css({
		fontSize: 'xs',
		color: 'text.muted',
		textTransform: 'uppercase',
		letterSpacing: 'wider'
	});

	const navLinksTopStyles = css({
		flex: 1,
		display: 'flex',
		flexDirection: 'column',
		gap: '0.5rem'
	});

	const navLinksBottomStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '0.5rem',
		borderTop: '1px solid',
		borderColor: 'border.liminal',
		paddingTop: '1rem'
	});

	const navLinkStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.75rem',
		padding: '0.75rem 1rem',
		borderRadius: 'md',
		color: 'text.secondary',
		fontSize: 'sm',
		fontWeight: 'medium',
		textDecoration: 'none',
		transition: 'all 0.2s',
		cursor: 'pointer',
		'&:hover': {
			backgroundColor: 'liminal.hover',
			color: 'text.accent',
			borderColor: 'border.hover'
		}
	});

	const navLinkActiveStyles = css({
		backgroundColor: 'void.800',
		color: 'text.accent',
		boxShadow: 'glow',
		border: '1px solid',
		borderColor: 'border.active'
	});
</script>

<nav class={navStyles}>
	<div class={logoSectionStyles}>
		<h1 class={logoStyles}>Limnl</h1>
		<p class={logoSubtitleStyles}>Dream Journal</p>
	</div>

	<div class={navLinksTopStyles}>
		<a
			href="/dreams"
			class={`${navLinkStyles} ${isActive('/dreams') ? navLinkActiveStyles : ''}`}
		>
			<Moon size={20} />
			<span>Dreams</span>
		</a>
		<a
			href="/cards/list"
			class={`${navLinkStyles} ${isActive('/cards') ? navLinkActiveStyles : ''}`}
		>
			<Sparkles size={20} />
			<span>Cards</span>
		</a>
		<a
			href="/bugs"
			class={`${navLinkStyles} ${isActive('/bugs') ? navLinkActiveStyles : ''}`}
		>
			<Bug size={20} />
			<span>Bug Tracker</span>
		</a>
	</div>

	<div class={navLinksBottomStyles}>
		<a
			href="/settings"
			class={`${navLinkStyles} ${isActive('/settings') ? navLinkActiveStyles : ''}`}
		>
			<Settings size={20} />
			<span>Settings</span>
		</a>
		<a
			href="/help"
			class={`${navLinkStyles} ${isActive('/help') ? navLinkActiveStyles : ''}`}
		>
			<HelpCircle size={20} />
			<span>Help</span>
		</a>
	</div>
</nav>
