<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from './Button.svelte';
	import { loading } from './Loader.svelte';
	import PassCode from './PassCode.svelte';
	let input = '';

	function goToPage() {
		if (input.length !== 6) {
			input = '';
			return;
		}
		loading.set(true);
		goto('/' + input).then(() => {
			loading.set(false);
		});
	}
</script>

<div id="page">
	<h1>Enter a code</h1>
	<PassCode bind:input />
	<Button label="Connect" onSubmit={goToPage} />
	<Button label="Scan QR Code" onSubmit={() => goto('/scan')} />
</div>

<style lang="sass">
div#page
	display: flex
	flex-direction: column
	align-items: center
	justify-content: center
	height: 50vh
</style>
