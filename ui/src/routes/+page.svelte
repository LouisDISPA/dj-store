<script lang="ts">
	import Button from '$lib/Button.svelte';
	import PassCode from '$lib/PassCode.svelte';
	import { goto } from '$lib/utils';
	import { onMount } from 'svelte';
	import { auth, disconnect, joinRoom } from '$lib/auth';
	import Hero from '$lib/Hero.svelte';
	import { page } from '$app/stores';

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
	<Hero>
		<!-- Modal content -->
		<div class="bg-base-100 rounded-lg shadow-lg">
			<!-- Modal header -->
			<h3 class="text-xl font-semibold text-center pt-4 pb-3">Connected</h3>
			<!-- Modal body -->
			<div class="px-5 stack">
				<p class="text-center">
					You are already connected to room "{$auth?.room_id}". <br /> Do you want to reconnect?
				</p>
			</div>
			<!-- Modal footer -->
			<div class="flex justify-between rounded p-3">
				<Button label="Cancel" type="primary" onSubmit={disconnect} />
				<Button label="Reconnect" type="primary" onSubmit={reconnect} />
			</div>
		</div>
	</Hero>
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
