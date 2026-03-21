<script>
	import Menu from './Menu.svelte';
	import { env, goto } from '$lib/utils';
</script>

<div class="navbar bg-base-100 shadow-sm">
	<div class="navbar-start">
		<button class="btn btn-ghost text-xl" onclick={() => goto('/')}>
			<img src="{env.BASE_HREF}/favicon.png" class="w-10 h-10" alt="logo" />
			DJ Store
		</button>
	</div>
	<div class="navbar-end">
		<div class="sm:hidden">
			<details class="dropdown dropdown-end">
				<summary class="btn btn-ghost">
					<svg class="w-6 h-6 stroke-current"
						><path stroke-linecap="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" /></svg
					>
				</summary>
				<ul class="menu dropdown-content bg-base-200 rounded-box z-10 w-52 p-2 shadow-sm">
					<Menu />
				</ul>
			</details>
		</div>
		<ul class="menu menu-horizontal hidden sm:flex">
			<Menu />
		</ul>
	</div>
</div>
