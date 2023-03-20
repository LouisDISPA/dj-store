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
</script>

<td>
	<Stack>
		<div class="avatar shadow-md">
			<div class="mask rounded-lg w-16 h-16 md:w-20 md:h-20">
				{#if image_hash}
					<img src={image_hash} alt="music poster" />
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
	<div class="text-max  font-bold truncate">{title}</div>
</td>
<td>
	<div class="rating flex items-center">
		{#if onVote}
			<input
				type="checkbox"
				class="mask mask-heart bg-red-500 opacity-30 checked:opacity-80"
				on:change={toogleVote}
				checked={is_voted}
			/>
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
