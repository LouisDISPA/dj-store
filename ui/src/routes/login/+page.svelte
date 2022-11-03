<script>
	import Button from '$lib/Button.svelte';
	import TextInput from '$lib/TextInput.svelte';
</script>

<div class="hero mt-5">
	<div class="hero-content flex-col">
		<div class="text-center lg:text-left my-3">
			<h1 class="text-2xl font-bold">Login to your account</h1>
		</div>
		<TextInput label="Username" />
		<TextInput label="Password" />
		<div class="mt-5">
			<Button label="Connect" />
		</div>
	</div>
</div>
