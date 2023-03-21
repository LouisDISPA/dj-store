<script lang="ts">
	type SubmitFunstion = () => void;
	type ButtonType = 'neutral' | 'primary' | 'secondary' | 'accent' | 'ghost' | 'error';
	export let label: string;
	export let onSubmit: SubmitFunstion | undefined = undefined;
	export let type: ButtonType = 'neutral';
	export let disabled = false;
	export let loading = false;
	export let no_marging = false;
	export let outlined = false;
	export let normal_case = false;

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
	class:loading
	on:click={submit}
	{disabled}
>
	{label}
</button>
