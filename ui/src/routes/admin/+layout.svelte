<script lang="ts">
	import { auth } from '$lib/auth';
	import { goto } from '$lib/utils';
	import Button from '$lib/components/Button.svelte';
	import Modal from '$lib/components/Modal.svelte';
</script>

{#if $auth?.role !== 'Admin'}
	<Modal>
		<span slot="title">You are not connect</span>
		<span slot="body">
			<p>
				You can't access this page, <br />
				please connect as an admin.
			</p>
		</span>
		<span slot="actions">
			<Button label="Home" type="neutral" onSubmit={() => goto('/')} />
			<Button label="Login" type="primary" onSubmit={() => goto('/login')} />
		</span>
	</Modal>
{:else}
	<slot />
{/if}
