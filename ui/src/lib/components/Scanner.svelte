<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$lib/utils';
	import ScannerBorders from './ScannerBorders.svelte';
	import jsQR from 'jsqr';
	import Spinner from './Spinner.svelte';
	import Stack from './Stack.svelte';
	import { joinRoom } from '$lib/auth';

	const millisecondsBetweenScans = 500;

	let video: HTMLVideoElement | null;
	let canvas: HTMLCanvasElement | null;
	$: context = canvas?.getContext('2d', { willReadFrequently: true });

	onMount(startMediaStream);

	function startMediaStream() {
		const stream = navigator.mediaDevices
			.getUserMedia({
				video: {
					facingMode: 'environment'
				}
			})
			.then((userStream) => {
				if (video) {
					video.srcObject = userStream;
				}
				return userStream;
			})
			.catch((err) => {
				console.error(err);
				alert('Please allow camera access to scan QR code');
				return goto('/');
			});

		return () => stream.then(stopMediaStream);
	}

	function stopMediaStream(stream: MediaStream | void) {
		console.log('stopping media stream');
		if (stream) {
			stream.getTracks().forEach((track) => {
				track.stop();
				stream?.removeTrack(track);
			});
		}
		if (video) {
			video.srcObject = null;
		}
	}

	async function startCapturing() {
		if (!canvas || !video || !context) return;

		const { width, height } = canvas;

		context.drawImage(video, 0, 0, width, height);
		const imageData = context.getImageData(0, 0, width, height);
		const qrCode = jsQR(imageData.data, width, height);

		if (qrCode) {
			video.pause();
			const data = qrCode.data;
			const currentUrl = window.location.origin;

			if (!data.startsWith(currentUrl)) {
				alert('Invalid QR code: please scan a QR code for this app.');
			} else {
				const room_id = data.split('/').pop();
				if (room_id) {
					try {
						await joinRoom(room_id);
						await goto(data);
						return;
					} catch (error) {
						console.error(error);
						alert('Could not connect to room');
					}
				}
			}
			video.play();
		}
		setTimeout(startCapturing, millisecondsBetweenScans);
	}

	async function handleCanPlay() {
		if (!video || !canvas) return;
		canvas.width = video.videoWidth;
		canvas.height = video.videoHeight;
		await video.play();
		startCapturing();
	}
</script>

<Stack>
	<div class="scanner">
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
	{#if video?.paused}
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
