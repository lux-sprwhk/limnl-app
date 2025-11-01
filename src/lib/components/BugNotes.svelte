<script lang="ts">
	import { css } from '../../../styled-system/css';
	import type { BugNote } from '$lib/types/bug';
	import { bugsApi } from '$lib/api/bugs';
	import Button from './ui/Button.svelte';
	import { Plus, X } from 'lucide-svelte';

	interface Props {
		bugId: number;
		notes?: string; // JSON string of notes array
		onUpdate?: (notes: string) => void;
	}

	let { bugId, notes = '[]', onUpdate }: Props = $props();

	let parsedNotes = $state<BugNote[]>([]);
	let newNoteContent = $state('');
	let isAdding = $state(false);
	let saving = $state(false);

	// Parse notes when they change
	$effect(() => {
		try {
			parsedNotes = notes ? JSON.parse(notes) : [];
		} catch (e) {
			console.error('Failed to parse notes:', e);
			parsedNotes = [];
		}
	});

	async function addNote() {
		const trimmed = newNoteContent.trim();
		if (!trimmed || saving) return;

		if (trimmed.length > 500) {
			alert('Note must be 500 characters or less');
			return;
		}

		saving = true;
		try {
			const newNote: BugNote = {
				content: trimmed,
				timestamp: new Date().toISOString()
			};

			// Add new note at the beginning (most recent first)
			const updatedNotes = [newNote, ...parsedNotes];
			const notesJson = JSON.stringify(updatedNotes);

			// Update bug with new notes
			const updated = await bugsApi.update({
				id: bugId,
				notes: notesJson
			});

			if (updated) {
				parsedNotes = updatedNotes;
				newNoteContent = '';
				isAdding = false;
				if (onUpdate) {
					onUpdate(notesJson);
				}
			}
		} catch (error) {
			console.error('Failed to add note:', error);
			alert('Failed to add note. Please try again.');
		} finally {
			saving = false;
		}
	}

	async function deleteNote(index: number) {
		if (!confirm('Are you sure you want to delete this note?')) return;

		saving = true;
		try {
			const updatedNotes = parsedNotes.filter((_, i) => i !== index);
			const notesJson = JSON.stringify(updatedNotes);

			const updated = await bugsApi.update({
				id: bugId,
				notes: notesJson
			});

			if (updated) {
				parsedNotes = updatedNotes;
				if (onUpdate) {
					onUpdate(notesJson);
				}
			}
		} catch (error) {
			console.error('Failed to delete note:', error);
			alert('Failed to delete note. Please try again.');
		} finally {
			saving = false;
		}
	}

	function formatTimestamp(timestamp: string) {
		const date = new Date(timestamp);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	const notesContainerStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '1rem'
	});

	const noteItemStyles = css({
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal',
		position: 'relative',
		paddingRight: '3rem'
	});

	const noteContentStyles = css({
		color: 'text.secondary',
		lineHeight: '1.6',
		whiteSpace: 'pre-wrap',
		marginBottom: '0.5rem'
	});

	const noteTimestampStyles = css({
		fontSize: 'xs',
		color: 'text.muted'
	});

	const deleteButtonStyles = css({
		position: 'absolute',
		top: '0.75rem',
		right: '0.75rem',
		padding: '0.25rem',
		color: 'text.muted',
		cursor: 'pointer',
		borderRadius: 'sm',
		transition: 'all 0.2s',
		backgroundColor: 'transparent',
		border: 'none',
		'&:hover': {
			color: 'red.400',
			backgroundColor: 'rgba(239, 68, 68, 0.1)'
		}
	});

	const inputContainerStyles = css({
		display: 'flex',
		flexDirection: 'column',
		gap: '0.75rem',
		padding: '1rem',
		backgroundColor: 'void.900',
		borderRadius: 'md',
		border: '1px solid',
		borderColor: 'border.liminal'
	});

	const textareaStyles = css({
		width: '100%',
		minHeight: '80px',
		padding: '0.75rem',
		backgroundColor: 'void.950',
		border: '1px solid',
		borderColor: 'border.liminal',
		borderRadius: 'md',
		color: 'text.primary',
		fontSize: 'sm',
		lineHeight: '1.5',
		resize: 'vertical',
		fontFamily: 'inherit',
		'&:focus': {
			outline: 'none',
			borderColor: 'border.hover'
		},
		'&::placeholder': {
			color: 'text.muted'
		}
	});

	const inputActionsStyles = css({
		display: 'flex',
		justifyContent: 'space-between',
		alignItems: 'center'
	});

	const charCountStyles = css({
		fontSize: 'xs',
		color: 'text.muted'
	});

	const inputButtonsStyles = css({
		display: 'flex',
		gap: '0.5rem'
	});

	const addButtonStyles = css({
		display: 'flex',
		alignItems: 'center',
		gap: '0.5rem',
		width: '100%',
		justifyContent: 'center'
	});

	const emptyStyles = css({
		textAlign: 'center',
		padding: '2rem',
		color: 'text.muted',
		fontSize: 'sm'
	});
</script>

<div class={notesContainerStyles}>
	<!-- Add Note Input -->
	{#if isAdding}
		<div class={inputContainerStyles}>
			<textarea
				bind:value={newNoteContent}
				placeholder="Write your note... (max 500 characters)"
				maxlength={500}
				class={textareaStyles}
				autofocus
			></textarea>
			<div class={inputActionsStyles}>
				<span class={charCountStyles}>
					{newNoteContent.length}/500
				</span>
				<div class={inputButtonsStyles}>
					<Button
						size="sm"
						variant="outline"
						onclick={() => {
							isAdding = false;
							newNoteContent = '';
						}}
						disabled={saving}
					>
						Cancel
					</Button>
					<Button
						size="sm"
						onclick={addNote}
						disabled={!newNoteContent.trim() || saving}
					>
						{saving ? 'Adding...' : 'Add Note'}
					</Button>
				</div>
			</div>
		</div>
	{:else}
		<Button variant="outline" size="sm" onclick={() => (isAdding = true)}>
			<div class={addButtonStyles}>
				<Plus size={16} />
				Add Note
			</div>
		</Button>
	{/if}

	<!-- Existing Notes -->
	{#if parsedNotes.length === 0 && !isAdding}
		<div class={emptyStyles}>No notes yet. Add your first note above.</div>
	{:else}
		{#each parsedNotes as note, index (note.timestamp)}
			<div class={noteItemStyles}>
				<div class={noteContentStyles}>{note.content}</div>
				<div class={noteTimestampStyles}>{formatTimestamp(note.timestamp)}</div>
				<button
					class={deleteButtonStyles}
					onclick={() => deleteNote(index)}
					disabled={saving}
					title="Delete note"
				>
					<X size={16} />
				</button>
			</div>
		{/each}
	{/if}
</div>
