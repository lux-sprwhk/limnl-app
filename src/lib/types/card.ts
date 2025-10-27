/**
 * Card data types for the card reading system
 * Cards are read-only and used for gaining insights
 */

export interface LifeAreaInsights {
	creative: string;
	work: string;
	life: string;
	relationship: string;
}

export interface Card {
	id: number;
	name: string;
	emoji: string;
	traditional_equivalent: string;
	core_meaning: string;
	card_question: string;
	perspective_prompts: string[];
	life_area_insights: LifeAreaInsights;
	fortune_cookie: string;
	reversed_meaning: string;
	tags: string[];
}

/**
 * Represents a card draw in a reading
 */
export interface CardDraw {
	card: Card;
	reversed: boolean;
	position?: number; // Position in the spread (e.g., 1, 2, 3 for a 3-card spread)
}

/**
 * Represents a complete card reading session
 */
export interface CardReading {
	id: number;
	dream_id?: number; // Optional: link to a specific dream
	cards: CardDraw[];
	notes?: string; // User's personal notes about the reading
	created_at: string;
}

/**
 * Input for creating a new card reading
 */
export interface CreateCardReadingInput {
	dream_id?: number;
	card_ids: number[]; // IDs of cards drawn
	reversed: boolean[]; // Whether each card is reversed
	notes?: string;
}
