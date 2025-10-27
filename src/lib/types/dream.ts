export interface Dream {
	id?: number;
	date_recorded: string;
	date_occurred: string;
	title: string;
	content: string;
	emotions_tags?: string;
	sleep_quality?: number;
	created_at: string;
	updated_at: string;
}

export interface CreateDreamInput {
	date_occurred: string;
	title: string;
	content: string;
	emotions_tags?: string;
	sleep_quality?: number;
}

export interface UpdateDreamInput {
	id: number;
	date_occurred?: string;
	title?: string;
	content?: string;
	emotions_tags?: string;
	sleep_quality?: number;
}
