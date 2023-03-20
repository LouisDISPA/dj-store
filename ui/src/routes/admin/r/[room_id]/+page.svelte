<script lang="ts">
	import Hero from '$lib/components/Hero.svelte';
	import MusicTile from '$lib/components/MusicTile.svelte';
	import Table from '$lib/components/Table.svelte';
	import type { Music } from '$lib/types';
	import { env, goto } from '$lib/utils';
	import { onMount } from 'svelte';
	import { auth } from '$lib/auth';
	import { page } from '$app/stores';
	import { getMusics } from '$lib/client';
	import Spinner from '$lib/components/Spinner.svelte';
	import { flip } from 'svelte/animate';
	import { sineInOut } from 'svelte/easing';

	let musics: Music[] | undefined;

	// Since the authentification is done in the layout, we can assume that the user is authenticated
	const auth_token = $auth?.access_token as string;
	const room_id = $page.params.room_id as string;

	const interval = setInterval(async () => {
		musics = await getMusics(auth_token, room_id);
	}, 60000 * 5);

	onMount(async () => {
		musics = await getMusics(auth_token, room_id);

		if (musics === undefined) {
			alert('You are not allowed to access this room.');
			await goto('/admin');
			return;
		}

		const api_url = env.API_URL.replace('http', 'ws');
		const socket = new WebSocket(`${api_url}/api/room/${room_id}/ws`);

		socket.onmessage = async (event) => {
			const { music_id, like } = JSON.parse(event.data);

			const music = musics?.find((music) => music.id === music_id);
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
				musics?.push(new_music);
			} else {
				music.votes += like ? 1 : -1;
			}

			musics?.sort((a, b) => b.votes - a.votes);
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
			clearInterval(interval);
		};
	});
</script>

<div class="grid-cols-1">
	{#if musics === undefined}
		<Hero>
			<Spinner />
		</Hero>
	{:else if musics.length === 0}
		<Hero>
			<h1 class="text-3xl font-bold">No music voted yet</h1>
		</Hero>
	{:else}
		<Table>
			{#each musics as music (music.id)}
				<tr id={music.id.toString()} animate:flip={{ duration: 400, easing: sineInOut }}>
					<MusicTile {...music} />
				</tr>
			{/each}
		</Table>
	{/if}
</div>
