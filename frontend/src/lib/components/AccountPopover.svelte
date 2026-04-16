<script lang="ts">
	let {
		open,
		signedIn,
		darkMode,
		onClose,
		onToggleTheme,
		onAuthAction
	} = $props<{
		open: boolean;
		signedIn: boolean;
		darkMode: boolean;
		onClose: () => void;
		onToggleTheme: () => void;
		onAuthAction: () => void;
	}>();
</script>

{#if open}
	<!-- click outside, close -->
	<div class="backdrop" onclick={onClose}></div>

	<div class="popover" role="dialog" aria-label="Account menu">

		<div class="section">
			<p class="status">
				{signedIn ? 'Signed in' : 'Signed out'}
			</p>
			<button class="close-btn" onclick={onClose}>
				<!-- <img src="../images/close.png" title="close button" alt="close"> -->
				 X
			</button>
		</div>

		<button class="menu-btn" onclick={onAuthAction}>
			{signedIn ? 'Sign out' : 'Sign in'}
		</button>

		<button class="menu-btn" onclick={onToggleTheme}>
			{darkMode ? 'Light Mode' : 'Dark Mode'}
		</button>

	</div>
{/if}

<style>
	.backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.2);
		z-index: 20;
	}

	.popover {
		position: fixed;
		top: 4.5rem;
		right: 1rem;
		width: min(260px, calc(100vw - 2rem));
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.75rem;
		border-radius: 16px;
		background: white;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.18);
		z-index: 30;
	}

	.section {
		padding: 0.25rem 0.25rem 0.5rem;
		display: flex;
		justify-content: space-between;
	}

	.status {
		margin: 0;
		font-size: var(--text-sm);
	}

	.menu-btn {
		border: 1px solid #ddd;
		border-radius: 12px;
		padding: 0.75rem 0.9rem;
		background: white;
		text-align: left;
		cursor: pointer;
		font-size: var(--text-md);
	}

	.close-btn{
		background-color: white;
		border: 0;
	}
</style>