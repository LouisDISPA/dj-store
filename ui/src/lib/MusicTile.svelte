<script lang="ts">
	export let id: number;
	export let title: string;
	export let artist: string;
	export let is_voted = false;
	export let votes: number | undefined = undefined;

	export let onVote: (is_voted: boolean, id: number) => void;

	function toogleVote() {
		onVote(!is_voted, id);
		is_voted = !is_voted;
		if (votes) {
			votes += is_voted ? 1 : -1;
		}
	}
</script>

<tr id={id.toString()}>
	<!-- <td>
		<div class="avatar">
			<div class="mask mask-squircle w-12 h-12">
				<img src="https://picsum.photos/200/300" alt="music poster" />
			</div>
		</div>
	</td> -->
	<td>
		<div class="text-max text-sm opacity-50 truncate">{artist}</div>
		<div class="text-max  font-bold truncate">{title}</div>
	</td>
	<td>
		<div class="rating flex items-center">
			<input
				type="checkbox"
				class="mask mask-heart bg-red-500 opacity-30 checked:opacity-80"
				on:change={toogleVote}
				checked={is_voted}
			/>
			{#if votes}
				<p class="ml-2">
					{votes}
				</p>
			{/if}
		</div>
	</td>
</tr>

<style>
	.text-max {
		width: 65vw;
	}
</style>
