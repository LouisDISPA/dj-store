<script lang="ts">
	import { page } from '$app/stores';
	import Hero from '$lib/Hero.svelte';
	import { goto } from '$lib/utils';
</script>

<Hero>
	<h1 class="text-5xl font-bold p-3">{$page.error?.message}</h1>
	{#if $page.error?.detail}
		<p class="m-3">{$page.error?.detail}</p>
	{/if}
	<button class="btn btn-primary btn-outline normal-case m-3" on:click={() => goto('/')}>
		Go home
	</button>
</Hero>
