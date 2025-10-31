import { browser } from '$app/environment';
import type { PinConfig, AuthState, AuthSession } from '$lib/types/auth';

const PIN_STORAGE_KEY = 'limnl-pin-config';
const SESSION_STORAGE_KEY = 'limnl-auth-session';
const SESSION_TIMEOUT = 1000 * 60 * 60; // 1 hour

/**
 * Simple hash function for PIN (using Web Crypto API)
 * In production, consider using a proper library like bcrypt.js
 */
async function hashPin(pin: string): Promise<string> {
	const encoder = new TextEncoder();
	const data = encoder.encode(pin);
	const hashBuffer = await crypto.subtle.digest('SHA-256', data);
	const hashArray = Array.from(new Uint8Array(hashBuffer));
	return hashArray.map((b) => b.toString(16).padStart(2, '0')).join('');
}

/**
 * Verify a PIN against the stored hash
 */
async function verifyPin(pin: string, hash: string): Promise<boolean> {
	const inputHash = await hashPin(pin);
	return inputHash === hash;
}

class AuthStore {
	private state = $state<AuthState>({
		isAuthenticated: false,
		requirePin: false,
		hasPin: false
	});

	constructor() {
		if (browser) {
			this.loadPinConfig();
			this.checkSession();
		}
	}

	/**
	 * Load PIN configuration from localStorage
	 */
	private loadPinConfig(): void {
		try {
			const stored = localStorage.getItem(PIN_STORAGE_KEY);
			if (stored) {
				const config: PinConfig = JSON.parse(stored);
				this.state.requirePin = config.enabled;
				this.state.hasPin = !!config.hash;
			}
		} catch (error) {
			console.error('Failed to load PIN config:', error);
		}
	}

	/**
	 * Check if there's a valid session
	 */
	private checkSession(): void {
		try {
			const stored = sessionStorage.getItem(SESSION_STORAGE_KEY);
			if (stored) {
				const session: AuthSession = JSON.parse(stored);
				const now = Date.now();

				// Check if session is still valid
				if (session.authenticated && now - session.timestamp < SESSION_TIMEOUT) {
					this.state.isAuthenticated = true;
				} else {
					// Session expired
					sessionStorage.removeItem(SESSION_STORAGE_KEY);
					this.state.isAuthenticated = false;
				}
			}
		} catch (error) {
			console.error('Failed to check session:', error);
		}
	}

	/**
	 * Save session to sessionStorage
	 */
	private saveSession(): void {
		try {
			const session: AuthSession = {
				authenticated: true,
				timestamp: Date.now()
			};
			sessionStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(session));
		} catch (error) {
			console.error('Failed to save session:', error);
		}
	}

	/**
	 * Get current auth state (reactive)
	 */
	get authState(): AuthState {
		return this.state;
	}

	/**
	 * Check if user should be prompted for PIN
	 */
	get shouldPromptPin(): boolean {
		return this.state.requirePin && this.state.hasPin && !this.state.isAuthenticated;
	}

	/**
	 * Check if user needs to set up PIN for the first time
	 */
	get needsPinSetup(): boolean {
		return this.state.requirePin && !this.state.hasPin;
	}

	/**
	 * Set up a new PIN
	 */
	async setupPin(pin: string): Promise<boolean> {
		try {
			const hash = await hashPin(pin);
			const config: PinConfig = {
				enabled: true,
				hash
			};
			localStorage.setItem(PIN_STORAGE_KEY, JSON.stringify(config));

			this.state.hasPin = true;
			this.state.requirePin = true;
			this.state.isAuthenticated = true;
			this.saveSession();

			return true;
		} catch (error) {
			console.error('Failed to setup PIN:', error);
			return false;
		}
	}

	/**
	 * Verify and authenticate with PIN
	 */
	async authenticate(pin: string): Promise<boolean> {
		try {
			const stored = localStorage.getItem(PIN_STORAGE_KEY);
			if (!stored) return false;

			const config: PinConfig = JSON.parse(stored);
			if (!config.hash) return false;

			const isValid = await verifyPin(pin, config.hash);
			if (isValid) {
				this.state.isAuthenticated = true;
				this.saveSession();
				return true;
			}
			return false;
		} catch (error) {
			console.error('Failed to authenticate:', error);
			return false;
		}
	}

	/**
	 * Change the PIN (requires current PIN)
	 */
	async changePin(currentPin: string, newPin: string): Promise<boolean> {
		try {
			// Verify current PIN first
			const isValid = await this.authenticate(currentPin);
			if (!isValid) return false;

			// Set new PIN
			const hash = await hashPin(newPin);
			const config: PinConfig = {
				enabled: true,
				hash
			};
			localStorage.setItem(PIN_STORAGE_KEY, JSON.stringify(config));

			return true;
		} catch (error) {
			console.error('Failed to change PIN:', error);
			return false;
		}
	}

	/**
	 * Enable or disable PIN requirement
	 */
	async togglePinRequirement(enabled: boolean, currentPin?: string): Promise<boolean> {
		try {
			const stored = localStorage.getItem(PIN_STORAGE_KEY);
			let config: PinConfig = { enabled, hash: undefined };

			if (stored) {
				const existingConfig: PinConfig = JSON.parse(stored);

				// If disabling, verify current PIN first
				if (!enabled && existingConfig.hash && currentPin) {
					const isValid = await verifyPin(currentPin, existingConfig.hash);
					if (!isValid) return false;
				}

				config.hash = existingConfig.hash;
			}

			config.enabled = enabled;
			localStorage.setItem(PIN_STORAGE_KEY, JSON.stringify(config));

			this.state.requirePin = enabled;

			// If enabling PIN and no PIN set, remain authenticated
			// If disabling PIN, grant authentication
			if (!enabled || !config.hash) {
				this.state.isAuthenticated = true;
				this.saveSession();
			}

			return true;
		} catch (error) {
			console.error('Failed to toggle PIN requirement:', error);
			return false;
		}
	}

	/**
	 * Log out (clear session)
	 */
	logout(): void {
		sessionStorage.removeItem(SESSION_STORAGE_KEY);
		this.state.isAuthenticated = false;
	}

	/**
	 * Clear all PIN data (for testing/reset)
	 */
	clearPinData(): void {
		localStorage.removeItem(PIN_STORAGE_KEY);
		sessionStorage.removeItem(SESSION_STORAGE_KEY);
		this.state.hasPin = false;
		this.state.requirePin = false;
		this.state.isAuthenticated = false;
	}
}

export const authStore = new AuthStore();
