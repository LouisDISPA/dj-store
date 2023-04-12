<script lang="ts">
	import Button from './Button.svelte';
	import type { RoomId } from '$lib/types';
	import { goto, timeFormat } from '$lib/utils';

	export let id: RoomId;
	export let user_count: number;
	export let expiration: Date;
	export let creation: Date;

	export let onDelete: ((id: RoomId) => void) | undefined = undefined;
	export let onShare: ((id: RoomId) => void) | undefined = undefined;
</script>

<div class="card w-96 bg-base-100 shadow-xl">
	<div class="card-body">
		<h2 class="card-title">{id}</h2>
		<p>Creation: {timeFormat.format(creation)}</p>
		<p>Expiration: {timeFormat.format(expiration)}</p>
		<p>Users: {user_count}</p>
		<div class="card-actions justify-end">
			<Button label="GoTo" type="primary" onSubmit={() => goto(`admin/r/${id}`)} />
			<Button label="Share" type="primary" onSubmit={() => onShare?.(id)} />
			<Button label="Delete" type="error" onSubmit={() => onDelete?.(id)} />
		</div>
	</div>
</div>
