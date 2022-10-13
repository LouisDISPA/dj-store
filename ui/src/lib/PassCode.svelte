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

	/**
	 * The function to call when the user presses enter.
	 */
	export let onSubmit: (() => void) | undefined = undefined;

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
	// mobile works only with this
	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		const value = target.value;

		if (value.length > 1) {
			target.value = value[1].toUpperCase();
		}

		if (value === '') {
			return;
		}

		updateInput();

		const nextElement = nextInput(target.id);
		if (nextElement) {
			nextElement.focus();
		} else {
			// If there is no next element and we are on a mobile device
			// we can hide the keyboard so the user can see the button
			if (navigator.userAgent.includes('Mobile')) {
				target.blur();
			}
		}
	}

	// Check for backspace, ctrl + backspace, arrow keys and letters
	// And move the cursor accordingly
	function keyDown(event: KeyboardEvent) {
		event.preventDefault();
		const key = event.key;
		const target = event.target as HTMLInputElement;
		const previousElement = previousInput(target.id);
		const nextElement = nextInput(target.id);

		switch (key) {
			case 'ArrowRight':
				nextElement?.focus();
				break;

			case 'ArrowLeft':
				previousElement?.focus();
				break;

			case 'Backspace':
				if (event.ctrlKey) {
					const inputs = getInputElements();
					for (const innerElem of inputs) {
						innerElem.value = '';
					}
					inputs[0].focus();
				} else if (target.value === '' && previousElement) {
					previousElement.value = '';
					previousElement.focus();
				} else {
					target.value = '';
				}
				break;

			case 'Enter':
				onSubmit?.();
				break;

			default:
				// If the key is a letter, put it in the input
				if (key.length === 1 && key.match(/[A-Za-z]/)) {
					target.value = key.toLocaleUpperCase();
					if (nextElement) {
						nextElement.focus();
						// If there is no next element and we are on a mobile device
						// we can hide the keyboard so the user can see the button
					} else if (navigator.userAgent.includes('Mobile')) {
						target.blur();
					}
				}
				break;
		}

		updateInput();
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
					class="input input-bordered w-20 h-20 text-4xl text-center shadow-lg"
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
    margin: 10px
    text-transform: uppercase
</style>
