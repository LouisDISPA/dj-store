<script lang="ts">
	import '../app.css';
	import NavBar from '$lib/components/NavBar.svelte';
	import { joinRoom, tryRecallUser } from '$lib/auth';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { goto } from '$lib/utils';

	onMount(async () => {
		const reccaled = await tryRecallUser();

		if ($page.route.id?.includes('admin')) {
			return;
		}

		// If we have a room_id in the URL, and we're not already in that room
		if (!reccaled && $page.params.room_id) {
			joinRoom($page.params.room_id).catch((err) => {
				console.error(err);
				goto('/?error=Could not connect to room');
			});
		}
	});
</script>

<NavBar />
<slot />
