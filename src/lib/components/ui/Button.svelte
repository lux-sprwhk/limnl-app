<script lang="ts">
	import { Button as BitsButton } from 'bits-ui';
	import { css } from '../../../../styled-system/css';

	interface Props {
		variant?: 'primary' | 'secondary' | 'outline';
		size?: 'sm' | 'md' | 'lg';
		children?: any;
		onclick?: () => void;
	}

	let { variant = 'primary', size = 'md', children, onclick }: Props = $props();

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
			ringOffset: '2px'
		},
		'&:disabled': {
			opacity: 0.5,
			cursor: 'not-allowed'
		}
	});

	const variantStyles = {
		primary: css({
			bg: 'blue.600',
			color: 'white',
			'&:hover': {
				bg: 'blue.700'
			}
		}),
		secondary: css({
			bg: 'gray.200',
			color: 'gray.900',
			'&:hover': {
				bg: 'gray.300'
			}
		}),
		outline: css({
			border: '1px solid',
			borderColor: 'gray.300',
			bg: 'transparent',
			'&:hover': {
				bg: 'gray.50'
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

<BitsButton.Root class={buttonClass} {onclick}>
	{@render children?.()}
</BitsButton.Root>
