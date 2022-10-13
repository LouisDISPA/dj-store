<script>
	import Dropdown from './Dropdown.svelte';
	import Menu from './Menu.svelte';
	import { env } from '$lib/env';
</script>

<div class="navbar shadow-lg justify-between">
	<div class="btn btn-ghost px-1 no-animation">
		<a href="{env.BASE_HREF}/" class="normal-case text-xl"
			><img src="{env.BASE_HREF}/favicon.png" class="inline-block w-10 h-10 mr-3" alt="logo" />DJ
			Store</a
		>
	</div>
	<div class="md:hidden">
		<Dropdown>
			<span slot="label">
				<svg class=" w-6 h-6 stroke-current"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 6h16M4 12h16M4 18h16"
					/></svg
				>
			</span>
			<span slot="content">
				<Menu direction="vertical" />
			</span>
		</Dropdown>
	</div>
	<div class="hidden md:block">
		<Menu direction="horizontal" />
	</div>
</div>
