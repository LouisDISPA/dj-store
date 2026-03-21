<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children?: Snippet } = $props();
</script>

<div class="hero my-10">
	<div class="hero-content">
		{@render children?.()}
	</div>
</div>
