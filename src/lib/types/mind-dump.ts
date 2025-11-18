export interface MindDump {
	id?: number;
	title?: string;
	content: string;
	character_count: number;
	mood_tags?: string; // JSON string of mood tags
	created_at: string;
	updated_at: string;
}

export interface CreateMindDumpInput {
	title?: string;
	content: string;
	character_count: number;
}

export interface UpdateMindDumpInput {
	id: number;
	title?: string;
	content?: string;
	character_count?: number;
}

export interface MindDumpAnalysis {
	id?: number;
	mind_dump_id: number;
	created_at: string;
}

export interface MindDumpAnalysisCard {
	mind_dump_analysis_id: number;
	card_id: number;
	card_name: string;
	relevance_note?: string;
	created_at: string;
}

export interface MindDumpAnalysisTask {
	id?: number;
	mind_dump_analysis_id: number;
	title: string;
	description?: string;
	created_at: string;
}

export interface MindDumpAnalysisWithCardsAndTasks {
	analysis: MindDumpAnalysis;
	cards: MindDumpAnalysisCard[];
	tasks: MindDumpAnalysisTask[];
}
