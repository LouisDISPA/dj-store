<script lang="ts">
	export let id: number;
	export let title: string;
	export let artist: string;
	export let votes: number | undefined = undefined;
</script>

<tr id={id.toString()}>
	<td>
		<div class="avatar">
			<div class="mask mask-squircle w-12 h-12">
				<img src="https://picsum.photos/200/300" alt="music poster" />
			</div>
		</div>
	</td>
	<td>
		<div>
			<div class="font-bold">{title}</div>
			<div class="text-sm opacity-50">{artist}</div>
		</div>
	</td>
	<td>
		<div class="rating">
			<input type="checkbox" class="mask mask-heart bg-red-500 opacity-30 checked:opacity-80" />
		</div>
	</td>
</tr>
