<script lang="ts">
	import { page } from '$app/stores';
	import Button from '$lib/Button.svelte';
	import Hero from '$lib/Hero.svelte';
	import { goto } from '$lib/utils';
</script>

<Hero>
	<div class="grid grid-cols-1 justify-items-center gap-3">
		<h1 class="text-5xl font-bold p-3">{$page.error?.message}</h1>
		{#if $page.error?.detail}
			<p class="m-3">{$page.error?.detail}</p>
		{/if}
		<Button label="Go home" type="primary" outlined normal_case onSubmit={() => goto('/')} />
	</div>
</Hero>
