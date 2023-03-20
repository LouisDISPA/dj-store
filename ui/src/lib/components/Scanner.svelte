<script lang="ts">
	import { onDestroy } from 'svelte';
	import { goto } from '$lib/utils.js';
	import ScannerBorders from './ScannerBorders.svelte';
	import jsQR from 'jsqr';
	import Spinner from './Spinner.svelte';
	import Stack from './Stack.svelte';

	let video: HTMLVideoElement;
	let canvas: HTMLCanvasElement;
	let active = true;
	let stream: MediaStream | null = null;

	onDestroy(stopMediaStream);

	const isMediaStream = (
		candidate: MediaStream | MediaSource | Blob | null
	): candidate is MediaStream => candidate !== null && 'getTracks' in candidate;

	function startMediaStream() {
		navigator.mediaDevices
			.getUserMedia({
				audio: false,
				video: {
					facingMode: 'environment'
				}
			})
			.then((userStream) => {
				stream = userStream;
			})
			.catch((err) => {
				console.log(err);
				alert('Please allow camera access to scan QR code');
				goto('/');
			});
	}

	function stopMediaStream() {
		console.log('stopping media stream');
		if (isMediaStream(stream)) {
			stream.getTracks().forEach((track) => {
				track.stop();
				stream?.removeTrack(track);
			});
		}
		video.srcObject = null;
	}

	const startCapturing = (): void => {
		if (!canvas || !video) return;
		const context = canvas.getContext('2d');
		if (!context) return;
		const { width, height } = canvas;
		context.drawImage(video, 0, 0, width, height);
		const imageData = context.getImageData(0, 0, width, height);
		const qrCode = jsQR(imageData.data, width, height);
		if (qrCode === null) {
			setTimeout(startCapturing, 750);
		} else {
			video.pause();
			const data = qrCode.data;
			console.log(data);

			const currentUrl = window.location.origin;
			if (!data.startsWith(currentUrl)) {
				alert('Invalid QR code: please scan a QR code for this app.');
				video.play();
				setTimeout(startCapturing, 750);
				return;
			} else {
				goto(data);
			}
		}
	};
	const handleCanPlay = (): void => {
		console.log('canplay');
		if (canvas === null || canvas === null || video === null || video === null) {
			return;
		}
		canvas.width = video.videoWidth;
		canvas.height = video.videoHeight;
		startCapturing();
	};
	$: if (video !== null && stream) {
		console.log('Resolve, stream');
		video.srcObject = stream;
		video.play().catch(console.error);
	}
	$: if (active && startMediaStream) {
		startMediaStream();
	}
</script>

<Stack>
	<div class="scanner" class:scanner--hidden={!active}>
		<div class="scanner__aspect-ratio-container">
			<canvas bind:this={canvas} class="scanner__canvas" />
			<video bind:this={video} on:canplay={handleCanPlay} class="scanner__video">
				<track kind="captions" />
			</video>
			<ScannerBorders />
		</div>

		<div class="scanner-tip">
			<div>Scan a QR code with your camera to join a room.</div>
		</div>
	</div>
	{#if !stream}
		<div class="h-32">
			<Spinner />
		</div>
	{/if}
</Stack>

<style>
	.scanner {
		width: 100%;
		max-width: 500px;
	}
	.scanner--hidden {
		display: none;
	}
	.scanner__aspect-ratio-container {
		position: relative;
		overflow: hidden;
		padding-bottom: 100%;
		border-radius: 10%;
		aspect-ratio: 1 / 1;
	}
	.scanner__video {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border-radius: inherit;
		outline: none;
		object-fit: cover;
	}
	.scanner__canvas {
		display: none;
	}
	.scanner-tip {
		display: flex;
		justify-content: center;
		margin-top: 15px;
		font-size: 0.8rem;
	}
</style>
