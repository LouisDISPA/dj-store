<script lang="ts">
	import Button from '$lib/Button.svelte';
	import PassCode from '$lib/PassCode.svelte';
	import { goto } from '$lib/utils';
	import { onMount } from 'svelte';
	import { auth, disconnect, joinRoom } from '$lib/auth';
	import { page } from '$app/stores';
	import Modal from '$lib/Modal.svelte';

	let input = '';
	let loading = false;
	let error: string | null = $page.url.searchParams.get('error');

	if (error) {
		setTimeout(() => {
			error = null;
		}, 3000);
		goto('/');
	}

	async function goToPage() {
		if (input.length !== 6) {
			input = '';
			return;
		}
		loading = true;
		try {
			await joinRoom(input);
			await goto(`/r/${input}`);
		} catch (e) {
			loading = false;
			error = "Couldn't connect to room";
			setTimeout(() => {
				error = null;
			}, 3000);
		}
		loading = false;
	}

	function reconnect() {
		const room_id = $auth?.room_id;
		if (room_id) {
			goto(`/r/${room_id}`);
		} else {
			disconnect();
		}
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

{#if $auth?.room_id}
	<Modal>
		<span slot="title">Connected</span>
		<span slot="body">
			You are already connected to room "{$auth?.room_id}". <br />
			Do you want to reconnect?
		</span>
		<span slot="actions">
			<Button label="Cancel" type="primary" onSubmit={disconnect} />
			<Button label="Reconnect" type="primary" onSubmit={reconnect} />
		</span>
	</Modal>
{:else}
	<div id="page">
		<h1 class="text-2xl font-bold p-2">Enter a code</h1>
		<PassCode bind:input onSubmit={goToPage} />
		<div>
			<Button label="Scan QR Code" onSubmit={goToWebcam} disabled={videoAvailable} />
			<Button label="Connect" type="primary" onSubmit={goToPage} {loading} />
		</div>
		{#if error}
			<div class="badge badge-error gap-2">
				{error}
			</div>
		{/if}
	</div>
{/if}

<style lang="sass">
div#page
	display: flex
	flex-direction: column
	align-items: center
	justify-content: flex-end
	height: 400px
</style>
