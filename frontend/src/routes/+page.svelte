<script lang="ts">
//   const links = [
//     { href: "/stations", label: "Stations" },
//     { href: "/scan", label: "Scan QR" },
//     { href: "/game", label: "Game Status" }
//   ];
</script>

<svelte:head>
	<title>Home</title>
	<meta name="index" content="Scottylabs O-Quest Homepage" />
</svelte:head>

<section>
	<h1>Find the Station</h1>
	<p>Welcome to the app prototype.</p>
</section>

<section>
	<h1>QR Code Scanner</h1>
</section>

<section>
	<h1>Scanned Stations</h1>
</section>

<section>
	<h1>Unscanned Stations</h1>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}
</style>
