<script lang="ts">
	type SubmitFunstion = () => void;
	type ButtonType = 'neutral' | 'primary' | 'secondary' | 'accent' | 'ghost' | 'error';

	let {
		label,
		onSubmit = undefined,
		type = 'neutral',
		disabled = false,
		loading = false,
		no_marging = false,
		outlined = false,
		normal_case = false
	}: {
		label: string;
		onSubmit?: SubmitFunstion | undefined;
		type?: ButtonType;
		disabled?: boolean;
		loading?: boolean;
		no_marging?: boolean;
		outlined?: boolean;
		normal_case?: boolean;
	} = $props();

	function submit() {
		if (onSubmit) onSubmit();
	}
	// because of the way daisyui works, we need to add the btn- prefix to the file
	// btn-neutral btn-primary btn-secondary btn-accent btn-ghost btn-error
</script>

<button
	type="button"
	class={`btn btn-${type}`}
	class:btn-outline={outlined}
	class:normal-case={normal_case}
	class:m-2={!no_marging}
	onclick={submit}
	{disabled}
>
	{#if loading}
		<span class="loading loading-spinner"></span>
	{/if}
	{label}
</button>
