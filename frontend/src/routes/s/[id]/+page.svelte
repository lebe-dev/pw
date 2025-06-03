<script lang="ts">
	import { onMount } from 'svelte';
	import { AES, enc } from 'crypto-js';
	import { toast } from 'svelte-sonner';
	import { getEncodedUrlSlugParts } from '$lib/url';
	import { Secret, SecretContentType, SecretDownloadPolicy } from '$lib/secret';
	import PrecautionMessage from '$lib/components/PrecautionMessage.svelte';
	import { error } from '@sveltejs/kit';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { t } from 'svelte-intl-precompile';
	import { Button } from '$lib/components/ui/button';
	import { base64ToBlob } from '$lib/file.js';
	import { getPrettySize } from '$lib/size.js';

	let { data } = $props();

	let inProgress: boolean = $state(true);
	let unlocking: boolean = $state(false);

	let secret: Secret = $state(new Secret());

	let message: string = $state('');

	let notFound: boolean = $state(false);
	let invalidPassword: boolean = $state(false);

	let possibleReasonsItems: string[] = $state([]);

	let customPasswordInput: any;

	let askForPassword: boolean = $state(false);
	let customPassword: string = $state('');

	$inspect('secret', secret);
	$inspect('askForPassword', askForPassword);
	$inspect('customPassword', customPassword);

	function onDecryptWithCustomPassword() {
		try {
			unlocking = true;
			message = AES.decrypt(secret.payload, customPassword).toString(enc.Utf8);

			if (message === '') {
				invalidPassword = true;
				unlocking = false;

				setTimeout(() => {
					customPassword = '';
					customPasswordInput.focus();
				}, 100);
			}
		} catch (e) {
			console.error(e);
			invalidPassword = true;
			customPassword = '';
			unlocking = false;
			setTimeout(() => {
				customPassword = '';
				customPasswordInput.focus();
			}, 100);
		}
	}

	onMount(async () => {
		possibleReasonsItems = $t('secretNotFoundPage.possibleReasonsItems').split('\n');

		try {
			const slugParts = getEncodedUrlSlugParts(data.secretId);

			if (slugParts.privateKey === '') {
				askForPassword = true;
			}

			let response = await fetch(`/api/secret/${slugParts.secretId}`, {
				method: 'GET'
			});

			const status = response.status;

			if (status === 200) {
				secret = await response.json();

				if (!askForPassword) {
					message = AES.decrypt(secret.payload, slugParts.privateKey).toString(enc.Utf8);
				}

				inProgress = false;
			} else if (status === 400) {
				notFound = true;
				inProgress = false;
			} else {
				toast.error('Internal error');
				error(500, 'Internal error');
			}
		} catch (e) {
			console.error(e);
			notFound = true;
		}
	});

	function onDownloadFile() {
		const blob = base64ToBlob(message, secret.metadata!.type);
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = secret.metadata!.name;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
	}

	async function onRemoveUrl(secretId: string) {
		if (confirm($t('secretUrlPage.removeConfirmMessage'))) {
			try {
				const response = await fetch(`/api/secret/${secretId}`, {
					method: 'DELETE'
				});

				const status = response.status;

				inProgress = false;

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

{#if !inProgress}
	{#if !notFound}
		{#if secret.contentType === SecretContentType.Text}
			<div class="mb-4 select-none ps-1 text-start text-xl">{$t('secretUrlPage.textTitle')}</div>
		{:else if secret.contentType === SecretContentType.File}
			<div class="mb-4 select-none ps-1 text-start text-xl">{$t('secretUrlPage.fileTitle')}</div>
		{/if}

		{#if askForPassword && message === ''}
			<div class="mb-2 pb-0">{$t('secretUrlPage.customPasswordTitle')}</div>
			<input
				bind:this={customPasswordInput}
				autofocus={true}
				type="password"
				value={customPassword}
				maxlength="32"
				disabled={unlocking}
				oninput={(e) => (customPassword = (e.target as HTMLInputElement).value)}
				class="mb-3 flex h-10 w-64 rounded-md border-2 border-input bg-background px-3 py-2 text-base placeholder:text-muted-foreground focus-visible:border-accent focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
			/>

			<Button
				disabled={customPassword.length === 0 || unlocking}
				onclick={() => onDecryptWithCustomPassword()}
				>{$t('secretUrlPage.customPasswordButton')}</Button
			>

			{#if invalidPassword}
				<div class="mb-2 mt-4 pb-0">
					{$t('secretUrlPage.invalidPasswordMessage')}
				</div>
			{/if}
		{:else if secret.contentType === SecretContentType.Text}
			<div
				id="secret-url"
				class="text-md border-prim mb-5 whitespace-pre-wrap break-all rounded border p-5 font-mono dark:border-accent"
			>
				{message}
			</div>
		{:else if secret.contentType === SecretContentType.File}
			<div id="secret-url" class="text-md border-prim mb-5 rounded border p-5 dark:border-accent">
				<div class="mb-1 text-sm">
					<span class="text-muted-foreground">{$t('secretUrlPage.fileNameTitle')}:</span>
					{secret.metadata?.name}
				</div>
				<div class="mb-5 text-sm">
					<span class="text-muted-foreground">{$t('secretUrlPage.fileSizeTitle')}:</span>
					{getPrettySize(
						secret.metadata.size.toString(),
						$t('sizes.kb'),
						$t('sizes.mb'),
						$t('sizes.gb')
					)}
				</div>

				<div class="columns-2">
					<div class="column-xs">
						<Button
							class="uppercase dark:disabled:bg-gray-700"
							title={$t('secretUrlPage.downloadButton')}
							onclick={() => onDownloadFile()}
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="24"
								height="24"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								class="lucide lucide-arrow-down-to-line-icon lucide-arrow-down-to-line"
								><path d="M12 17V3" /><path d="m6 11 6 6 6-6" /><path d="M19 21H5" /></svg
							>

							{$t('secretUrlPage.downloadButton')}
						</Button>
					</div>
					<div class="column-xs pe-1 text-end">
						<Button
							variant="outline"
							class="hover:bg-destructive hover:text-primary-foreground dark:hover:bg-destructive dark:hover:text-secondary-foreground"
							onclick={() => onRemoveUrl(secret.id)}
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
			</div>
		{/if}

		{#if message !== ''}
			{#if secret.downloadPolicy === SecretDownloadPolicy.OneTime}
				<PrecautionMessage message={$t('secretUrlPage.oneTimeDownloadPrecautionMessage')} />

				{#if secret.contentType === SecretContentType.Text}
					<div class="mt-3 text-center">
						<CopyButton data={message} label={$t('homePage.copyButton')} />
					</div>
				{/if}
			{:else if secret.contentType === SecretContentType.Text}
				<div class="columns-2">
					<div class="column-xs ps-1">
						<CopyButton
							data={message}
							label={$t('secretUrlPage.copyButton')}
							onclick={() => onRemoveUrl(secret.id)}
						/>
					</div>

					<div class="column-xs pe-1 text-right">
						<Button
							variant="outline"
							size="sm"
							class="hover:bg-destructive hover:text-primary-foreground dark:hover:bg-destructive dark:hover:text-secondary-foreground"
							onclick={() => onRemoveUrl(secret.id)}
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
