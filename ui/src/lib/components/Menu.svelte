<script lang="ts">
	import { goto } from '$lib/utils';
	import { auth, disconnect } from '$lib/auth';

	function goToPage(href: string): () => void {
		return () => goto(href);
	}

	function logout(): void {
		disconnect();
		goto('/login');
	}

	const menu_logged = [
		{ label: 'Admin', action: goToPage('/admin') },
		{ label: 'Logout', action: logout }
	];

	const menu_not_logged = [{ label: 'Login', action: goToPage('/login') }];

	const menu_basic = [
		{ label: 'Home', action: goToPage('/') },
		{ label: 'About', action: goToPage('/about') }
	];

	let menu_items = $derived(
		menu_basic.concat($auth?.role === 'Admin' ? menu_logged : menu_not_logged)
	);
</script>

{#each menu_items as item (item.label)}
	<li>
		<button onclick={item.action}>{item.label}</button>
	</li>
{/each}
