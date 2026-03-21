<script lang="ts">
	import { onMount } from 'svelte';
	import Button from './Button.svelte';
	import Modal from './Modal.svelte';

	let {
		url,
		onClose = undefined
	}: {
		url: string;
		onClose?: (() => void) | undefined;
	} = $props();

	let canvas: HTMLCanvasElement;
	onMount(async () => {
		const QRious = (await import('qrious')).default;
		new QRious({
			element: canvas,
			value: url,
			size: 400
		});
	});
</script>

<Modal>
	{#snippet body()}
		<div class="p-6 mx-3 bg-white rounded-xl">
			<canvas class="w-full" bind:this={canvas}></canvas>
		</div>
		<h3 class="text-sm text-center pt-4 pb-3">{url}</h3>
	{/snippet}
	{#snippet actions()}
		<Button label="Close" type="primary" onSubmit={onClose} />
	{/snippet}
</Modal>
