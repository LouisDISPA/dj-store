<script lang="ts">
	import Hero from '$lib/Hero.svelte';
	import MusicTile from '$lib/MusicTile.svelte';
	import Search from '$lib/Search.svelte';
	import Table from '$lib/Table.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	const { musics } = data;

	async function onSearch(search: string) {
		alert(search);
	}
</script>

<div class="grid-cols-1">
	{#if musics}
		<Search onSubmit={onSearch} />
		<Table>
			{#each musics as music}
				<MusicTile {...music} />
			{/each}
		</Table>
	{:else}
		<Hero>
			<h1 class="text-5xl font-bold">Loading...</h1>
		</Hero>
	{/if}
</div>
