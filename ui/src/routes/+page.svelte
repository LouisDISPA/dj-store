<script lang="ts">
	import Button from '$lib/Button.svelte';
	import PassCode from '$lib/PassCode.svelte';
	import { goto } from '$lib/utils';
	import { onMount } from 'svelte';
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

	let videoAvailable = true;
	onMount(async () => {
		const devices = await navigator.mediaDevices.enumerateDevices();
		videoAvailable = !devices.some((device) => device.kind === 'videoinput');
	});

	function goToWebcam() {
		goto('/scan');
	}
</script>

<div id="page">
	<h1 class="text-2xl font-bold p-2">Enter a code</h1>
	<PassCode bind:input onSubmit={goToPage} />
	<div>
		<Button label="Scan QR Code" onSubmit={goToWebcam} disabled={videoAvailable} />
		<Button label="Connect" type="primary" onSubmit={goToPage} {loading} />
	</div>
</div>

<style lang="sass">
div#page
	display: flex
	flex-direction: column
	align-items: center
	justify-content: flex-end
	height: 400px
</style>
