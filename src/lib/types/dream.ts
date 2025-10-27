export type RecurrenceTimePeriod =
	| 'today'
	| 'yesterday'
	| 'last_week'
	| 'few_weeks_ago'
	| 'last_month'
	| 'months_ago'
	| 'last_year';

export interface Dream {
	id?: number;
	date_recorded: string;
	date_occurred: string;
	title: string;
	content: string;
	emotions_tags?: string;
	sleep_quality?: number;
	is_recurring?: boolean;
	last_occurrence_period?: RecurrenceTimePeriod; // Approximate time period of last occurrence
	is_lucid?: boolean; // Whether the dreamer was aware they were dreaming
	created_at: string;
	updated_at: string;
}

export interface CreateDreamInput {
	date_occurred: string;
	title: string;
	content: string;
	emotions_tags?: string;
	sleep_quality?: number;
	is_recurring?: boolean;
	last_occurrence_period?: RecurrenceTimePeriod;
	is_lucid?: boolean;
}

export interface UpdateDreamInput {
	id: number;
	date_occurred?: string;
	title?: string;
	content?: string;
	emotions_tags?: string;
	sleep_quality?: number;
	is_recurring?: boolean;
	last_occurrence_period?: RecurrenceTimePeriod;
	is_lucid?: boolean;
}

export const RECURRENCE_TIME_PERIODS: { value: RecurrenceTimePeriod; label: string }[] = [
	{ value: 'today', label: 'Today' },
	{ value: 'yesterday', label: 'Yesterday' },
	{ value: 'last_week', label: 'Last week' },
	{ value: 'few_weeks_ago', label: 'A few weeks ago' },
	{ value: 'last_month', label: 'Last month' },
	{ value: 'months_ago', label: 'Months ago' },
	{ value: 'last_year', label: 'Last year' }
];
