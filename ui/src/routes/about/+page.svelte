<script>
	import Hero from '$lib/components/Hero.svelte';
</script>

<Hero>
	<div class="max-w-4xl">
		<h1 class="text-3xl font-bold">About DJ-Store</h1>
		<p class="pt-6">
			DJ-Store is an application that allows DJs to get recommendations from people on the
			dancefloor. The idea is simple: dancers can scan a QR code and vote for the songs they want to
			hear. The DJ can then use this feedback to adjust their playlist and keep the dancefloor
			packed.
		</p>
		<p class="pt-6">
			This is a proof of concept and destined to be used at private parties and events to get
			feedback. I do not recommend using this for commercial purposes. If you want to use this at
			your event, you can contact me, I would be happy to help out.
		</p>
		<p class="pt-6">
			For any issue or question you can contact me by email:
			<a class="link" href="mailto:louis.dispa@outlook.fr?subject=[DJ-Store] ..."
				>louis.dispa@outlook.fr</a
			>
		</p>
		<p class="pt-6">
			The project source code is (soon) available on <a
				class="link"
				href="https://github.com/LouisDISPA/dj-store">GitHub</a
			>.
		</p>
	</div>
</Hero>
