<script lang="ts">
	import Button from './Button.svelte';
	import { goto } from './utils';

	export let id: string;
	export let user_count: number;
	export let expiration: Date;
	export let creation: Date;
	export let active: boolean;

	export let onDelete: (() => void) | undefined = undefined;
	export let onShare: (() => void) | undefined = undefined;

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
		<div class="text-sm opacity-50">{user_count}</div>
	</td>
	<td>
		<div class="text-sm opacity-50">{timeFormat.format(expiration)}</div>
	</td>
	<td>
		<div class="text-sm opacity-50">{timeFormat.format(creation)}</div>
	</td>
	<td>
		<Button label="GoTo" type="primary" onSubmit={() => goto(`admin/r/${id}`)} />
		<Button label="Share" type="primary" onSubmit={onShare} />
		<Button label="Delete" type="error" onSubmit={onDelete} />
	</td>
</tr>
