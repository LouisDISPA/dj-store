<script>
	import Hero from './Hero.svelte';
</script>

<Hero>
	<div class="bg-base-100 rounded-lg shadow-lg p-4 grid grid-cols-1">
		{#if $$slots.title}
			<h2 class="text-2xl text-center font-semibold m-2">
				<slot name="title" />
			</h2>
		{/if}
		{#if $$slots.body}
			<div class="my-3">
				<slot name="body" />
			</div>
		{/if}
		{#if $$slots.actions}
			<div class="flex items-center justify-end">
				<slot name="actions" />
			</div>
		{/if}
	</div>
</Hero>
