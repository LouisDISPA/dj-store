<script lang="ts">
	export let label: string | undefined = undefined;
	export let placeholder: string | undefined = label ? label.toLowerCase() + '...' : undefined;
</script>

<div class="form-control">
	{#if label}
		<div class="label">
			<span class="label-text">{label}</span>
		</div>
	{/if}
	<input type="text" {placeholder} class="input input-bordered" />
</div>
