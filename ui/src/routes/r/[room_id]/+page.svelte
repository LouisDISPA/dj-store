<script lang="ts">
	import { page } from '$app/stores';
	import { auth } from '$lib/auth';
	import Button from '$lib/Button.svelte';
	import { getMusics, getSearch, voteForMusic, votes } from '$lib/client';
	import Hero from '$lib/Hero.svelte';
	import MusicTile from '$lib/MusicTile.svelte';
	import Search from '$lib/Search.svelte';
	import Table from '$lib/Table.svelte';
	import type { Music, MusicId } from '$lib/types';
	import { onMount } from 'svelte';

	let musics: Music[] = [];
	let searched = false;
	let error: string | undefined;

	// Since the authentification is done in the layout, we can assume that the user is authenticated
	const auth_token = $auth?.access_token as string;
	const room_id = $page.params.room_id as string;

	onMount(pageLoad);

	async function pageLoad() {
		musics = await getMusics(auth_token, room_id);
		console.log(musics);
		searched = false;
	}

	async function onSearch(search: string) {
		try {
			musics = await getSearch(auth_token, room_id, search);
			console.log(musics);
			searched = true;
		} catch (err) {
			error = 'Search failed (retry later)';
			setTimeout(() => (error = undefined), 3000);
		}
	}

	function onVote(is_voted: boolean, id: MusicId) {
		voteForMusic(auth_token, room_id, is_voted, id);
	}
</script>

<div class="grid-cols-1">
	{#if musics}
		<div class="flex flex-wrap justify-center items-center">
			<Search onSubmit={onSearch} />
			{#if searched}
				<Button label="Back" onSubmit={pageLoad} />
			{/if}
		</div>

		{#if error}
			<div class="flex flex-wrap justify-center items-center">
				<div class="badge badge-error gap-2">
					{error}
				</div>
			</div>
		{/if}
		<Table>
			{#each musics as music (music.id)}
				<MusicTile {...music} {onVote} is_voted={$votes.has(music.id)} />
			{/each}
		</Table>
	{:else}
		<Hero>
			<h1 class="text-5xl font-bold">Loading...</h1>
		</Hero>
	{/if}
</div>