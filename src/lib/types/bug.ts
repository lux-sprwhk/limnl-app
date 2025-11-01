export interface Bug {
	id?: number;
	title: string;
	description: string;
	status: 'active' | 'resolved' | 'archived';
	cards_drawn?: string; // JSON string of card IDs
	conversation_history?: string; // JSON string of conversation messages
	notes?: string; // JSON string of notes array
	created_at: string;
	updated_at: string;
	resolved_at?: string;
}

export interface CreateBugInput {
	title: string;
	description: string;
	cards_drawn?: string;
	conversation_history?: string;
	notes?: string;
}

export interface UpdateBugInput {
	id: number;
	title?: string;
	description?: string;
	status?: 'active' | 'resolved' | 'archived';
	cards_drawn?: string;
	conversation_history?: string;
	notes?: string;
	resolved_at?: string;
}

export interface ConversationMessage {
	role: 'user' | 'assistant';
	content: string;
	timestamp: string;
}

export interface BugNote {
	content: string;
	timestamp: string;
}
