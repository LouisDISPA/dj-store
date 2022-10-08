<script lang="ts">
	export let input: string;

	function handleInput(event: Event) {
		input = (event.target as HTMLInputElement).value;
		input = input.toUpperCase().substring(0, 6);
		if (input.length === 6) {
			(event.target as HTMLInputElement).blur();
		}
	}

	function clear() {
		input = '';
	}
</script>

<input type="text" spellcheck="false" value={input} on:input={handleInput} on:focus={clear} />

<style lang="sass">
input
    font-family: monospace
    width: 360px
    font-size: 40px
    text-align: left
    padding-left: 20px
    padding-bottom: 5px
    padding-top: 8px
    letter-spacing: 40px
    border: 1px solid #ccc
    background: linear-gradient(to left, #ccc 2px, transparent 0)
    background-size: 64px
    overflow: visible

    &:focus
        background: linear-gradient(to left, #555 2px, transparent 0)
        background-size: 64px
        background-position: 0 100%
        background-repeat: repeat-x
</style>
