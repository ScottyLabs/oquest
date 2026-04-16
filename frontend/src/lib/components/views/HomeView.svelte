<script lang="ts">
    let scanning = $state(false);
    let error = $state<string | null>(null);

    async function handleScan() {
		if (scanning) return;

		scanning = true;
		error = null;

		try {
			// 1) request camera / scanner
			// 2) scan QR code
			// 3) send scanned value to your Rust/Tauri layer
		} catch (err) {
			error = err instanceof Error ? err.message : 'Scanning failed';
		} finally {
			scanning = false;
		}
	}
</script>

<section class="scan-section">
	<button class="scan-btn" onclick={handleScan}>
		Scan QR Code
	</button>
</section>

<style>

    .scan-section {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .scan-btn {
		width: 180px;
		height: 180px;
		border-radius: 20%;
		border: 1px solid var(--color-border-0);
		color: black;
		background-color: var(--color-bg-0);
		cursor: pointer;
		font-size: var(--text-md);
		font-weight: 600;
		line-height: 1.2;
		text-align: center;
		font-family: var(--font-body);
		font-weight: var(--weight-superbold);

        transition: background 0.6s;

        &:active{
            background-color: var(--color-bg-0-darker);
            transition: background 0.2s;
        }
	}
</style>