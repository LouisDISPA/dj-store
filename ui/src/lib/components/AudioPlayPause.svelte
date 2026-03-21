<script lang="ts">
	import { onDestroy } from 'svelte';

	let { url }: { url: string } = $props();

	let audio_element = $state<HTMLAudioElement>();
	let button_element = $state<HTMLButtonElement>();
	let playing = $state(false);

	onDestroy(() => {
		audio_element?.pause();
	});

	function toogleAudio() {
		if (!audio_element) return;
		if (audio_element.paused) {
			const players = document.querySelectorAll('audio');
			for (const player of players) {
				player.pause();
			}
			audio_element.currentTime = 0;
			audio_element.onpause = () => (playing = false);
			audio_element.play();
			playing = true;
		} else {
			audio_element.pause();
			playing = false;
		}
	}
</script>

<button
	bind:this={button_element}
	class="btn btn-ghost swap swap-rotate w-16 h-16 md:w-20 md:h-20"
	class:swap-active={playing}
	onclick={toogleAudio}
>
	{#if playing}
		<div class="radial-progress text-white progress-animation"></div>
	{/if}
	<audio bind:this={audio_element} src={url}></audio>
	<svg class="swap-off w-12 h-12 stroke-white fill-none" viewBox="0 0 24 24">
		<circle class="fill-black stroke-transparent opacity-30" cx="12" cy="12" r="11" />
		<path stroke-linecap="round" stroke-linejoin="round" d="M9 19l7-7-7-7" />
	</svg>
	<svg class="swap-on w-12 h-12 stroke-white fill-none" viewBox="0 0 24 24">
		<circle class="fill-black stroke-transparent opacity-60" cx="12" cy="12" r="11" />
		<path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25v13.5m-7.5-13.5v13.5" />
	</svg>
</button>

<style>
	@property --temp {
		syntax: '<number>';
		initial-value: 0;
		inherits: false;
	}

	@keyframes circle {
		100% {
			--temp: 100;
		}
	}

	.progress-animation {
		z-index: 4;
		animation: circle 30.3s infinite linear;
		--value: var(--temp);
		--thickness: 3px;
		--size: 3rem;
	}
</style>
