import { browser } from '$app/environment';
import type { UserProfile } from '$lib/types/user';
import { DEFAULT_USER_PROFILE } from '$lib/types/user';

const STORAGE_KEY = 'limnl-user-profile';

function loadProfile(): UserProfile {
	if (!browser) return DEFAULT_USER_PROFILE;

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			return { ...DEFAULT_USER_PROFILE, ...JSON.parse(stored) };
		}
	} catch (error) {
		console.error('Failed to load user profile:', error);
	}

	return DEFAULT_USER_PROFILE;
}

function saveProfile(profile: UserProfile): void {
	if (!browser) return;

	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(profile));
	} catch (error) {
		console.error('Failed to save user profile:', error);
	}
}

class UserProfileStore {
	profile = $state<UserProfile>(loadProfile());

	updateProfile(updates: Partial<UserProfile>) {
		this.profile = { ...this.profile, ...updates };
		saveProfile(this.profile);
	}

	resetProfile() {
		this.profile = DEFAULT_USER_PROFILE;
		saveProfile(this.profile);
	}
}

export const userProfile = new UserProfileStore();
