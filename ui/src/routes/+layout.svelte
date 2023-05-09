<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { tryRecallUser } from '$lib/auth';
	import NavBar from '$lib/components/NavBar.svelte';

	onMount(tryRecallUser);
</script>

<NavBar />
<slot />
