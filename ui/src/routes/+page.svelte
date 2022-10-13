<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Button.svelte';
	import PassCode from '$lib/PassCode.svelte';
	let input = '';
	let loading = false;
	function goToPage() {
		if (input.length !== 6) {
			input = '';
			return;
		}
		loading = true;
		goto('/r/' + input).then(() => {
			loading = false;
		});
	}
</script>

<div id="page">
	<h1 class="text-2xl font-bold p-2">Enter a code</h1>
	<PassCode bind:input onSubmit={goToPage} />
	<div>
		<Button label="Connect" onSubmit={goToPage} {loading} />
		<Button label="Scan QR Code" onSubmit={() => goto('/scan')} disabled />
	</div>
</div>

<style lang="sass">
div#page
	display: flex
	flex-direction: column
	align-items: center
	justify-content: center
	height: 35rem
</style>
