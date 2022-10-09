<script lang="ts" context="module">
	export async function load({ params }: { params: { code: string } }) {
		await new Promise((resolve) => setTimeout(resolve, 2000));
		return { code: params.code };
	}
</script>

<script lang="ts">
	export let data: { code: string };
</script>

<h1>hey you are on the code page: {data.code}</h1>
<p>Visit <a href="/">Home</a></p>

<style lang="sass">
h1
	color: red

p
	color: blue
</style>
