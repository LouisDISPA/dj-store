<script lang="ts">
	import { auth, connect } from '$lib/auth';
	import Button from '$lib/components/Button.svelte';
	import TextInput from '$lib/components/TextInput.svelte';
	import { goto } from '$lib/utils';
	import { onMount } from 'svelte';

	let username = '';
	let password = '';
	let error: string | null = null;

	onMount(() => {
		if ($auth?.role === 'Admin') {
			goto('/admin');
			return;
		}
	});

	function onSubmit() {
		connect(username, password)
			.then(() => goto('/admin'))
			.catch(() => {
				error = 'Invalid username or password';
				setTimeout(() => (error = null), 3000);
			});
	}
</script>

<div class="hero mt-5">
	<div class="hero-content flex-col">
		<div class="text-center lg:text-left my-3">
			<h1 class="text-2xl font-bold">Login to your account</h1>
		</div>
		<TextInput label="Username" bind:value={username} />
		<TextInput label="Password" bind:value={password} type="password" {onSubmit} />
		<div class="mt-5">
			<Button label="Connect" type="primary" {onSubmit} />
		</div>
		{#if error}
			<div class="badge badge-error gap-2">{error}</div>
		{/if}
	</div>
</div>
