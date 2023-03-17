<script lang="ts">
	import Button from '$lib/Button.svelte';
	import RoomTile from '$lib/RoomTile.svelte';
	import Table from '$lib/Table.svelte';
	import QrCodePopup from '$lib/QrCodePopup.svelte';
	import { env, nowPlus, randomRoomID } from '$lib/utils';
	import type { Room } from '$lib/types';
	import { createRoom, deleteRoom, getRooms } from '$lib/client';
	import { auth } from '$lib/auth';
	import { onMount } from 'svelte';
	import Hero from '$lib/Hero.svelte';

	let rooms: Room[] | undefined;
	let roomUrl: string | undefined;

	// Since the authentification is done in the layout, we can assume that the user is authenticated
	const auth_token = $auth?.access_token as string;

	const header = ['ID', 'Users', 'Expires', 'Created', 'Actions'];

	onMount(pageLoad);

	async function pageLoad() {
		rooms = await getRooms(auth_token);
	}

	async function onDelete(id: string) {
		await deleteRoom(auth_token, id);
		rooms = rooms?.filter((room) => room.id !== id);
	}

	function onShare(id: string) {
		roomUrl = window.location.origin + `${env.BASE_HREF}/r/${id}`;
	}

	function closeShare() {
		roomUrl = undefined;
	}

	async function onCreate() {
		const room = await createRoom(auth_token, randomRoomID(), nowPlus({ days: 1 }));
		rooms?.push(room);
		rooms = rooms;
	}
</script>

<div class="grid-cols-1">
	{#if rooms !== undefined}
		<Table {header}>
			{#each rooms as room}
				<RoomTile {...room} {onDelete} {onShare} />
			{/each}
		</Table>
		<Button label="Create Room" type="primary" onSubmit={onCreate} />
	{:else}
		<Hero>
			<h1 class="text-5xl font-bold">Loading...</h1>
		</Hero>
	{/if}
	{#if roomUrl}
		<QrCodePopup url={roomUrl} onClose={closeShare} />
	{/if}
</div>
