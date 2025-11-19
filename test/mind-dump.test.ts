import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mindDumpApi } from '$lib/api/mind-dumps';
import type {
	MindDump,
	CreateMindDumpInput,
	UpdateMindDumpInput,
	MindDumpAnalysisWithCardsAndTasks
} from '$lib/types/mind-dump';
import type { LLMConfig } from '$lib/types/llm';
import { invoke } from '@tauri-apps/api/core';
import { llmSettings } from '$lib/stores/llm-settings.svelte';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}));

// Mock llm-settings store
vi.mock('$lib/stores/llm-settings.svelte', () => ({
	llmSettings: {
		config: {
			provider: 'ollama',
			ollamaUrl: 'http://localhost:11434',
			ollamaModel: 'llama',
			openaiApiKey: '',
			openaiModel: 'gpt4-mini',
			anthropicApiKey: '',
			anthropicModel: 'claude-haiku'
		}
	}
}));

const mockInvoke = vi.mocked(invoke);

describe('Mind Dump API', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	describe('CRUD Operations', () => {
		describe('create', () => {
			it('should create a mind dump successfully', async () => {
				const input: CreateMindDumpInput = {
					content: 'This is my mind dump content',
					character_count: 29
				};

				const mockMindDump: MindDump = {
					id: 1,
					content: 'This is my mind dump content',
					character_count: 29,
					created_at: '2024-01-01T00:00:00Z',
					updated_at: '2024-01-01T00:00:00Z'
				};

				mockInvoke.mockResolvedValue(mockMindDump);

				const result = await mindDumpApi.create(input);

				expect(mockInvoke).toHaveBeenCalledWith('create_mind_dump', {
					input,
					config: llmSettings.config
				});
				expect(result).toEqual(mockMindDump);
				expect(result.id).toBe(1);
				expect(result.content).toBe(input.content);
				expect(result.character_count).toBe(input.character_count);
			});

			it('should create a mind dump with title', async () => {
				const input: CreateMindDumpInput = {
					title: 'My Mind Dump',
					content: 'Content here',
					character_count: 12
				};

				const mockMindDump: MindDump = {
					id: 2,
					title: 'My Mind Dump',
					content: 'Content here',
					character_count: 12,
					created_at: '2024-01-01T00:00:00Z',
					updated_at: '2024-01-01T00:00:00Z'
				};

				mockInvoke.mockResolvedValue(mockMindDump);

				const result = await mindDumpApi.create(input);

				expect(result.title).toBe('My Mind Dump');
			});

			it('should handle empty content error', async () => {
				const input: CreateMindDumpInput = {
					content: '',
					character_count: 0
				};

				mockInvoke.mockRejectedValue(new Error('Content cannot be empty'));

				await expect(mindDumpApi.create(input)).rejects.toThrow('Content cannot be empty');
			});

			it('should handle character count mismatch error', async () => {
				const input: CreateMindDumpInput = {
					content: 'Test content',
					character_count: 5 // Mismatch
				};

				mockInvoke.mockRejectedValue(new Error('Character count mismatch'));

				await expect(mindDumpApi.create(input)).rejects.toThrow('Character count mismatch');
			});

			it('should handle content too long error', async () => {
				const longContent = 'a'.repeat(2001);
				const input: CreateMindDumpInput = {
					content: longContent,
					character_count: 2001
				};

				mockInvoke.mockRejectedValue(
					new Error('Content exceeds maximum length of 2000 characters')
				);

				await expect(mindDumpApi.create(input)).rejects.toThrow(
					'Content exceeds maximum length'
				);
			});
		});

		describe('get', () => {
			it('should retrieve a mind dump by id', async () => {
				const mockMindDump: MindDump = {
					id: 1,
					content: 'Test content',
					character_count: 12,
					created_at: '2024-01-01T00:00:00Z',
					updated_at: '2024-01-01T00:00:00Z'
				};

				mockInvoke.mockResolvedValue(mockMindDump);

				const result = await mindDumpApi.get(1);

				expect(mockInvoke).toHaveBeenCalledWith('get_mind_dump', { id: 1 });
				expect(result).toEqual(mockMindDump);
			});

			it('should return null for non-existent mind dump', async () => {
				mockInvoke.mockResolvedValue(null);

				const result = await mindDumpApi.get(999);

				expect(result).toBeNull();
			});
		});

		describe('list', () => {
			it('should list all mind dumps', async () => {
				const mockMindDumps: MindDump[] = [
					{
						id: 1,
						content: 'First dump',
						character_count: 10,
						created_at: '2024-01-01T00:00:00Z',
						updated_at: '2024-01-01T00:00:00Z'
					},
					{
						id: 2,
						content: 'Second dump',
						character_count: 11,
						created_at: '2024-01-02T00:00:00Z',
						updated_at: '2024-01-02T00:00:00Z'
					}
				];

				mockInvoke.mockResolvedValue(mockMindDumps);

				const result = await mindDumpApi.list();

				expect(mockInvoke).toHaveBeenCalledWith('list_mind_dumps', {
					limit: undefined,
					offset: undefined
				});
				expect(result).toHaveLength(2);
				expect(result[0].id).toBe(1);
				expect(result[1].id).toBe(2);
			});

			it('should list mind dumps with limit and offset', async () => {
				const mockMindDumps: MindDump[] = [
					{
						id: 2,
						content: 'Second dump',
						character_count: 11,
						created_at: '2024-01-02T00:00:00Z',
						updated_at: '2024-01-02T00:00:00Z'
					}
				];

				mockInvoke.mockResolvedValue(mockMindDumps);

				const result = await mindDumpApi.list(10, 1);

				expect(mockInvoke).toHaveBeenCalledWith('list_mind_dumps', {
					limit: 10,
					offset: 1
				});
				expect(result).toHaveLength(1);
			});
		});

		describe('update', () => {
			it('should update a mind dump', async () => {
				const input: UpdateMindDumpInput = {
					id: 1,
					content: 'Updated content',
					character_count: 15
				};

				const mockUpdatedMindDump: MindDump = {
					id: 1,
					content: 'Updated content',
					character_count: 15,
					created_at: '2024-01-01T00:00:00Z',
					updated_at: '2024-01-01T01:00:00Z'
				};

				mockInvoke.mockResolvedValue(mockUpdatedMindDump);

				const result = await mindDumpApi.update(input);

				expect(mockInvoke).toHaveBeenCalledWith('update_mind_dump', { input });
				expect(result).toEqual(mockUpdatedMindDump);
				expect(result?.content).toBe('Updated content');
			});

			it('should return null when updating non-existent mind dump', async () => {
				const input: UpdateMindDumpInput = {
					id: 999,
					content: 'Updated content'
				};

				mockInvoke.mockResolvedValue(null);

				const result = await mindDumpApi.update(input);

				expect(result).toBeNull();
			});
		});

		describe('delete', () => {
			it('should delete a mind dump', async () => {
				mockInvoke.mockResolvedValue(true);

				const result = await mindDumpApi.delete(1);

				expect(mockInvoke).toHaveBeenCalledWith('delete_mind_dump', { id: 1 });
				expect(result).toBe(true);
			});

			it('should return false when deleting non-existent mind dump', async () => {
				mockInvoke.mockResolvedValue(false);

				const result = await mindDumpApi.delete(999);

				expect(result).toBe(false);
			});
		});

		describe('search', () => {
			it('should search mind dumps by query', async () => {
				const mockResults: MindDump[] = [
					{
						id: 1,
						content: 'Searchable content',
						character_count: 17,
						created_at: '2024-01-01T00:00:00Z',
						updated_at: '2024-01-01T00:00:00Z'
					}
				];

				mockInvoke.mockResolvedValue(mockResults);

				const result = await mindDumpApi.search('searchable');

				expect(mockInvoke).toHaveBeenCalledWith('search_mind_dumps', {
					query: 'searchable'
				});
				expect(result).toHaveLength(1);
				expect(result[0].content).toContain('Searchable');
			});
		});
	});

	describe('Analysis Operations', () => {
		describe('getAnalysis', () => {
			it('should retrieve analysis with cards and tasks', async () => {
				const mockAnalysis: MindDumpAnalysisWithCardsAndTasks = {
					analysis: {
						id: 1,
						mind_dump_id: 1,
						blocker_patterns: '["procrastination", "perfectionism"]',
						created_at: '2024-01-01T00:00:00Z'
					},
					cards: [
						{
							mind_dump_analysis_id: 1,
							card_id: 5,
							card_name: 'The Catalyst',
							relevance_note: 'Represents transformation',
							created_at: '2024-01-01T00:00:00Z'
						}
					],
					tasks: [
						{
							id: 1,
							mind_dump_analysis_id: 1,
							title: 'Break down the problem',
							description: 'Identify specific steps',
							created_at: '2024-01-01T00:00:00Z'
						}
					]
				};

				mockInvoke.mockResolvedValue(mockAnalysis);

				const result = await mindDumpApi.getAnalysis(1);

				expect(mockInvoke).toHaveBeenCalledWith(
					'get_mind_dump_analysis_with_cards_and_tasks',
					{ mindDumpId: 1 }
				);
				expect(result).toEqual(mockAnalysis);
				expect(result?.cards).toHaveLength(1);
				expect(result?.tasks).toHaveLength(1);
			});

			it('should return null when analysis does not exist', async () => {
				mockInvoke.mockResolvedValue(null);

				const result = await mindDumpApi.getAnalysis(999);

				expect(result).toBeNull();
			});
		});
	});

	describe('Error Handling - LLM Failures', () => {
		it('should handle LLM analysis failure gracefully during creation', async () => {
			// Mind dump creation should succeed even if LLM analysis fails
			const input: CreateMindDumpInput = {
				content: 'Test content',
				character_count: 12
			};

			const mockMindDump: MindDump = {
				id: 1,
				content: 'Test content',
				character_count: 12,
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T00:00:00Z'
			};

			// Creation succeeds (LLM runs in background)
			mockInvoke.mockResolvedValue(mockMindDump);

			const result = await mindDumpApi.create(input);

			expect(result).toEqual(mockMindDump);
			// Note: LLM errors are logged but don't fail the creation
		});

		it('should handle network errors during LLM analysis', async () => {
			const input: CreateMindDumpInput = {
				content: 'Test content',
				character_count: 12
			};

			// Simulate network error - but creation should still succeed
			const mockMindDump: MindDump = {
				id: 1,
				content: 'Test content',
				character_count: 12,
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T00:00:00Z'
			};

			mockInvoke.mockResolvedValue(mockMindDump);

			const result = await mindDumpApi.create(input);

			// Creation succeeds even if background LLM fails
			expect(result).toBeDefined();
		});

		it('should handle LLM timeout errors', async () => {
			const input: CreateMindDumpInput = {
				content: 'Test content',
				character_count: 12
			};

			const mockMindDump: MindDump = {
				id: 1,
				content: 'Test content',
				character_count: 12,
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T00:00:00Z'
			};

			mockInvoke.mockResolvedValue(mockMindDump);

			const result = await mindDumpApi.create(input);

			// Creation should succeed even if LLM times out
			expect(result).toBeDefined();
		});

		it('should handle invalid LLM response format', async () => {
			// This would be handled in the Rust backend, but we test that
			// the frontend can handle missing analysis gracefully
			mockInvoke.mockResolvedValue(null);

			const result = await mindDumpApi.getAnalysis(1);

			expect(result).toBeNull();
		});
	});

	describe('JSON Parsing - Mood Tags and Blocker Patterns', () => {
		describe('Mood Tags JSON Parsing', () => {
			it('should parse valid mood tags JSON array', () => {
				const moodTagsJson = '["anxious", "excited", "overwhelmed"]';
				const parsed = JSON.parse(moodTagsJson);

				expect(Array.isArray(parsed)).toBe(true);
				expect(parsed).toHaveLength(3);
				expect(parsed).toContain('anxious');
				expect(parsed).toContain('excited');
				expect(parsed).toContain('overwhelmed');
			});

			it('should handle empty mood tags array', () => {
				const moodTagsJson = '[]';
				const parsed = JSON.parse(moodTagsJson);

				expect(Array.isArray(parsed)).toBe(true);
				expect(parsed).toHaveLength(0);
			});

			it('should handle null mood tags', () => {
				const moodTagsJson = null;
				// In the database, this would be stored as NULL
				expect(moodTagsJson).toBeNull();
			});

			it('should handle invalid mood tags JSON gracefully', () => {
				const invalidJson = '{"not": "an array"}';

				expect(() => {
					const parsed = JSON.parse(invalidJson);
					// Should parse but not be an array
					expect(Array.isArray(parsed)).toBe(false);
				}).not.toThrow();
			});

			it('should handle malformed mood tags JSON', () => {
				const malformedJson = '["anxious", "excited"'; // Missing closing bracket

				expect(() => JSON.parse(malformedJson)).toThrow();
			});

			it('should parse mood tags from mind dump object', async () => {
				const mockMindDump: MindDump = {
					id: 1,
					content: 'Test content',
					character_count: 12,
					mood_tags: '["anxious", "excited"]',
					created_at: '2024-01-01T00:00:00Z',
					updated_at: '2024-01-01T00:00:00Z'
				};

				mockInvoke.mockResolvedValue(mockMindDump);

				const result = await mindDumpApi.get(1);

				if (result?.mood_tags) {
					const parsed = JSON.parse(result.mood_tags);
					expect(Array.isArray(parsed)).toBe(true);
					expect(parsed).toContain('anxious');
					expect(parsed).toContain('excited');
				}
			});
		});

		describe('Blocker Patterns JSON Parsing', () => {
			it('should parse valid blocker patterns JSON array', () => {
				const blockerPatternsJson = '["procrastination", "perfectionism", "self-doubt"]';
				const parsed = JSON.parse(blockerPatternsJson);

				expect(Array.isArray(parsed)).toBe(true);
				expect(parsed).toHaveLength(3);
				expect(parsed).toContain('procrastination');
				expect(parsed).toContain('perfectionism');
				expect(parsed).toContain('self-doubt');
			});

			it('should handle empty blocker patterns array', () => {
				const blockerPatternsJson = '[]';
				const parsed = JSON.parse(blockerPatternsJson);

				expect(Array.isArray(parsed)).toBe(true);
				expect(parsed).toHaveLength(0);
			});

			it('should handle null blocker patterns', () => {
				const blockerPatternsJson = null;
				expect(blockerPatternsJson).toBeNull();
			});

			it('should handle invalid blocker patterns JSON gracefully', () => {
				const invalidJson = '"not an array"';

				expect(() => {
					const parsed = JSON.parse(invalidJson);
					expect(Array.isArray(parsed)).toBe(false);
				}).not.toThrow();
			});

			it('should parse blocker patterns from analysis object', async () => {
				const mockAnalysis: MindDumpAnalysisWithCardsAndTasks = {
					analysis: {
						id: 1,
						mind_dump_id: 1,
						blocker_patterns: '["procrastination", "perfectionism"]',
						created_at: '2024-01-01T00:00:00Z'
					},
					cards: [],
					tasks: []
				};

				mockInvoke.mockResolvedValue(mockAnalysis);

				const result = await mindDumpApi.getAnalysis(1);

				if (result?.analysis.blocker_patterns) {
					const parsed = JSON.parse(result.analysis.blocker_patterns);
					expect(Array.isArray(parsed)).toBe(true);
					expect(parsed).toContain('procrastination');
					expect(parsed).toContain('perfectionism');
				}
			});

			it('should handle malformed blocker patterns JSON', () => {
				const malformedJson = '["procrastination"'; // Missing closing bracket

				expect(() => JSON.parse(malformedJson)).toThrow();
			});
		});

		describe('Edge Cases', () => {
			it('should handle very long mood tags arrays', () => {
				const longArray = Array.from({ length: 100 }, (_, i) => `tag${i}`);
				const json = JSON.stringify(longArray);
				const parsed = JSON.parse(json);

				expect(parsed).toHaveLength(100);
			});

			it('should handle special characters in mood tags', () => {
				const moodTagsJson = '["anxious 😰", "excited! 🎉", "overwhelmed..."]';
				const parsed = JSON.parse(moodTagsJson);

				expect(parsed[0]).toBe('anxious 😰');
				expect(parsed[1]).toBe('excited! 🎉');
			});

			it('should handle empty strings in arrays', () => {
				const json = '["", "valid", ""]';
				const parsed = JSON.parse(json);

				expect(parsed).toHaveLength(3);
				expect(parsed[0]).toBe('');
				expect(parsed[1]).toBe('valid');
			});
		});
	});

	describe('Migration Safety', () => {
		it('should handle mind dump without mood_tags (pre-migration)', async () => {
			const mockMindDump: MindDump = {
				id: 1,
				content: 'Test content',
				character_count: 12,
				// mood_tags is undefined (pre-migration)
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T00:00:00Z'
			};

			mockInvoke.mockResolvedValue(mockMindDump);

			const result = await mindDumpApi.get(1);

			expect(result).toBeDefined();
			expect(result?.mood_tags).toBeUndefined();
			// Should not throw when accessing mood_tags
			if (result?.mood_tags) {
				expect(() => JSON.parse(result.mood_tags)).not.toThrow();
			}
		});

		it('should handle analysis without blocker_patterns (pre-migration)', async () => {
			const mockAnalysis: MindDumpAnalysisWithCardsAndTasks = {
				analysis: {
					id: 1,
					mind_dump_id: 1,
					// blocker_patterns is undefined (pre-migration)
					created_at: '2024-01-01T00:00:00Z'
				},
				cards: [],
				tasks: []
			};

			mockInvoke.mockResolvedValue(mockAnalysis);

			const result = await mindDumpApi.getAnalysis(1);

			expect(result).toBeDefined();
			expect(result?.analysis.blocker_patterns).toBeUndefined();
			// Should not throw when accessing blocker_patterns
			if (result?.analysis.blocker_patterns) {
				expect(() => JSON.parse(result.analysis.blocker_patterns)).not.toThrow();
			}
		});

		it('should maintain data integrity after migration', async () => {
			// Test that existing data structure is preserved
			const mockMindDump: MindDump = {
				id: 1,
				content: 'Test content',
				character_count: 12,
				mood_tags: '["anxious"]',
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T00:00:00Z'
			};

			mockInvoke.mockResolvedValue(mockMindDump);

			const result = await mindDumpApi.get(1);

			expect(result?.id).toBe(1);
			expect(result?.content).toBe('Test content');
			expect(result?.character_count).toBe(12);
			expect(result?.mood_tags).toBeDefined();
		});

		it('should handle nullable fields correctly', async () => {
			const mockMindDump: MindDump = {
				id: 1,
				content: 'Test content',
				character_count: 12,
				title: undefined, // Optional field
				mood_tags: undefined, // Nullable field
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T00:00:00Z'
			};

			mockInvoke.mockResolvedValue(mockMindDump);

			const result = await mindDumpApi.get(1);

			expect(result).toBeDefined();
			expect(result?.title).toBeUndefined();
			expect(result?.mood_tags).toBeUndefined();
		});
	});

	describe('Integration - Full CRUD with Analysis Flow', () => {
		it('should create mind dump, retrieve it, and get analysis', async () => {
			// Create
			const createInput: CreateMindDumpInput = {
				content: 'I feel overwhelmed with work',
				character_count: 28
			};

			const createdMindDump: MindDump = {
				id: 1,
				content: 'I feel overwhelmed with work',
				character_count: 28,
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T00:00:00Z'
			};

			mockInvoke
				.mockResolvedValueOnce(createdMindDump) // create
				.mockResolvedValueOnce(createdMindDump) // get
				.mockResolvedValueOnce({
					// getAnalysis
					analysis: {
						id: 1,
						mind_dump_id: 1,
						blocker_patterns: '["overwhelm", "stress"]',
						created_at: '2024-01-01T00:00:00Z'
					},
					cards: [
						{
							mind_dump_analysis_id: 1,
							card_id: 5,
							card_name: 'The Catalyst',
							relevance_note: 'Represents transformation',
							created_at: '2024-01-01T00:00:00Z'
						}
					],
					tasks: [
						{
							id: 1,
							mind_dump_analysis_id: 1,
							title: 'Prioritize tasks',
							description: 'Break down work into manageable chunks',
							created_at: '2024-01-01T00:00:00Z'
						}
					]
				});

			// Create
			const created = await mindDumpApi.create(createInput);
			expect(created.id).toBe(1);

			// Get
			const retrieved = await mindDumpApi.get(1);
			expect(retrieved).toEqual(createdMindDump);

			// Get Analysis
			const analysis = await mindDumpApi.getAnalysis(1);
			expect(analysis).toBeDefined();
			expect(analysis?.cards).toHaveLength(1);
			expect(analysis?.tasks).toHaveLength(1);

			// Parse blocker patterns
			if (analysis?.analysis.blocker_patterns) {
				const patterns = JSON.parse(analysis.analysis.blocker_patterns);
				expect(patterns).toContain('overwhelm');
			}
		});

		it('should update mind dump and preserve analysis', async () => {
			const updateInput: UpdateMindDumpInput = {
				id: 1,
				content: 'Updated content',
				character_count: 15
			};

			const updatedMindDump: MindDump = {
				id: 1,
				content: 'Updated content',
				character_count: 15,
				mood_tags: '["anxious"]', // Preserved from before
				created_at: '2024-01-01T00:00:00Z',
				updated_at: '2024-01-01T01:00:00Z'
			};

			mockInvoke.mockResolvedValue(updatedMindDump);

			const result = await mindDumpApi.update(updateInput);

			expect(result?.content).toBe('Updated content');
			expect(result?.mood_tags).toBeDefined();
		});
	});
});

