<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		label,
		content
	}: {
		label?: Snippet;
		content?: Snippet;
	} = $props();
</script>

<div class="dropdown dropdown-end dropdown-hover">
	<label tabindex="-1" class="btn btn-ghost">
		{#if label}
			{@render label()}
		{:else}
			Dropdown
		{/if}
	</label>
	<ul tabindex="-1" class="dropdown-content menu p-2 shadow-lg bg-base-300 rounded-box">
		{#if content}
			{@render content()}
		{/if}
	</ul>
</div>
