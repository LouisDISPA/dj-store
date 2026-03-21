<script lang="ts">
	let {
		value = $bindable(''),
		label = undefined,
		placeholder = '...',
		type = 'text',
		onSubmit = undefined
	}: {
		value?: string;
		label?: string | undefined;
		placeholder?: string;
		type?: string;
		onSubmit?: ((value: string) => void) | undefined;
	} = $props();

	function onChange(event: Event) {
		value = (event.target as HTMLInputElement).value;
	}

	function onKeypress(event: KeyboardEvent) {
		onChange(event);
		if (event.key === 'Enter') onSubmit?.(value);
	}
</script>

<fieldset class="fieldset">
	{#if label}
		<legend class="fieldset-legend">{label}</legend>
	{/if}

	<input {type} {placeholder} class="input" onchange={onChange} onkeypress={onKeypress} />
</fieldset>
