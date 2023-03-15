<script lang="ts">
	import { page } from '$app/stores';
	import { auth } from '$lib/auth';
	import { getMusics } from '$lib/client';
	import Hero from '$lib/Hero.svelte';
	import MusicTile from '$lib/MusicTile.svelte';
	import Search from '$lib/Search.svelte';
	import Table from '$lib/Table.svelte';
	import type { Music } from '$lib/types';
	import { env, goto } from '$lib/utils';
	import { onMount } from 'svelte';

	let musics: Music[] = [];

	// Since the authentification is done in the layout, we can assume that the user is authenticated
	const auth_token = $auth?.access_token as string;
	const room_id = $page.params.room_id as string;

	onMount(async () => {
		musics = await getMusics(room_id, auth_token);
	});

	async function onSearch(search: string) {
		await goto(`/r/${room_id}/search?query=${search}`);
	}

	async function onVote(like: boolean, id: string) {
		const response = await fetch(`${env.API_URL}/api/room/${room_id}/vote`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${auth_token}`
			},
			body: JSON.stringify({
				music_id: id,
				like
			})
		});

		if (!response.ok) {
			throw new Error(`Error voting: ${await response.text()}`);
		}
		auth.update((auth) => {
			if (like && auth && !auth.votes.includes(id)) {
				auth.votes.push(id);
			}
			if (!like && auth) {
				auth.votes = auth.votes.filter((vote) => vote !== id);
			}
			return auth;
		});
	}
</script>

<div class="grid-cols-1">
	{#if musics}
		<div class="flex flex-wrap justify-center items-center">
			<Search onSubmit={onSearch} />
		</div>
		<Table>
			{#each musics as music (music.id)}
				<MusicTile {...music} {onVote} is_voted={$auth?.votes.includes(music.id)} />
			{/each}
		</Table>
	{:else}
		<Hero>
			<h1 class="text-5xl font-bold">Loading...</h1>
		</Hero>
	{/if}
</div>
