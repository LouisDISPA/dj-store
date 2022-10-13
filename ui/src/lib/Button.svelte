<script lang="ts">
	type SubmitFunstion = () => void;
	export let label: string;
	export let onSubmit: SubmitFunstion | undefined = undefined;
	export let disabled = false;
	export let loading = false;

	function submit() {
		if (onSubmit) onSubmit();
	}

	const buttonClass = 'btn btn-primary m-2 text-base shadow-lg';
</script>

<button type="button" class={buttonClass + (loading ? ' loading' : '')} on:click={submit} {disabled}
	>{label}</button
>
