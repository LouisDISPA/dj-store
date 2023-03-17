<script lang="ts">
	import { auth } from '$lib/auth';
	import Hero from '$lib/Hero.svelte';
	import { goto } from '$lib/utils';
	import Button from '$lib/Button.svelte';
</script>

{#if $auth?.role !== 'Admin'}
	<Hero>
		<!-- Modal content -->
		<div class="bg-base-100 rounded-lg shadow-lg">
			<!-- Modal header -->
			<h3 class="text-xl font-semibold text-center pt-4 pb-3">You are not connect</h3>
			<!-- Modal body -->
			<div class="px-5 stack">
				<p class="text-center">You can't access this page. Please connect as an admin.</p>
			</div>
			<!-- Modal footer -->
			<div class="flex justify-between rounded p-3">
				<Button label="Goto Login" type="primary" onSubmit={() => goto('/login')} />
			</div>
		</div>
	</Hero>
{:else}
	<slot />
{/if}
