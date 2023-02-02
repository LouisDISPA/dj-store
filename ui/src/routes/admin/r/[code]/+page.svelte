<script lang="ts">
	import Hero from '$lib/Hero.svelte';
	import MusicTile from '$lib/MusicTile.svelte';
	import Table from '$lib/Table.svelte';
	import { env, goto } from '$lib/utils';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	let musics = data.musics;
	const { authToken, roomCode } = data;

	onMount(async () => {
		const api_url = env.API_URL.replace('http', 'ws').replace('://', `://Bearer ${authToken}@`);
		const socket = new WebSocket(`${api_url}/api/room/${roomCode}/ws`);

		socket.onmessage = async (event) => {
			const array = new Uint32Array(await event.data.arrayBuffer());
			if (array.length != 2) return;

			const [id, votes] = array;

			const music = musics.find((music) => music.id === id);
			if (!music) {
				fetch(`${env.API_URL}/api/room/${roomCode}/music/${id}`, {
					method: 'GET',
					headers: {
						Authorization: `Bearer ${authToken}`
					}
				})
					.then((res) => res.json())
					.then((music) => {
						musics.push(music);
						musics.sort((a, b) => b.votes - a.votes);
						musics = musics;
					});
				return;
			}

			music.votes = votes;
			musics.sort((a, b) => b.votes - a.votes);
			musics = musics;
		};

		socket.onopen = () => {
			socket.send(authToken);
		};

		socket.onclose = () => {
			console.log('Socket closed');
			goto('/login');
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
