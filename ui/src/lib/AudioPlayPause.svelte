<script lang="ts">
	import { onDestroy } from 'svelte';

	export let url: string;

	let audio_element: HTMLAudioElement;

	onDestroy(() => {
		audio_element?.pause();
	});

	function toogleAudio() {
		if (audio_element?.paused) {
			const players = document.querySelectorAll('audio');
			for (const player of players) {
				player.pause();
			}
			audio_element.currentTime = 0;
			audio_element.onpause = () => {
				audio_element = audio_element;
			};
			audio_element.play();
		} else {
			audio_element?.pause();
		}
	}
</script>

<button class="btn btn-square btn-ghost w-20 h-20" on:click={toogleAudio}>
	<audio bind:this={audio_element} src={url} />
	<svg
		xmlns="http://www.w3.org/2000/svg"
		fill="none"
		viewBox="0 0 24 24"
		stroke-width="1"
		class="w-16 h-16 stroke-white"
	>
		<circle class="fill-black stroke-transparent opacity-50" cx="12" cy="12" r="10" />
		{#if audio_element?.paused}
			<path stroke-linecap="round" stroke-linejoin="round" d="M9 19l7-7-7-7" />
		{:else}
			<path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25v13.5m-7.5-13.5v13.5" />
		{/if}
	</svg>
</button>
