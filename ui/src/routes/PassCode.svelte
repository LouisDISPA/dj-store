<script lang="ts">
	import { onMount } from 'svelte';

	/**
	 * The pass code. This is a string of 6 capital letters.
	 * You need to bind input to get the value.
	 * ```svelte
	 * <PassCode bind:input />
	 * ```
	 */
	export let input: string;

	const codeSize = 6;
	const inputIds = [...Array(codeSize).keys()].map((i) => `input-${i}`);

	/**
	 * Change the input with every value in each of the inputs
	 */
	function updateInput() {
		const inputElements = getInputElements();
		const tempInput = Array.from(inputElements)
			.map((input) => input.value)
			.join('');
		input = tempInput;
	}

	/**
	 * @returns The list of Input elements
	 */
	function getInputElements(): HTMLInputElement[] {
		return inputIds.map((id) => document.getElementById(id) as HTMLInputElement);
	}
	/**
	 * @param inputId - id of the current input element
	 *
	 * @returns the next Input element or null id there is none
	 */
	function nextInput(inputId: string): HTMLInputElement | null {
		const index = inputIds.indexOf(inputId);
		if (index >= inputIds.length || index === -1) return null;
		return document.getElementById(inputIds[index + 1]) as HTMLInputElement;
	}

	/**
	 * @param inputId - id of the current input element
	 *
	 * @returns the previous Input element or null if there is none
	 */
	function previousInput(inputId: string): HTMLInputElement | null {
		const index = inputIds.indexOf(inputId);
		if (index <= 0) return null;
		return document.getElementById(inputIds[index - 1]) as HTMLInputElement;
	}

	// Go to the next input when the current input is full
	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		const value = target.value;
		if (value.length > 1) {
			target.value = value[1];
		}

		updateInput();

		if (value === '') {
			return;
		}
		const nextElement = nextInput(target.id);
		if (nextElement) {
			nextElement.focus();
		} else {
			target.blur();
		}
	}

	// Check for backspace, ctrl + backspace, and arrow keys
	function keyDown(event: KeyboardEvent) {
		const target = event.target as HTMLInputElement;
		const previousElement = previousInput(target.id);
		const nextElement = nextInput(target.id);

		//Right Arrow Key
		if (event.keyCode === 39) {
			nextElement?.focus();
		}

		//Left Arrow Key
		//Add Highlight
		if (event.keyCode === 37) {
			previousElement?.focus();
		}

		// ctrl + Backspace Key
		if (event.keyCode === 8 && event.ctrlKey) {
			const inputs = getInputElements();
			for (const innerElem of inputs) {
				innerElem.value = '';
			}
			inputs[0].focus();
		}

		// Backspace Key
		if (event.keyCode === 8 && !event.ctrlKey) {
			if (target.value === '') {
				previousElement?.focus();
				return;
			}
			target.value = '';
		}
	}

	// Focus the first input on mount
	onMount(() => {
		const input = document.getElementById(inputIds[0]) as HTMLInputElement;
		input.focus();
	});
</script>

<div class="passcode-area">
	{#each [0, Math.ceil(codeSize / 2)] as block_number}
		<div class="passcode-block">
			{#each inputIds.slice(block_number, block_number + Math.ceil(codeSize / 2)) as id}
				<input
					{id}
					type="text"
					autocomplete="off"
					spellcheck="false"
					maxlength="1"
					on:input={handleInput}
					on:keydown={keyDown}
				/>
			{/each}
		</div>
	{/each}
</div>

<style lang="sass">
.passcode-area
    display: flex
    align-items: center
    justify-content: center
    flex-direction: row
    flex-wrap: wrap

.passcode-block 
    padding: 10px 20px

.passcode-block > input
    border: 2px solid #d6d6d6
    border-radius: 4px
    padding: 0
    margin: 10px
    width: 65px
    height: 65px
    text-align: center
    font-size: 32px
    line-height: 1.29
    text-transform: uppercase
    background-clip: padding-box

    &:focus
        -webkit-appearance: none
        border: 2px solid skyblue
        outline: 0
        box-shadow: 0px 0px 3px rgba(131, 192, 253, 0.5)
</style>
