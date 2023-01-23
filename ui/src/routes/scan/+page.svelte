<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Button.svelte';
	import Hero from '$lib/Hero.svelte';
	import Spinner from '$lib/Spinner.svelte';
	import { onDestroy, onMount } from 'svelte';

	let stream: undefined | MediaStream;
	let loading = true;
	let stopped = false;
	let canvasElement: HTMLCanvasElement;

	const stopStream = () => {
		stopped = true;
		if (stream) {
			stream.getTracks().forEach((track) => track.stop());
		}
	};

	onDestroy(stopStream);

	onMount(async function () {
		const jsQR = (await import('jsqr')).default;
		const video = document.createElement('video');
		const canvas = canvasElement?.getContext('2d') as CanvasRenderingContext2D;

		if (canvas == null) {
			alert('Error loading qrcode scanner.');
			goto('/');
		}

		// Use facingMode: environment to attemt to get the front camera on phones
		navigator.mediaDevices
			.getUserMedia({ video: { facingMode: 'environment' } })
			.then(function (mediaStream) {
				video.srcObject = mediaStream;
				video.setAttribute('playsinline', 'true'); // required to tell iOS safari we don't want fullscreen
				video.play();
				stream = mediaStream;
				requestAnimationFrame(tick);
			})
			.catch(function () {
				loading = false;
				alert('Failed to get camera access.');
				goto('/');
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
				if (code) checkQrCode(code);
			}
			if (!stopped) {
				requestAnimationFrame(tick);
			}
		}
	});

	function checkQrCode(code: { data: string }) {
		if (!code.data.startsWith('http')) {
			return;
		}
		if (!code.data.startsWith(document.location.origin)) {
			alert('Invalid QR Code');
		}
		stopStream();
		goto(code.data.substring(document.location.origin.length));
	}
</script>

<Hero>
	<!-- Modal content -->
	<div class="bg-base-100 rounded-lg shadow-lg">
		<!-- Modal header -->
		<h3 class="text-xl font-semibold text-center pt-4 pb-3">Scan QR Code</h3>
		<!-- Modal body -->
		<div class="p-6 space-y-6 stack">
			<canvas bind:this={canvasElement} hidden />

			{#if loading && !stopped}
				<Spinner />
			{/if}
		</div>
		<!-- Modal footer -->
		<div class="flex items-center rounded p-3">
			<Button label="Cancel" type="primary" onSubmit={() => goto('/')} />
		</div>
	</div>
</Hero>
