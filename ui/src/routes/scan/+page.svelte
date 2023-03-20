<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/components/Button.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import Scanner from '$lib/components/Scanner.svelte';
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
