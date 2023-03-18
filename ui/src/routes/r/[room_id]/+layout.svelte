<script lang="ts">
	import { auth } from '$lib/auth';
	import Hero from '$lib/Hero.svelte';
	import { page } from '$app/stores';
	import Spinner from '$lib/Spinner.svelte';
</script>

{#if !$auth && $page.params.room_id}
	<Hero>
		<Spinner />
	</Hero>
{:else}
	<slot />
{/if}
