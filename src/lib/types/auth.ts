export interface AuthState {
	isAuthenticated: boolean;
	requirePin: boolean;
	hasPin: boolean;
}

export interface PinConfig {
	enabled: boolean;
	hash?: string; // Hashed PIN
}

export interface AuthSession {
	authenticated: boolean;
	timestamp: number;
}
