<script lang="ts" context="module">
	import { fade } from 'svelte/transition';
	import { writable } from 'svelte/store';

	export const loading = writable(false);
</script>

{#if $loading}
	<div
		id="overlay"
		class="overlay"
		in:fade={{ duration: 400, delay: 100 }}
		out:fade={{ duration: 200 }}
	>
		<div class="overlay__inner">
			<div class="overlay__content"><span class="spinner" /></div>
		</div>
	</div>
{/if}

<style lang="sass">
.overlay
    opacity: 1
    left: 0
    top: 0
    width: 100%
    height: 100%
    position: fixed
    background: rgba(44, 44, 44, 0.3)
    backdrop-filter: blur(10px)

.overlay__inner
    left: 0
    top: 0
    width: 100%
    height: 100%
    position: absolute

.overlay__content
    left: 50%
    position: absolute
    top: 50%
    transform: translate(-50%, -50%)

.spinner
    width: 75px
    height: 75px
    display: inline-block
    border-width: 4px
    border-color: rgba(0, 0, 0, 0.20)
    border-top-color: #fff
    animation: spin 1s infinite linear
    border-radius: 100%
    border-style: solid

@keyframes spin
    100%
        transform: rotate(360deg)

</style>
