<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Button.svelte';
	import Modal from '$lib/Modal.svelte';
	import Scanner from '$lib/Scanner.svelte';
</script>

<Modal>
	<span slot="title">Scan QR Code</span>
	<span slot="body">
		<Scanner />
	</span>
	<span slot="actions">
		<Button label="Cancel" type="primary" onSubmit={() => goto('/')} />
	</span>
</Modal>
