<script lang="ts">
	import "../app.css";
</script>

<div class="container max-w-full max-h-full h-screen">
	<nav data-sveltekit-reload class="bg-gray-900 text-white p-4 flex flex-row items-center justify-start">
		<div class="font-bold inline-block basis-1/8 me-5 text-lg">
			<a href="/" title="Back to home page">PW</a>
		</div>
	</nav>

	<div class="container w-full max-w-5xl bg-white shadow-md p-10">
		<slot />
	</div>

</div>