<script lang="ts">
	import '../app.css';
	import NavBar from '$lib/NavBar.svelte';
	import { joinRoom, tryRecallUser } from '$lib/auth';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { auth } from '$lib/auth';
	import { goto } from '$lib/utils';

	onMount(async () => {
		const reccaled = await tryRecallUser();

		if ($page.route.id?.includes('admin')) {
			return;
		}

		if (
			// If we have a room_id in the URL, and we're not already in that room
			(!reccaled && $page.params.room_id) ||
			// Or if we're already in a room, but the URL room_id is different
			(reccaled &&
				$auth?.room_id &&
				$page.params.room_id &&
				$page.params.room_id !== $auth?.room_id)
		) {
			joinRoom($page.params.room_id).catch((err) => {
				console.error(err);
				goto('/?error=Could not connect to room');
			});
		}
	});
</script>

<NavBar />
<slot />
