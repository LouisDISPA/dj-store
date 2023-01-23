<script>
	import Button from '$lib/Button.svelte';
	import TextInput from '$lib/TextInput.svelte';
	import { env, goto } from '$lib/utils';

	let username = '';
	let password = '';

	async function login() {
		const response = await fetch(`${env.API_URL}/api/admin/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username: username,
				password: password
			})
		});
		if (!response.ok) {
			alert(`Login failed: ${await response.text()}`);
			return;
		}
		const data = await response.json();
		const authToken = data['access_token'];
		localStorage.setItem('authToken', authToken);
		goto('/admin');
	}
</script>

<div class="hero mt-5">
	<div class="hero-content flex-col">
		<div class="text-center lg:text-left my-3">
			<h1 class="text-2xl font-bold">Login to your account</h1>
		</div>
		<TextInput label="Username" bind:value={username} />
		<TextInput label="Password" bind:value={password} type="password" onSubmit={() => login()} />
		<div class="mt-5">
			<Button label="Connect" type="primary" onSubmit={() => login()} />
		</div>
	</div>
</div>
