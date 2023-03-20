<script lang="ts">
	import { goto } from '$lib/utils';
	import { auth, disconnect } from '$lib/auth';

	export let direction: 'horizontal' | 'vertical';

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

	$: menu_items = menu_basic.concat($auth?.role === 'Admin' ? menu_logged : menu_not_logged);
</script>

<ul class="menu" class:menu-horizontal={direction === 'horizontal'}>
	{#each menu_items as item}
		<li>
			<button
				class="btn btn-link normal-case text-lg no-animation no-underline"
				on:click={item.action}>{item.label}</button
			>
		</li>
	{/each}
</ul>
