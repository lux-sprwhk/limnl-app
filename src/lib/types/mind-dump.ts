export interface MindDump {
	id?: number;
	title?: string;
	content: string;
	word_count: number;
	created_at: string;
	updated_at: string;
}

export interface CreateMindDumpInput {
	title?: string;
	content: string;
	word_count: number;
}

export interface UpdateMindDumpInput {
	id: number;
	title?: string;
	content?: string;
	word_count?: number;
}
