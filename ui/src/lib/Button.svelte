<script lang="ts">
	type SubmitFunstion = () => void;
	type ButtonType =
		| 'neutral'
		| 'primary'
		| 'secondary'
		| 'accent'
		| 'ghost'
		| 'link'
		| 'info'
		| 'success'
		| 'warning'
		| 'error';
	export let label: string;
	export let onSubmit: SubmitFunstion | undefined = undefined;
	export let type: ButtonType = 'neutral';
	export let disabled = false;
	export let loading = false;

	function submit() {
		if (onSubmit) onSubmit();
	}

	const buttonClass = `btn btn-${type} m-2 text-base shadow-lg`;
</script>

<button type="button" class={buttonClass + (loading ? ' loading' : '')} on:click={submit} {disabled}
	>{label}</button
>
