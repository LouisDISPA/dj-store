<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { tryRecallUser } from '$lib/auth';
	import NavBar from '$lib/components/NavBar.svelte';
	import type { Snippet } from 'svelte';

	let { children }: { children?: Snippet } = $props();

	onMount(tryRecallUser);
</script>

<NavBar />
{@render children?.()}
