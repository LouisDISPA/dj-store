<script lang="ts">
	import { onMount } from 'svelte';
	import Button from './Button.svelte';
	import Modal from './Modal.svelte';

	export let url: string;
	export let onClose: (() => void) | undefined = undefined;

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

<div class="fixed top-0 left-0 w-full h-full bg-black bg-opacity-50">
	<Modal>
		<span slot="body">
			<div class="p-6 mx-3 bg-white rounded-xl">
				<canvas bind:this={canvas} />
			</div>
			<h3 class="text-sm text-center pt-4 pb-3">{url}</h3>
		</span>
		<span slot="actions">
			<Button label="Close" type="primary" onSubmit={onClose} />
		</span>
	</Modal>
</div>
