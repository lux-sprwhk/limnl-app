import { invoke } from '@tauri-apps/api/core';
import type { DbCard, CardWithCount, BugCard } from '$lib/types/card';
import type { Bug, CreateBugInput } from '$lib/types/bug';

/**
 * API client for card operations
 * Cards are read-only - they form a fixed deck like tarot cards
 */
export const cardsApi = {
	/**
	 * Get a single card by ID
	 */
	async get(id: number): Promise<DbCard | null> {
		return await invoke<DbCard | null>('get_card', { id });
	},

	/**
	 * Get a card by its name
	 */
	async getByName(name: string): Promise<DbCard | null> {
		return await invoke<DbCard | null>('get_card_by_name', { name });
	},

	/**
	 * List all cards (alphabetically)
	 */
	async list(): Promise<DbCard[]> {
		return await invoke<DbCard[]>('list_cards');
	},

	/**
	 * List all cards sorted by usage (most used in bugs first)
	 * This is the main method for displaying cards in the UI
	 */
	async listByUsage(): Promise<CardWithCount[]> {
		return await invoke<CardWithCount[]>('list_cards_by_usage');
	},

	/**
	 * Create a bug with associated cards
	 * Card names must exist in the deck
	 */
	async createBugWithCards(input: CreateBugInput, cardNames: string[]): Promise<Bug> {
		return await invoke<Bug>('create_bug_with_cards', { input, cardNames });
	},

	/**
	 * Link a card to an existing bug
	 */
	async linkCardToBug(bugId: number, cardId: number, position?: number): Promise<BugCard> {
		return await invoke<BugCard>('link_card_to_bug', {
			bugId,
			cardId,
			position: position ?? null
		});
	},

	/**
	 * Get all cards associated with a bug
	 */
	async getBugCards(bugId: number): Promise<DbCard[]> {
		return await invoke<DbCard[]>('get_bug_cards', { bugId });
	},

	/**
	 * Remove a card from a bug
	 */
	async unlinkCardFromBug(bugId: number, cardId: number): Promise<boolean> {
		return await invoke<boolean>('unlink_card_from_bug', { bugId, cardId });
	},

	/**
	 * Remove all cards from a bug
	 */
	async clearBugCards(bugId: number): Promise<void> {
		await invoke<void>('clear_bug_cards', { bugId });
	},

	/**
	 * Get all bugs associated with a card
	 */
	async getCardBugs(cardId: number): Promise<Bug[]> {
		return await invoke<Bug[]>('get_card_bugs', { cardId });
	}
};
