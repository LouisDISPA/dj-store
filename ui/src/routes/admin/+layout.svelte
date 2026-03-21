<script lang="ts">
	import { auth } from '$lib/auth';
	import { goto } from '$lib/utils';
	import Button from '$lib/components/Button.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import type { Snippet } from 'svelte';

	let { children }: { children?: Snippet } = $props();
</script>

{#if $auth?.role !== 'Admin'}
	<Modal>
		{#snippet title()}You are not connect{/snippet}
		{#snippet body()}
			<p>
				You can't access this page, <br />
				please connect as an admin.
			</p>
		{/snippet}
		{#snippet actions()}
			<Button label="Home" type="neutral" onSubmit={() => goto('/')} />
			<Button label="Login" type="primary" onSubmit={() => goto('/login')} />
		{/snippet}
	</Modal>
{:else}
	{@render children?.()}
{/if}
