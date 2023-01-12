<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Button.svelte';
	import jsQR from 'jsqr';
	import { onDestroy, onMount } from 'svelte';

	let stream_outer: undefined | MediaStream;
	let loading = false;
	let stopped = false;

	const stopStream = () => {
		stopped = true;
		if (stream_outer) {
			stream_outer.getTracks().forEach((track) => track.stop());
		}
	};

	onDestroy(stopStream);

	onMount(function () {
		const video = document.createElement('video');
		const canvasElement = document.getElementById('canvas') as HTMLCanvasElement;
		const canvas = canvasElement?.getContext('2d') as CanvasRenderingContext2D;

		if (canvas == null) {
			alert('Error loading qrcode scanner.');
			goto('/');
		}

		// Use facingMode: environment to attemt to get the front camera on phones
		navigator.mediaDevices
			.getUserMedia({ video: { facingMode: 'environment' } })
			.then(function (stream) {
				video.srcObject = stream;
				video.setAttribute('playsinline', 'true'); // required to tell iOS safari we don't want fullscreen
				video.play();
				stream_outer = stream;
				requestAnimationFrame(tick);
			});

		function tick() {
			if (video.readyState === video.HAVE_ENOUGH_DATA) {
				loading = false;
				canvasElement.hidden = false;
				canvasElement.height = video.videoHeight;
				canvasElement.width = video.videoWidth;
				canvas.drawImage(video, 0, 0, canvasElement.width, canvasElement.height);
				var imageData = canvas.getImageData(0, 0, canvasElement.width, canvasElement.height);
				var code = jsQR(imageData.data, imageData.width, imageData.height, {
					inversionAttempts: 'dontInvert'
				});
				if (code) {
					if (code.data.startsWith('http')) {
						if (code.data.startsWith(document.location.origin)) {
							stopStream();
							canvasElement.hidden = true;
							goto(code.data.substring(document.location.origin.length));
						} else {
							alert('Invalid QR Code');
						}
					}
				}
			}
			if (!stopped) {
				requestAnimationFrame(tick);
			}
		}
	});
</script>

<div
	class="absolute inset-0 h-screen p-4 overflow-x-hidden overflow-y-auto md:inset-0 h-modal md:h-full"
>
	<div class="relative w-full h-full max-w-2xl md:h-auto">
		<!-- Modal content -->
		<div class="relative bg-base-100 rounded-lg shadow-lg">
			<!-- Modal header -->
			<h3 class="text-xl font-semibold text-center">Scan QR Code</h3>
			<!-- Modal body -->
			<div class="p-6 space-y-6">
				<canvas id="canvas" hidden />

				{#if loading || stopped}
					<div class="flex items-center justify-center">loading...</div>
				{/if}
			</div>
			<!-- Modal footer -->
			<div class="flex items-center rounded p-3">
				<Button label="Cancel" type="primary" onSubmit={() => goto('/')} />
			</div>
		</div>
	</div>
</div>
