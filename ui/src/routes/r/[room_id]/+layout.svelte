<script lang="ts">
	import { auth } from '$lib/auth';
	import Hero from '$lib/Hero.svelte';
	import { page } from '$app/stores';
	import Spinner from '$lib/Spinner.svelte';
</script>

{#if !$auth && $page.params.room_id}
	<Hero>
		<div class="bg-base-100 rounded-lg shadow-lg px-6">
			<h3 class="text-xl font-semibold text-center pt-4 pb-3">Connecting to room</h3>
			<div class="flex items-center">
				<Spinner />
			</div>
		</div>
	</Hero>
{:else}
	<slot />
{/if}
