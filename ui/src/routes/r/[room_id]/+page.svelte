<script lang="ts">
	import { page } from '$app/stores';
	import { auth } from '$lib/auth';
	import Button from '$lib/components/Button.svelte';
	import { getMusics, getSearch, voteForMusic, voted_for } from '$lib/client';
	import Hero from '$lib/components/Hero.svelte';
	import MusicTile from '$lib/components/MusicTile.svelte';
	import Search from '$lib/components/Search.svelte';
	import Spinner from '$lib/components/Spinner.svelte';
	import Table from '$lib/components/Table.svelte';
	import type { Music, MusicId } from '$lib/types';
	import { onMount } from 'svelte';

	let musics: Music[] | undefined;
	let searched: string | undefined;
	let error: string | undefined;

	// Since the authentification is done in the layout, we can assume that the user is authenticated
	// TODO: use context instead of stores
	const auth_token = $auth?.access_token as string;
	const room_id = $page.params.room_id as string;

	onMount(loadMusic);

	async function loadMusic() {
		musics = await getMusics(auth_token, room_id);
		searched = undefined;
	}

	async function onSearch(search: string) {
		try {
			musics = await getSearch(auth_token, room_id, search);
			searched = search;
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
		<div class="flex flex-wrap justify-center w-full items-center">
			<Search onSubmit={onSearch} />
			{#if searched}
				<Button label="Back" onSubmit={loadMusic} no_marging />
			{/if}
		</div>

		{#if error}
			<div class="flex flex-wrap justify-center items-center">
				<div class="badge badge-error">
					{error}
				</div>
			</div>
		{/if}

		<div class="flex flex-wrap justify-center items-center">
			<div class="badge badge-lg">
				{#if searched}
					Search results for '{searched}'
				{:else}
					Most voted
				{/if}
			</div>
		</div>

		{#if musics.length === 0}
			<Hero>
				<h1 class="text-2xl font-bold">No musics</h1>
			</Hero>
		{/if}

		<Table>
			{#each musics as music (music.id)}
				<tr id={music.id.toString()}>
					<MusicTile {...music} {onVote} is_voted={$voted_for.has(music.id)} />
				</tr>
			{/each}
		</Table>
	{:else}
		<Hero>
			<Spinner />
		</Hero>
	{/if}
</div>
