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

	$: menuItems = [
		{ label: 'Home', action: goToPage('/') },
		{ label: 'About', action: goToPage('/about') },
		$auth?.role === 'Admin'
			? { label: 'Admin', action: goToPage('/admin') }
			: { label: 'Login', action: goToPage('/login') }
	];
</script>

<ul class={'menu ' + (direction === 'horizontal' ? 'menu-horizontal' : '')}>
	{#each menuItems as item}
		<li>
			<button
				class="btn btn-link normal-case text-lg no-animation no-underline"
				on:click={item.action}>{item.label}</button
			>
		</li>
	{/each}
</ul>
