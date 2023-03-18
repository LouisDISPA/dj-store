<script lang="ts">
	export let onSubmit: (search: string) => Promise<void>;

	let loading = false;

	async function submit() {
		loading = true;
		await onSubmit(input.value);
		loading = false;
	}

	let input: HTMLInputElement;
</script>

<form class="form-control" on:submit|preventDefault={() => submit()}>
	<id class="input-group m-2">
		<input bind:this={input} type="text" placeholder="Search…" class="input input-bordered pb-0" />
		{#if loading}
			<button class="btn btn-square loading" />
		{:else}
			<button class="btn btn-square">
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
	</id>
</form>
