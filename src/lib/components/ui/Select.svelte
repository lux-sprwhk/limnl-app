<script lang="ts">
	import { Select } from 'bits-ui';
	import { css } from '../../../../styled-system/css';

	interface Props {
		value?: string;
		disabled?: boolean;
		options: Array<{ value: string; label: string }>;
		id?: string;
	}

	let { value = $bindable(), disabled = false, options, id }: Props = $props();

	const triggerStyles = css({
		width: '100%',
		padding: '0.75rem',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		fontSize: 'md',
		backgroundColor: 'void.900',
		color: 'text.primary',
		cursor: 'pointer',
		textAlign: 'left',
		display: 'flex',
		justifyContent: 'space-between',
		alignItems: 'center',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.active',
			boxShadow: 'glow'
		},
		'&:disabled': {
			backgroundColor: 'void.800',
			color: 'text.muted',
			cursor: 'not-allowed'
		},
		'&[data-state="open"]': {
			borderColor: 'border.active'
		}
	});

	const contentStyles = css({
		minWidth: '100%',
		backgroundColor: 'void.900',
		border: '1px solid',
		borderColor: 'border.liminal',
		borderRadius: 'md',
		boxShadow: 'breakthrough',
		zIndex: 50
	});

	const itemStyles = css({
		padding: '0.75rem',
		cursor: 'pointer',
		color: 'text.primary',
		'&:hover': {
			backgroundColor: 'liminal.hover'
		},
		'&[data-highlighted]': {
			backgroundColor: 'void.800',
			color: 'text.accent'
		}
	});

	const selectedLabel = $derived(options.find((opt) => opt.value === value)?.label || 'Select...');
</script>

<Select.Root type="single" bind:value={value as never} {disabled}>
	<Select.Trigger class={triggerStyles} {id}>
		<span>{selectedLabel}</span>
		<span>▼</span>
	</Select.Trigger>
	<Select.Portal>
		<Select.Content class={contentStyles}>
			<Select.Viewport>
				{#each options as option (option.value)}
					<Select.Item value={option.value} label={option.label} class={itemStyles}>
						{option.label}
					</Select.Item>
				{/each}
			</Select.Viewport>
		</Select.Content>
	</Select.Portal>
</Select.Root>
