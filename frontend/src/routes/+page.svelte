<script lang="ts">

	import UnscannedView from '$lib/components/views/UnscannedView.svelte';
	import HomeView from '$lib/components/views/HomeView.svelte';
	import ScannedView from '$lib/components/views/ScannedView.svelte';
	import LeaderboardView from '$lib/components/views/LeaderboardView.svelte';

	let currentView = $state(1);
	let touchStartX = $state(0);
	let homeViewIndex = 1; 
	// views are defined from left to right. 

	let views = [
		{ id: 'unscanned', component: UnscannedView },
		{ id: 'home', component: HomeView },
		{ id: 'scanned', component: ScannedView },
		{ id: 'leaderboard', component: LeaderboardView }
	];

	// set initial swipe point
	function onTouchStart(e: TouchEvent) { touchStartX = e.touches[0].clientX;}
	// reaction needs to change based on number of views
	function onTouchEnd(e: TouchEvent) {
		const dx = e.changedTouches[0].clientX - touchStartX;
		if (Math.abs(dx) < 50) return; // ignore small swipes
		if (dx < 0 && currentView < (views.length - 1)) currentView++;  // swipe left → scanned
		if (dx > 0 && currentView > 0) currentView--;  // swipe right → unscanned
	}
	// for desktop testing
	function onMouseStart(e: MouseEvent) { touchStartX = e.clientX;}
	function onMouseEnd(e: MouseEvent) {
		const dx = e.clientX - touchStartX;
		if (Math.abs(dx) < 50) return;
		if (dx < 0 && currentView < (views.length - 1)) currentView++;
		if (dx > 0 && currentView > 0) currentView--;
	}
</script>

<svelte:head>
	<title>Find the Station</title>
	<meta name="index" content="Scottylabs O-Quest Homepage" />
</svelte:head>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div class="page" 
	role="region"
	aria-label="swipeable views"
	ontouchstart={onTouchStart}
	ontouchend={onTouchEnd}
	onmousedown={onMouseStart}
	onmouseup={onMouseEnd}
	>
	<header class="topbar">
		<span class="app-title">O-Quest</span> 
		<!-- <button class="account-btn">Account</button> -->
		 <button class="account-btn">
			<img src="https://i.pravatar.cc/150" alt="Account" />
		</button>
	</header>
	
	<div 
	class="slider" 
	style=
	"width: {views.length * 100}%; 
	margin-left: -{homeViewIndex * 100}%;
	transform: translateX({(1 - currentView) * 100/views.length}%);
	">
		<!-- unpack views -->
		{#each views as view (view.id)}
			{@const ViewComponent = view.component}
			<div class="view" style="width: {100 / views.length}%;">
				<ViewComponent />
			</div>
		{/each}

	</div>

	<div class="dots">
		{#each views as view, i (view.id)}
			<button 
				title="view-indicator"
				class="dot"
				class:active={currentView === i}
				onclick={() => currentView = i}
			></button>
		{/each}
	</div>

</div>

<style>
	:global(body) {
		margin: 0;
		overflow: hidden;
	}

	.page {
		position: fixed;
		inset: 0;
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		background-color: var(--color-bg-0);
		overflow: hidden;
		font-family: inherit;
		font-size: var(--text-md);
		line-height: 1.5;
	}

	.topbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.2rem;
		flex-shrink: 0;
		
	}

	.app-title {
		font-size: var(--text-lg);
		font-weight: 600;
		line-height: 1.2;
	}

	.account-btn {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		border: none;
		padding: 0;
		cursor: pointer;
		overflow: hidden;
	}

	.account-btn img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		display: block;
	}

	.slider {
		display: flex;
		flex: 1;
		min-height: 0;
		transition: transform 0.3s ease;
	}

	.view {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		text-align: center;
		box-sizing: border-box;
	}

	.view h2 {
		font-size: var(--text-xl);
		line-height: 1.15;
	}

	.view p {
		font-size: var(--text-sm);
		line-height: 1.5;
		margin: 0;
	}

	.dots {
		display: flex;
		justify-content: center;
		gap: 8px;
		padding: 1rem 0;
		flex-shrink: 0;
	}

	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		border: none;
		background: grey;
		padding: 0;
		cursor: pointer;
		transition: background 0.2s;
	}

	.dot.active {
		background: black;
	}
</style>