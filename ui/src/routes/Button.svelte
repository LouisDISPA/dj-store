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
    margin: 15px
    padding: 15px 25px
    font-size: 25px
    font-weight: bold
    color: white
    border: 1px solid #ccc
    border-radius: 20px
    background: #555
    cursor: pointer

    &:disabled, &[disabled]
        border: 1px solid #999999
        background-color: #cccccc
        color: #666666
</style>
