<script lang="ts">
	let { onSubmit }: { onSubmit: (search: string) => Promise<void> } = $props();

	let loading = $state(false);

	async function submit() {
		loading = true;
		await onSubmit(input.value);
		loading = false;
	}

	let input: HTMLInputElement;
</script>

<form
	onsubmit={(e) => {
		e.preventDefault();
		submit();
	}}
>
	<div class="join m-2">
		<input bind:this={input} type="text" placeholder="Search…" class="input join-item pb-0" />
		{#if loading}
			<button class="btn btn-square join-item" aria-label="Searching">
				<span class="loading loading-spinner"></span>
			</button>
		{:else}
			<button class="btn btn-square join-item" aria-label="Search">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/></svg
				>
			</button>
		{/if}
	</div>
</form>
