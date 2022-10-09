<script lang="ts">
	type SubmitFunstion = () => void;
	export let label: string;
	export let onSubmit: SubmitFunstion | undefined = undefined;
	export let disabled = false;

	function submit() {
		if (onSubmit) onSubmit();
	}
</script>

<button type="button" on:click={submit} {disabled}>{label}</button>

<style lang="sass">
button
    margin-top: 20px
    padding: 10px 20px
    font-size: 20px
    color: white
    border: 1px solid #ccc
    border-radius: 5px
    background: #555
    cursor: pointer

    &:disabled, &[disabled]
        border: 1px solid #999999
        background-color: #cccccc
        color: #666666
</style>
