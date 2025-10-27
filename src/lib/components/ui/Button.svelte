<script lang="ts">
	import { Button as BitsButton } from 'bits-ui';
	import { css } from '../../../../styled-system/css';

		interface Props {
		variant?: 'primary' | 'secondary' | 'outline';
		size?: 'sm' | 'md' | 'lg';
		children?: any;
		onclick?: () => void;
		disabled?: boolean;
		type?: 'button' | 'submit' | 'reset';
		title?: string;
	}

	let { variant = 'primary', size = 'md', children, onclick, disabled = false, type = 'button', title }: Props = $props();

	const baseStyles = css({
		display: 'inline-flex',
		alignItems: 'center',
		justifyContent: 'center',
		fontWeight: 'medium',
		borderRadius: 'md',
		transition: 'all',
		cursor: 'pointer',
		'&:focus': {
			outline: 'none',
			ring: '2px',
			ringColor: 'breakthrough.400',
			ringOffset: '2px'
		},
		'&:disabled': {
			opacity: 0.5,
			cursor: 'not-allowed'
		}
	});

	const variantStyles = {
		primary: css({
			bg: 'breakthrough.500',
			color: 'void.950',
			boxShadow: 'glow',
			'&:hover': {
				bg: 'breakthrough.400',
				boxShadow: 'pulse'
			},
			'&:active': {
				bg: 'breakthrough.600'
			}
		}),
		secondary: css({
			bg: 'void.800',
			color: 'text.primary',
			border: '1px solid',
			borderColor: 'border.default',
			'&:hover': {
				bg: 'void.700',
				borderColor: 'border.hover'
			}
		}),
		outline: css({
			border: '1px solid',
			borderColor: 'border.liminal',
			bg: 'transparent',
			color: 'text.secondary',
			'&:hover': {
				bg: 'liminal.hover',
				borderColor: 'border.hover',
				color: 'text.accent'
			}
		})
	};

	const sizeStyles = {
		sm: css({ px: '3', py: '1.5', fontSize: 'sm' }),
		md: css({ px: '4', py: '2', fontSize: 'md' }),
		lg: css({ px: '6', py: '3', fontSize: 'lg' })
	};

	const buttonClass = `${baseStyles} ${variantStyles[variant]} ${sizeStyles[size]}`;
</script>

<BitsButton.Root class={buttonClass} {onclick} {disabled} {type} {title}>
	{@render children?.()}
</BitsButton.Root>
