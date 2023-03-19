<script lang="ts">
	import { onMount } from 'svelte';
	import Button from './Button.svelte';

	export let url: string;
	export let onClose: (() => void) | undefined = undefined;

	let canvas: HTMLCanvasElement;
	onMount(async () => {
		const QRious = (await import('qrious')).default;
		new QRious({
			element: canvas,
			value: url,
			size: 500
		});
	});
</script>

<!-- absolute popup with darker background and if you click outside it closes the popup -->

<div
	class="fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 z-50 flex items-center justify-center"
>
	<!-- Modal content -->
	<div class="bg-base-100 rounded-lg shadow-lg">
		<!-- Modal body -->
		<div class="mx-5 mt-6 p-6  bg-white rounded-xl">
			<canvas bind:this={canvas} />
		</div>
		<h3 class="text-sm text-center pt-4 pb-3">{url}</h3>
		<!-- Modal footer -->
		<div class="flex items-center justify-end rounded p-3">
			<Button label="Close" type="primary" onSubmit={onClose} />
		</div>
	</div>
</div>
