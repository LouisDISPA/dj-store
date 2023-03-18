<script lang="ts">
	import Hero from '$lib/Hero.svelte';
	import MusicTile from '$lib/MusicTile.svelte';
	import Table from '$lib/Table.svelte';
	import type { Music } from '$lib/types';
	import { env, goto } from '$lib/utils';
	import { onMount } from 'svelte';
	import { auth } from '$lib/auth';
	import { page } from '$app/stores';
	import { getMusics } from '$lib/client';

	let musics: Music[] = [];

	// Since the authentification is done in the layout, we can assume that the user is authenticated
	const auth_token = $auth?.access_token as string;
	const room_id = $page.params.room_id as string;

	onMount(async () => {
		musics = await getMusics(auth_token, room_id);

		const api_url = env.API_URL.replace('http', 'ws');
		const socket = new WebSocket(`${api_url}/api/room/${room_id}/ws`);

		socket.onmessage = async (event) => {
			const { music_id, like } = JSON.parse(event.data);

			const music = musics.find((music) => music.id === music_id);
			if (!music) {
				const new_music: Music = await fetch(
					`${env.API_URL}/api/room/${room_id}/music/${music_id}`,
					{
						method: 'GET',
						headers: {
							Authorization: `Bearer ${auth_token}`
						}
					}
				).then((res) => res.json());
				musics.push(new_music);
			} else {
				music.votes += like ? 1 : -1;
			}

			musics.sort((a, b) => b.votes - a.votes);
			musics = musics;
		};

		socket.onopen = async () => {
			socket.send(auth_token);
		};

		socket.onclose = (frame) => {
			console.log(`WebSocket closed: ${frame.code} ${frame.reason}`);
			alert('You have been disconnected from the room.');
			goto('/admin');
		};

		socket.onerror = (error) => {
			console.log(error);
		};

		return () => {
			socket.close();
		};
	});
</script>

<div class="grid-cols-1">
	{#if musics}
		<Table>
			{#each musics as music (music.id)}
				<MusicTile {...music} />
			{/each}
		</Table>
	{:else}
		<Hero>
			<h1 class="text-5xl font-bold">Loading...</h1>
		</Hero>
	{/if}
</div>
