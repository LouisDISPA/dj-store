<script>
	import Dropdown from './Dropdown.svelte';
	import Menu from './Menu.svelte';
	import { env, goto } from '$lib/utils';
</script>

<div class="flex p-2 shadow-lg justify-between bg-base-100">
	<button class="btn btn-ghost px-2 no-animation" on:click={() => goto('/')}>
		<img src="{env.BASE_HREF}/favicon.png" class="inline-block w-10 h-10 mr-2" alt="logo" />
		<h1 class="normal-case text-xl mx-1">DJ Store</h1>
	</button>
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
	<div class="hidden md:block mr-2">
		<Menu direction="horizontal" />
	</div>
</div>
