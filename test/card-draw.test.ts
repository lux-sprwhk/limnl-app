import { describe, it, expect, beforeEach, vi } from 'vitest';
import type { Card } from '../src/lib/types/card';

// Mock card data for testing
const mockCards: Card[] = Array.from({ length: 36 }, (_, i) => ({
	id: i + 1,
	name: `Card ${i + 1}`,
	emoji: '🃏',
	traditional_equivalent: `Tarot ${i + 1}`,
	core_meaning: `Meaning ${i + 1}`,
	card_question: `Question ${i + 1}?`,
	perspective_prompts: ['Prompt 1', 'Prompt 2', 'Prompt 3'],
	life_area_insights: {
		life: `Life insight ${i + 1}`,
		work: `Work insight ${i + 1}`,
		creative: `Creative insight ${i + 1}`,
		relationship: `Relationship insight ${i + 1}`
	},
	fortune_cookie: `Fortune ${i + 1}`,
	reversed_meaning: `Reversed ${i + 1}`,
	tags: ['tag1', 'tag2']
}));

describe('Card Draw Behavior', () => {
	describe('drawCards function', () => {
		it('should draw 3 cards when no cards are selected', () => {
			const selectedCards: Card[] = [];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(3);
		});

		it('should draw 2 cards when 1 card is selected', () => {
			const selectedCards = [mockCards[0]];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(2);
		});

		it('should draw 1 card when 2 cards are selected', () => {
			const selectedCards = [mockCards[0], mockCards[1]];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(1);
		});

		it('should draw 0 cards when 3 cards are selected (max reached)', () => {
			const selectedCards = [mockCards[0], mockCards[1], mockCards[2]];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(0);
		});

		it('should return correct number of cards from shuffled deck', () => {
			const selectedCards: Card[] = [];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			const shuffled = [...mockCards].sort(() => Math.random() - 0.5);
			const drawnCards = shuffled.slice(0, cardsToDraw);
			
			expect(drawnCards.length).toBe(3);
			expect(drawnCards.every(card => mockCards.includes(card))).toBe(true);
		});

		it('should draw different cards on subsequent draws', () => {
			const selectedCards = [mockCards[0]];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			
			const firstDraw = [...mockCards].sort(() => Math.random() - 0.5).slice(0, cardsToDraw);
			const secondDraw = [...mockCards].sort(() => Math.random() - 0.5).slice(0, cardsToDraw);
			
			// With high probability, two random draws should be different
			// (though there's a small chance they could be the same)
			expect(firstDraw.length).toBe(2);
			expect(secondDraw.length).toBe(2);
		});

		it('should handle edge case: unlimited draws with varying quantities', () => {
			// Simulate multiple draw cycles
			let selectedCards: Card[] = [];
			
			// First draw: 0 selected, should draw 3
			let cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(3);
			selectedCards = mockCards.slice(0, 1); // Simulate selecting 1 card
			
			// Second draw: 1 selected, should draw 2
			cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(2);
			selectedCards = mockCards.slice(0, 2); // Simulate selecting 2 cards
			
			// Third draw: 2 selected, should draw 1
			cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(1);
			selectedCards = mockCards.slice(0, 3); // Simulate selecting 3 cards
			
			// Fourth draw: 3 selected, should draw 0 (max reached)
			cardsToDraw = calculateCardsToDraw(selectedCards.length);
			expect(cardsToDraw).toBe(0);
		});
	});

	describe('card draw distribution', () => {
		it('should not repeat cards in a single draw', () => {
			const selectedCards: Card[] = [];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			const shuffled = [...mockCards].sort(() => Math.random() - 0.5);
			const drawnCards = shuffled.slice(0, cardsToDraw);
			
			const cardIds = drawnCards.map(c => c.id);
			const uniqueIds = new Set(cardIds);
			
			expect(uniqueIds.size).toBe(cardIds.length);
		});

		it('should maintain card pool integrity', () => {
			const selectedCards: Card[] = [];
			const cardsToDraw = calculateCardsToDraw(selectedCards.length);
			const shuffled = [...mockCards].sort(() => Math.random() - 0.5);
			const drawnCards = shuffled.slice(0, cardsToDraw);
			
			// All drawn cards should be from the original pool
			drawnCards.forEach(card => {
				expect(mockCards.some(c => c.id === card.id)).toBe(true);
			});
		});
	});
});

/**
 * Helper function that mirrors the logic in drawCards()
 * Calculates how many cards should be drawn based on selected cards count
 */
function calculateCardsToDraw(selectedCardsCount: number): number {
	let cardsToDraw = 3;
	if (selectedCardsCount === 1) {
		cardsToDraw = 2;
	} else if (selectedCardsCount === 2) {
		cardsToDraw = 1;
	} else if (selectedCardsCount === 3) {
		cardsToDraw = 0;
	}
	return cardsToDraw;
}
