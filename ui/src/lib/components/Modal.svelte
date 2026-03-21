<script lang="ts">
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';

	let {
		title,
		body,
		actions
	}: {
		title?: Snippet;
		body?: Snippet;
		actions?: Snippet;
	} = $props();

	let dialog: HTMLDialogElement;

	onMount(() => {
		dialog.showModal();
	});
</script>

<dialog bind:this={dialog} class="modal modal-bottom sm:modal-middle">
	<div class="modal-box">
		{#if title}
			<h3 class="text-lg font-bold">
				{@render title()}
			</h3>
		{/if}
		{#if body}
			<div class="py-4">
				{@render body()}
			</div>
		{/if}
		{#if actions}
			<div class="modal-action">
				{@render actions()}
			</div>
		{/if}
	</div>
</dialog>
