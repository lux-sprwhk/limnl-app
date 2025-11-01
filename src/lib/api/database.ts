import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';

export const databaseApi = {
	async getDatabasePath(): Promise<string> {
		return await invoke<string>('get_database_path');
	},

	async backupDatabase(destination: string): Promise<void> {
		return await invoke<void>('backup_database', { destination });
	},

	async backupDatabaseWithDialog(): Promise<boolean> {
		try {
			// Get current database path for default filename
			const dbPath = await this.getDatabasePath();
			const defaultFilename = `limnl-backup-${new Date().toISOString().split('T')[0]}.db`;

			// Show save dialog
			const filePath = await save({
				defaultPath: defaultFilename,
				filters: [
					{
						name: 'Database',
						extensions: ['db']
					}
				]
			});

			if (!filePath) {
				// User cancelled
				return false;
			}

			// Perform backup
			await this.backupDatabase(filePath);
			return true;
		} catch (error) {
			console.error('Failed to backup database:', error);
			throw error;
		}
	}
};
