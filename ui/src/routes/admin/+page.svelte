<script lang="ts">
	import Button from '$lib/Button.svelte';
	import RoomTile from '$lib/RoomTile.svelte';
	import Table from '$lib/Table.svelte';
	import { env } from '$lib/utils';
	import type { PageData } from './$types';
	import { convertApiRoom } from './+page';

	export let data: PageData;
	const authToken = data.authToken;

	function randomID(): string {
		let result = '';
		const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
		for (var i = 0; i < 6; i++) {
			result += characters.charAt(Math.floor(Math.random() * characters.length));
		}
		return result;
	}

	let rooms = data.rooms;

	const header = ['ID', 'Users', 'Expires', 'Created', 'Actions'];

	async function deleteRoom(id: string) {
		const res = await fetch(`${env.API_URL}/api/room/${id}`, {
			method: 'DELETE',
			headers: {
				Authorization: `Bearer ${authToken}`
			}
		});
		if (res.ok) {
			rooms = rooms.filter((room) => room.id !== id);
		} else {
			alert(`Failed to delete room: ${await res.text()}`);
		}
	}

	async function createRoom() {
		const expiration = new Date();
		expiration.setHours(expiration.getHours() + 24);

		const body = JSON.stringify({
			id: randomID(),
			expiration: expiration.toISOString()
		});

		const res = await fetch(`${env.API_URL}/api/room`, {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${authToken}`,
				'Content-Type': 'application/json'
			},
			body
		});
		if (res.ok) {
			const apiRoom = await res.json();
			const room = convertApiRoom(apiRoom);
			rooms.push(room);
			rooms = rooms;
		} else {
			alert(`Failed to create room: ${await res.text()}`);
		}
	}
</script>

<div class="grid-cols-1">
	<Table {header}>
		{#each rooms as room}
			<RoomTile {...room} onDelete={() => deleteRoom(room.id)} />
		{/each}
	</Table>
	<Button label="Create Room" type="primary" onSubmit={createRoom} />
</div>
