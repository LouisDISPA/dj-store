<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import RoomTile from '$lib/components/RoomTile.svelte';
	import Table from '$lib/components/Table.svelte';
	import QrCodePopup from '$lib/components/QrCodePopup.svelte';
	import { env, nowPlus, randomRoomID } from '$lib/utils';
	import type { Room, RoomId } from '$lib/types';
	import { createRoom, deleteRoom, getRooms } from '$lib/client';
	import { auth } from '$lib/auth';
	import { onMount } from 'svelte';
	import Hero from '$lib/components/Hero.svelte';
	import TextInput from '$lib/components/TextInput.svelte';

	let rooms: Room[] | undefined;
	let roomUrl: string | undefined;

	// Since the authentification is done in the layout, we can assume that the user is authenticated
	const auth_token = $auth?.access_token as string;

	onMount(pageLoad);

	async function pageLoad() {
		rooms = await getRooms(auth_token);
	}

	async function onDelete(id: RoomId) {
		await deleteRoom(auth_token, id);
		rooms = rooms?.filter((room) => room.id !== id);
	}

	function onShare(id: RoomId) {
		roomUrl = window.location.origin + `${env.BASE_HREF}/r/${id}`;
	}

	function closeShare() {
		roomUrl = undefined;
	}

	let room_id = '';

	async function onCreate() {
		const room = await createRoom(auth_token, room_id ?? randomRoomID(), nowPlus({ days: 1 }));
		rooms?.push(room);
		rooms = rooms;
		room_id = '';
	}
</script>

{#if rooms !== undefined}
	<div class="flex flex-wrap gap-4 m-2">
		{#each rooms as room}
			<RoomTile {...room} {onDelete} {onShare} />
		{/each}
	</div>
	<div class="flex flex-wrap items-end m-4 gap-2">
		<TextInput bind:value={room_id} label="Room ID" />
		<Button label="Create Room" type="primary" onSubmit={onCreate} no_marging />
	</div>
{:else}
	<Hero>
		<h1 class="text-5xl font-bold">Loading...</h1>
	</Hero>
{/if}
{#if roomUrl}
	<QrCodePopup url={roomUrl} onClose={closeShare} />
{/if}
