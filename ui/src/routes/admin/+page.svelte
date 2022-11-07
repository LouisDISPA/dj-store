<script lang="ts">
	import Button from '$lib/Button.svelte';
	import RoomTile from '$lib/RoomTile.svelte';
	import Table from '$lib/Table.svelte';

	function randomDate() {
		let start = new Date(2020, 0, 1);
		let end = new Date();
		return new Date(start.getTime() + Math.random() * (end.getTime() - start.getTime()));
	}

	function randomID() {
		return Math.random().toString(36).substring(2, 8);
	}

	let rooms = [
		{
			id: randomID(),
			userCount: 2,
			expirationDate: randomDate(),
			creationDate: randomDate()
		},
		{
			id: randomID(),
			userCount: 349,
			expirationDate: randomDate(),
			creationDate: randomDate()
		},
		{
			id: randomID(),
			userCount: 0,
			expirationDate: randomDate(),
			creationDate: randomDate()
		}
	];

	function addRoom() {
		rooms.push({
			id: randomID(),
			userCount: 0,
			expirationDate: randomDate(),
			creationDate: randomDate()
		});
		rooms = rooms;
	}

	const header = ['ID', 'Users', 'Expires', 'Created', 'Actions'];
</script>

<div class="grid-cols-1">
	<Table {header}>
		{#each rooms as room}
			<RoomTile {...room} onDelete={() => (rooms = rooms.filter((r) => r.id != room.id))} />
		{/each}
	</Table>
	<Button label="Create Room" type="primary" onSubmit={addRoom} />
	<input class="bg-base-100 p-4 rounded-box select-none" type="datetime-local" />
</div>
