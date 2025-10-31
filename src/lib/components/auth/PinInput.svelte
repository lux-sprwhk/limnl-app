<script lang="ts">
	import { css } from '../../../../styled-system/css';

	interface Props {
		value: string;
		onchange: (value: string) => void;
		onenter?: () => void;
		placeholder?: string;
		error?: string;
		maxLength?: number;
	}

	let { value = '', onchange, onenter, placeholder = 'Enter PIN', error = '', maxLength = 6 }: Props = $props();

	function handleInput(e: Event) {
		const input = e.target as HTMLInputElement;
		const newValue = input.value.replace(/[^0-9]/g, ''); // Only allow digits
		onchange(newValue);
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' && onenter) {
			onenter();
		}
	}

	const containerStyles = css({
		width: '100%'
	});

	const inputStyles = css({
		width: '100%',
		padding: '1rem',
		fontSize: '2xl',
		fontWeight: 'bold',
		textAlign: 'center',
		letterSpacing: '0.5rem',
		borderRadius: 'md',
		border: '2px solid',
		borderColor: error ? 'error.600' : 'border.liminal',
		backgroundColor: 'void.900',
		color: 'text.primary',
		'&:focus': {
			outline: 'none',
			borderColor: error ? 'error.500' : 'breakthrough.600',
			boxShadow: error ? 'none' : 'glow'
		},
		'&::placeholder': {
			color: 'text.muted',
			fontSize: 'sm',
			letterSpacing: 'normal'
		}
	});

	const errorStyles = css({
		marginTop: '0.5rem',
		fontSize: 'sm',
		color: 'error.300',
		textAlign: 'center'
	});
</script>

<div class={containerStyles}>
	<input
		type="password"
		inputmode="numeric"
		pattern="[0-9]*"
		{placeholder}
		{value}
		maxlength={maxLength}
		class={inputStyles}
		oninput={handleInput}
		onkeydown={handleKeyDown}
		autocomplete="off"
	/>
	{#if error}
		<div class={errorStyles}>{error}</div>
	{/if}
</div>
