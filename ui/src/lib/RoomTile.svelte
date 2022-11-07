<script lang="ts">
	import Button from './Button.svelte';
	import { goto } from './utils';

	export let id: string;
	export let userCount: number;
	export let expirationDate: Date;
	export let creationDate: Date;

	export let onDelete: (() => void) | undefined = undefined;

	const timeFormat = new Intl.DateTimeFormat(undefined, {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: 'numeric',
		minute: 'numeric'
	});
</script>

<tr {id}>
	<td>
		<div class="font-bold">{id}</div>
	</td>
	<td>
		<div class="text-sm opacity-50">{userCount}</div>
	</td>
	<td>
		<div class="text-sm opacity-50">{timeFormat.format(expirationDate)}</div>
	</td>
	<td>
		<div class="text-sm opacity-50">{timeFormat.format(creationDate)}</div>
	</td>
	<td>
		<Button label="GoTo" type="primary" onSubmit={() => goto(`r/${id}`)} />
		<Button label="Delete" type="error" onSubmit={onDelete} />
	</td>
</tr>
