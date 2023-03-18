<script lang="ts">
	import { onDestroy } from 'svelte';

	export let url: string;

	let audio_element: HTMLAudioElement;
	let button_element: HTMLButtonElement;

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
				button_element.classList.remove('swap-active');
			};
			audio_element.play();
			button_element.classList.add('swap-active');
		} else {
			audio_element.pause();
			button_element.classList.remove('swap-active');
		}
	}
</script>

<button
	bind:this={button_element}
	class="btn btn-square btn-ghost swap swap-rotate w-16 h-16 md:w-20 md:h-20"
	on:click={toogleAudio}
>
	<audio bind:this={audio_element} src={url} />
	<svg class="swap-off w-12 h-12 stroke-white" viewBox="0 0 24 24">
		<circle class="fill-black stroke-transparent opacity-50" cx="12" cy="12" r="10" />
		<path stroke-linecap="round" stroke-linejoin="round" d="M9 19l7-7-7-7" />
	</svg>
	<svg class="swap-on w-12 h-12 stroke-white" viewBox="0 0 24 24">
		<circle class="fill-black stroke-transparent opacity-50" cx="12" cy="12" r="10" />
		<path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25v13.5m-7.5-13.5v13.5" />
	</svg>
</button>
