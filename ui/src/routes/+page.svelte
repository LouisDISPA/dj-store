<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from './Button.svelte';
	import PassCode from './PassCode.svelte';
	let input = '';
	let loading = false;

	function goToPage() {
		if (input.length !== 6) {
			input = '';
			return;
		}
		loading = true;
		goto('/' + input).then(() => {
			loading = false;
		});
	}
</script>

<div id="page">
	<h1>Enter a code</h1>
	<form on:submit|preventDefault={goToPage}>
		<PassCode bind:input />
		<Button label="Connect" />
	</form>
	{#if loading}
		<p>loading...</p>
	{/if}
	<Button label="Scan QR Code" onSubmit={() => goto('/scan')} />
</div>

<style lang="sass">
div#page
	display: flex
	flex-direction: column
	align-items: center
	justify-content: center
	height: 50vh
form
	display: flex
	flex-direction: column
	align-items: center
	justify-content: center
</style>
