<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$lib/utils';
</script>

<div class="hero my-10">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1 class="text-5xl font-bold">{$page.status} {$page.error?.message}</h1>
			{#if $page.status === 404}
				<p class="py-6">Sorry the page your looking does not seem to exist.</p>
			{:else}
				<p class="py-6">An unexpected error occured.</p>
			{/if}
			<button class="btn btn-primary btn-outline normal-case" on:click={() => goto('/')}>
				Go home
			</button>
		</div>
	</div>
</div>
