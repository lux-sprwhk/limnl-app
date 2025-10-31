<script lang="ts">
	import { css } from '../../styled-system/css';
	import Button from '$lib/components/ui/Button.svelte';
	import PinInput from '$lib/components/auth/PinInput.svelte';
	import { Moon, BookOpen, Lock } from 'lucide-svelte';
	import { browser } from '$app/environment';
	import { userProfile } from '$lib/stores/user-profile.svelte';
	import { authStore } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';

	let pin = $state('');
	let confirmPin = $state('');
	let pinError = $state('');
	let isSettingUpPin = $state(false);

	// Check authentication state
	$effect(() => {
		if (browser && userProfile.profile.name) {
			// User has profile
			if (!authStore.authState.requirePin) {
				// PIN not required, go straight to dreams
				goto('/dreams');
			} else if (authStore.authState.isAuthenticated) {
				// Already authenticated, go to dreams
				goto('/dreams');
			} else if (authStore.needsPinSetup) {
				// Need to set up PIN
				isSettingUpPin = true;
			}
			// Otherwise, show PIN entry
		}
	});

	async function handlePinSubmit() {
		pinError = '';

		if (isSettingUpPin) {
			// Setting up new PIN
			if (!pin || !confirmPin) {
				pinError = 'Please enter and confirm your PIN';
				return;
			}

			if (pin.length < 4) {
				pinError = 'PIN must be at least 4 digits';
				return;
			}

			if (pin !== confirmPin) {
				pinError = 'PINs do not match';
				return;
			}

			const success = await authStore.setupPin(pin);
			if (success) {
				goto('/dreams');
			} else {
				pinError = 'Failed to set up PIN. Please try again.';
			}
		} else {
			// Verifying existing PIN
			if (!pin) {
				pinError = 'Please enter your PIN';
				return;
			}

			const success = await authStore.authenticate(pin);
			if (success) {
				goto('/dreams');
			} else {
				pinError = 'Incorrect PIN';
				pin = '';
			}
		}
	}

	const containerStyles = css({
		minHeight: '100vh',
		display: 'flex',
		flexDirection: 'column',
		alignItems: 'center',
		justifyContent: 'center',
		padding: '2rem',
		fontFamily: 'system-ui, sans-serif',
		backgroundColor: 'bg.primary'
	});

	const titleStyles = css({
		fontSize: '4xl',
		fontWeight: 'bold',
		marginBottom: '0.5rem',
		color: 'text.primary',
		textAlign: 'center'
	});

	const subtitleStyles = css({
		fontSize: 'xl',
		color: 'text.secondary',
		marginBottom: '3rem',
		textAlign: 'center'
	});

	const cardStyles = css({
		backgroundColor: 'liminal.surface',
		backdropFilter: 'blur(4px)',
		borderRadius: 'xl',
		padding: '3rem',
		boxShadow: 'void',
		border: '1px solid',
		borderColor: 'border.liminal',
		maxWidth: '500px',
		width: '100%',
		textAlign: 'center'
	});

	const iconWrapperStyles = css({
		margin: '0 auto 1.5rem',
		width: '80px',
		height: '80px',
		borderRadius: 'full',
		backgroundColor: 'void.800',
		border: '2px solid',
		borderColor: 'breakthrough.500',
		boxShadow: 'glow',
		display: 'flex',
		alignItems: 'center',
		justifyContent: 'center'
	});

	const descriptionStyles = css({
		fontSize: 'md',
		color: 'text.secondary',
		marginBottom: '2rem',
		lineHeight: '1.6'
	});
</script>

<div class={containerStyles}>
	<div class={cardStyles}>
		{#if authStore.shouldPromptPin || isSettingUpPin}
			<!-- PIN Entry/Setup Screen -->
			<div class={iconWrapperStyles}>
				<Lock size={48} class={css({ color: 'breakthrough.400' })} />
			</div>

			<h2
				class={css({
					fontSize: '2xl',
					fontWeight: 'semibold',
					marginBottom: '1rem',
					color: 'text.primary'
				})}
			>
				{#if isSettingUpPin}
					Set Up Your PIN
				{:else}
					Welcome back, {userProfile.profile.name}!
				{/if}
			</h2>

			<p class={descriptionStyles}>
				{#if isSettingUpPin}
					Create a secure PIN to protect your journal. You can disable this later in Settings.
				{:else}
					Enter your PIN to access your journal
				{/if}
			</p>

			<div class={css({ marginBottom: '1rem' })}>
				<PinInput
					value={pin}
					onchange={(val) => (pin = val)}
					onenter={handlePinSubmit}
					placeholder={isSettingUpPin ? 'Create PIN (4+ digits)' : 'Enter PIN'}
					error={pinError}
					maxLength={6}
				/>
			</div>

			{#if isSettingUpPin}
				<div class={css({ marginBottom: '1.5rem' })}>
					<PinInput
						value={confirmPin}
						onchange={(val) => (confirmPin = val)}
						onenter={handlePinSubmit}
						placeholder="Confirm PIN"
						maxLength={6}
					/>
				</div>
			{/if}

			<div>
				<Button variant="primary" size="lg" onclick={handlePinSubmit}>
					<Lock size={20} />
					{isSettingUpPin ? 'Set Up PIN' : 'Unlock'}
				</Button>
			</div>

			{#if !isSettingUpPin}
				<p
					class={css({
						marginTop: '1rem',
						fontSize: 'sm',
						color: 'text.muted'
					})}
				>
					You can disable PIN in <a
						href="/settings"
						class={css({
							color: 'breakthrough.400',
							textDecoration: 'underline',
							'&:hover': { color: 'breakthrough.300' }
						})}>Settings</a
					>
				</p>
			{/if}
		{:else}
			<!-- Regular Welcome Screen (no PIN) -->
			<div class={iconWrapperStyles}>
				<Moon size={48} class={css({ color: 'breakthrough.400' })} />
			</div>

			<h2
				class={css({
					fontSize: '2xl',
					fontWeight: 'semibold',
					marginBottom: '1rem',
					color: 'text.primary'
				})}
			>
				{#if browser && userProfile.profile.name}
					Welcome back, {userProfile.profile.name}!
				{:else}
					Welcome to Limnl
				{/if}
			</h2>

			<p class={descriptionStyles}>
				Capture your dreams, track patterns, and explore the symbols of your subconscious. All
				your entries are stored locally and privately on your device.
			</p>

			<Button variant="primary" size="lg" onclick={() => (window.location.href = '/dreams')}>
				<BookOpen size={20} />
				Open Limnl
			</Button>

			<Button
				variant="primary"
				size="lg"
				onclick={() => (window.location.href = '/bugs/discover')}
			>
				<BookOpen size={20} />
				New Space
			</Button>
		{/if}
	</div>
</div>
