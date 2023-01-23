<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/Button.svelte';
	import Hero from '$lib/Hero.svelte';
	import Scanner from '$lib/Scanner.svelte';
</script>

<Hero>
	<!-- Modal content -->
	<div class="bg-base-100 rounded-lg shadow-lg">
		<!-- Modal header -->
		<h3 class="text-xl font-semibold text-center pt-4 pb-3">Scan QR Code</h3>
		<!-- Modal body -->
		<div class="px-5 stack">
			<Scanner />
		</div>
		<!-- Modal footer -->
		<div class="flex items-center rounded p-3">
			<Button label="Cancel" type="primary" onSubmit={() => goto('/')} />
		</div>
	</div>
</Hero>
