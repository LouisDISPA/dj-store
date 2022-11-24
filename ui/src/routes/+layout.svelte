<script lang="ts">
	import '../app.css';
	import NavBar from '$lib/NavBar.svelte';
	export const ssr = false;
</script>

<NavBar />
<slot />
