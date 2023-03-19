<script lang="ts">
	type SubmitFunstion = () => void;
	type ButtonType = 'neutral' | 'primary' | 'secondary' | 'accent' | 'ghost' | 'error';
	export let label: string;
	export let onSubmit: SubmitFunstion | undefined = undefined;
	export let type: ButtonType = 'neutral';
	export let disabled = false;
	export let loading = false;
	export let no_marging = false;

	function submit() {
		if (onSubmit) onSubmit();
	}
	// required to have each button style
	// daisyui don't have a way to do this
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const buttonClassStyles = 'btn-neutral btn-primary btn-secondary btn-accent btn-ghost btn-error';

	const buttonClass = `btn btn-${type} ${no_marging ? '' : 'm-2'} text-base shadow-lg`;
</script>

<button type="button" class={buttonClass + (loading ? ' loading' : '')} on:click={submit} {disabled}
	>{label}</button
>
