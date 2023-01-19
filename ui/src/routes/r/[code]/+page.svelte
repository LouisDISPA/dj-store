<script lang="ts">
	import Hero from '$lib/Hero.svelte';
	import MusicTile from '$lib/MusicTile.svelte';
	import Search from '$lib/Search.svelte';
	import Table from '$lib/Table.svelte';
	import { env, goto } from '$lib/utils';
	import type { PageData } from './$types';

	export let data: PageData;
	const { musics, authToken, roomCode } = data;

	async function onSearch(search: string)  {
		console.log('onSearch');
		
		await goto(`/r/${roomCode}/search?query=${search}`);
	}

	async function onVote(is_voted: boolean, id: number) {
		const response = await fetch(`${env.API_URL}/api/room/${roomCode}/vote`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${authToken}`
			},
			body: JSON.stringify({
				music_id: id,
				voted: is_voted
			})
		});

		if (!response.ok) {
			alert(`Error voting: ${await response.text()}`);
			throw new Error(`Error voting: ${await response.text()}`);
		}
	}
</script>

<div class="grid-cols-1">
	{#if musics}
		<div class="flex flex-wrap justify-center items-center">
			<Search onSubmit={onSearch} />
		</div>
		<Table>
			{#each musics as music (music.id)}
				<MusicTile {...music} {onVote} />
			{/each}
		</Table>
	{:else}
		<Hero>
			<h1 class="text-5xl font-bold">Loading...</h1>
		</Hero>
	{/if}
</div>
