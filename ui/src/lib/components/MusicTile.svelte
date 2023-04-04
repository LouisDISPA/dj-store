<script lang="ts">
	import AudioPlayPause from './AudioPlayPause.svelte';
	import Stack from './Stack.svelte';
	import type { MusicId } from '$lib/types';

	export let id: MusicId;
	export let title: string;
	export let artist: string;
	export let is_voted = false;
	export let preview_url: string | undefined = undefined;
	export let image_hash: string | undefined = undefined;
	export let votes: number | undefined = undefined;
	export let onVote: OnVote | undefined = undefined;

	type OnVote = (is_voted: boolean, id: MusicId) => void;

	function toogleVote() {
		onVote?.(!is_voted, id);
		is_voted = !is_voted;
		if (votes !== undefined) {
			votes += is_voted ? 1 : -1;
		}
	}

	$: image_url = image_hash?.startsWith('http')
		? image_hash
		: `https://e-cdns-images.dzcdn.net/images/cover/${image_hash}/150x150-000000-80-0-0.jpg`;
</script>

<td>
	<Stack>
		<div class="avatar shadow-md">
			<div class="mask rounded-lg w-16 h-16 md:w-20 md:h-20">
				{#if image_hash}
					<img src={image_url} alt="music poster" />
				{:else}
					<div class="w-16 h-16 md:w-20 md:h-20 bg-white opacity-5" />
				{/if}
			</div>
		</div>
		{#if preview_url}
			<AudioPlayPause url={preview_url} />
		{/if}
	</Stack>
</td>
<td>
	<div class="text-max text-sm opacity-50 truncate">{artist}</div>
	<div class="text-max font-bold truncate">{title}</div>
</td>
<td>
	<div class="rating flex items-center">
		{#if onVote}
			<button class="btn btn-ghost btn-circle p-2 md:p-1 md:mr-4" on:click={toogleVote}>
				<svg
					viewBox="0 0 24 24"
					class="stroke-white stroke-1 fill-transparent"
					class:fill-white={is_voted}
					class:opacity-70={!is_voted}
				>
					<path
						d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z"
					/>
				</svg>
			</button>
		{:else if votes !== undefined}
			<p class="badge text-xl py-5 px-4 mr-4">
				{votes}
			</p>
		{/if}
	</div>
</td>

<style>
	.text-max {
		width: 60vw;
		max-width: 30rem;
	}
</style>
