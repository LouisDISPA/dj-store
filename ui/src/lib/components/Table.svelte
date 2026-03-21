<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		header = undefined,
		children
	}: {
		header?: Array<string> | undefined;
		children?: Snippet;
	} = $props();
</script>

<div class="my-2">
	<table class="table table-zebra m-auto shadow-lg table-xs md:table-md">
		{#if header}
			<thead>
				<tr>
					{#each header as head (head)}
						<th class="font-bold text-lg">{head}</th>
					{/each}
				</tr>
			</thead>
		{/if}
		<tbody>
			{@render children?.()}
		</tbody>
	</table>
</div>
