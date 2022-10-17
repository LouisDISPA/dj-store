<script lang="ts">
	import MusicTile from '$lib/MusicTile.svelte';
	import Search from '$lib/Search.svelte';
	import Table from '$lib/Table.svelte';
	import type { Vote } from './+page';

	export let data: { code: string; votes: Array<Vote> };
</script>

<div class="grid-cols-1">
	<Search />
	<Table>
		{#each data.votes as vote}
			<MusicTile {...vote.music} votes={vote.count} />
		{/each}
	</Table>
</div>
