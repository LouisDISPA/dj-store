<div class="custom-stack">
	<slot />
</div>

<style>
	.custom-stack > :global(*) {
		display: grid;
		grid-column-start: 1;
		grid-row-start: 1;
	}

	.custom-stack {
		display: inline-grid;
		place-items: center;
		align-items: center;
	}
</style>
