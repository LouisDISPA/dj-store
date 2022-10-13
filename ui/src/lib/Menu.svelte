<script lang="ts">
	export let direction: 'horizontal' | 'vertical';

	let menuItems = [
		{ label: 'Home', href: '/' },
		{ label: 'Login', href: '/login' }
	];
</script>

<ul class={'menu ' + (direction === 'horizontal' ? 'menu-horizontal' : '')}>
	{#each menuItems as item}
		<li>
			<a href={item.href} class="normal-case text-xl">{item.label}</a>
		</li>
	{/each}
</ul>
