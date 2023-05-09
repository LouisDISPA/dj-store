<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$lib/utils';
	import { auth, joinRoom, tryRecallUser } from '$lib/auth';
	import Hero from '$lib/components/Hero.svelte';
	import Spinner from '$lib/components/Spinner.svelte';

	onMount(() => {
		tryRecallUser();
		joinRoom($page.params.room_id).catch(() => goto('/'));
	});
</script>

{#if !$auth}
	<Hero>
		<Spinner />
	</Hero>
{:else}
	<slot />
{/if}
