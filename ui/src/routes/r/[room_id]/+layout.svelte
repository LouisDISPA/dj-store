<script lang="ts">
	import { auth } from '$lib/auth';
	import Hero from '$lib/components/Hero.svelte';
	import Spinner from '$lib/components/Spinner.svelte';
</script>

{#if !$auth}
	<Hero>
		<Spinner />
	</Hero>
{:else}
	<slot />
{/if}
