<script lang="ts">
	import { onMount } from 'svelte';
	import { AES, enc } from 'crypto-js';
	import { toast } from 'svelte-sonner';
	import { getEncodedUrlSlugParts } from '$lib/url';
	import { SecretDownloadPolicy } from '$lib/secret';
	import PrecautionMessage from '$lib/components/PrecautionMessage.svelte';
	import { error } from '@sveltejs/kit';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { t } from 'svelte-intl-precompile';
	import { AppConfig } from '$lib/config';
	import { Button } from '$lib/components/ui/button';

	let { data } = $props();

	let loading: boolean = $state(true);
	let config: AppConfig = $state(new AppConfig());

	let message: string = $state('');

	let notFound: boolean = $state(false);

	let possibleReasonsItems: string[] = $state([]);

	onMount(async () => {
		possibleReasonsItems = $t('secretNotFoundPage.possibleReasonsItems').split('\n');

		let response = await fetch('/api/config', {
			method: 'GET'
		});

		config = await response.json();

		try {
			const slugParts = getEncodedUrlSlugParts(data.secretId);

			response = await fetch(`/api/secret/${slugParts.secretId}`, {
				method: 'GET'
			});

			const status = response.status;

			if (status === 200) {
				data.secret = await response.json();

				message = AES.decrypt(data.secret.payload, slugParts.privateKey).toString(enc.Utf8);

				loading = false;
			} else if (status === 400) {
				notFound = true;
				loading = false;
			} else {
				error(500, 'Internal error');
				toast.error('Internal error');
			}
		} catch (e) {
			console.error(e);
			notFound = true;
		}
	});

	async function onRemoveUrl(secretId: string) {
		if (confirm($t('secretUrlPage.removeConfirmMessage'))) {
			try {
				const response = await fetch(`/api/secret/${secretId}`, {
					method: 'DELETE'
				});

				const status = response.status;

				loading = false;

				if (status === 200) {
					location.href = '/';
				}
				if (status === 400) {
					notFound = true;
				} else {
					error(500, 'Internal error');
				}
			} catch (e) {
				console.error(e);
				notFound = true;
			}
		}
	}
</script>

<svelte:head>
	<title>{$t('secretUrlPage.title')}</title>
	<meta name="description" content="Secret page" />
</svelte:head>

{#if !loading}
	{#if !notFound}
		<div class="mb-4 select-none text-start text-xl">{$t('secretUrlPage.title')}</div>

		<div
			id="secret-url"
			class="text-md border-prim mb-5 whitespace-pre-wrap break-all rounded border p-5 font-mono dark:border-accent"
		>
			{message}
		</div>

		{#if data.secret.downloadPolicy === SecretDownloadPolicy.OneTime}
			<PrecautionMessage message={$t('secretUrlPage.oneTimeDownloadPrecautionMessage')} />

			<div class="mt-3 text-center">
				<CopyButton data={message} label={$t('homePage.copyButton')} />
			</div>
		{:else}
			<div class="columns-2">
				<div class="column-xs ps-1">
					<CopyButton
						data={message}
						label={$t('secretUrlPage.copyButton')}
						onclick={() => onRemoveUrl(data.secret.id)}
					/>
				</div>
				<div class="column-xs pe-1 text-right">
					<Button
						variant="outline"
						size="sm"
						class="hover:bg-destructive hover:text-primary-foreground dark:hover:bg-destructive dark:hover:text-secondary-foreground"
						onclick={() => onRemoveUrl(data.secret.id)}
					>
						<div class="flex items-center">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="24"
								height="24"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="1.5"
								stroke-linecap="round"
								stroke-linejoin="round"
								class="lucide lucide-trash-2 me-1 inline-block"
								><path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path
									d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
								/><line x1="10" x2="10" y1="11" y2="17" /><line
									x1="14"
									x2="14"
									y1="11"
									y2="17"
								/></svg
							>

							{$t('secretUrlPage.removeButton')}
						</div></Button
					>
				</div>
			</div>
		{/if}
	{:else}
		<div class="mb-4 text-start text-xl">{$t('secretNotFoundPage.title')}</div>

		<div id="secret-url" class="text-md mb-5 break-all rounded">
			<div class="mb-2">{$t('secretNotFoundPage.possibleReasonsText')}:</div>

			<ul class="ps-6">
				{#each possibleReasonsItems as reason}
					<li class="mb-1 list-disc">{reason}</li>
				{/each}
			</ul>
		</div>
	{/if}
{:else}
	{$t('messages.loadingTitle')}
{/if}
