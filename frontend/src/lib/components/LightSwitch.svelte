<script lang="ts">
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';

	import { toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/index.js';
</script>

<Button
	onclick={() => toggleMode()}
	variant="outline"
	size="icon"
	class="border-0 bg-transparent align-middle dark:bg-transparent"
	title="Toggle theme"
>
	<Sun
		class="h-[1.2rem] w-[1.2rem] rotate-90 scale-0 text-secondary-foreground transition-all hover:text-amber-400 dark:rotate-0 dark:scale-100 dark:hover:text-amber-400"
	/>
	<Moon
		class="absolute h-[1.2rem] w-[1.2rem] rotate-0 scale-100 text-secondary transition-all hover:text-primary dark:-rotate-90 dark:scale-0 dark:hover:text-amber-400"
	/>

	<span class="sr-only">Toggle theme</span>
</Button>
