<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { t } from 'svelte-intl-precompile';

	let { checked = $bindable(true), value = $bindable(''), disabled = $bindable(false) } = $props();

	let customPasswordInput;

	function focusOnCustomPasswordInput() {
		setTimeout(() => {
			customPasswordInput.focus();
		}, 100);
	}
</script>

<div class="mb-5">
	<div class="flex items-center space-x-2">
		<Checkbox
			id="use-custom-password"
			bind:checked
			aria-labelledby="use-custom-password-label"
			class="rounded dark:checked:border-accent dark:checked:bg-accent"
			onclick={() => {
				focusOnCustomPasswordInput();
			}}
			{disabled}
		/>
		<span>
			<Label
				id="use-custom-password-label"
				for="use-custom-password"
				class="cursor-pointer text-sm font-normal leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
			>
				{$t('homePage.autoGeneratePassword')}
			</Label>
		</span>
	</div>
	{#if !checked}
		<div>
			<input
				bind:value
				bind:this={customPasswordInput}
				maxlength="32"
				placeholder={$t('homePage.customPassword')}
				class="mt-2 flex h-10 w-full rounded-md border-2 border-input bg-background px-3 py-2 text-base placeholder:text-muted-foreground focus-visible:border-accent focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
				{disabled}
			/>
		</div>
	{/if}
</div>
