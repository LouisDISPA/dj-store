<script lang="ts">
	export let value = '';
	export let label: string | undefined = undefined;
	export let placeholder = '...';
	export let type = 'text';
	export let onSubmit: ((value: string) => void) | undefined = undefined;

	function onChange(event: Event) {
		value = (event.target as HTMLInputElement).value;
	}

	function onKeypress(event: KeyboardEvent) {
		if (event.key === 'Enter') onSubmit?.(value);
	}
</script>

<div class="form-control">
	{#if label}
		<div class="label">
			<span class="label-text">{label}</span>
		</div>
	{/if}

	<input
		{type}
		{placeholder}
		class="input input-bordered"
		on:change={onChange}
		on:keypress={onKeypress}
	/>
</div>
