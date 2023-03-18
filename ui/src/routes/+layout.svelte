<script lang="ts">
	import '../app.css';
	import NavBar from '$lib/NavBar.svelte';
	import { joinRoom, tryRecallUser } from '$lib/auth';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { auth } from '$lib/auth';

	onMount(async () => {
		const reccaled = await tryRecallUser();
		if (
			// If we have a room_id in the URL, and we're not already in that room
			(!reccaled && $page.params.room_id) ||
			// Or if we're already in a room, but the URL room_id is different
			($auth?.room_id && $page.params.room_id && $page.params.room_id !== $auth?.room_id)
		) {
			await joinRoom($page.params.room_id);
		}
	});
</script>

<NavBar />
<slot />
