<script lang="ts">
	export let header: Array<string> | undefined = undefined;
</script>

<div class="my-2">
	<table class="table table-zebra m-auto w-full">
		{#if header}
			<thead>
				<tr>
					{#each header as head}
						<th class="font-bold text-lg">{head}</th>
					{/each}
				</tr>
			</thead>
		{/if}
		<tbody>
			<slot />
		</tbody>
	</table>
</div>
