<script lang="ts">
	import { auth } from '$lib/auth';
	import Hero from '$lib/components/Hero.svelte';
	import { page } from '$app/stores';
	import Spinner from '$lib/components/Spinner.svelte';
</script>

{#if !$auth && $page.params.room_id && !$page.error}
	<Hero>
		<Spinner />
	</Hero>
{:else}
	<slot />
{/if}
