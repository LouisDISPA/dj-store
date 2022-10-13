<script lang="ts">
	import { goto } from '$lib/utils';
	export let direction: 'horizontal' | 'vertical';

	let menuItems = [
		{ label: 'Home', href: '/' },
		{ label: 'About', href: '/about' },
		{ label: 'Login', href: '/login' }
	];
</script>

<ul class={'menu ' + (direction === 'horizontal' ? 'menu-horizontal' : '')}>
	{#each menuItems as item}
		<li>
			<button
				class="btn btn-link normal-case text-lg no-animation no-underline"
				on:click={() => goto(item.href)}>{item.label}</button
			>
		</li>
	{/each}
</ul>
