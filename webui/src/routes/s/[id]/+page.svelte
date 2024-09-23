<script lang="ts">
	import {onMount} from "svelte";
	import type {PageData} from "./$types";
	import {AES, enc} from "crypto-js";
	import {getEncodedUrlSlugParts} from "$lib/url";
	import {SecretDownloadPolicy} from "$lib/secret";
	import PrecautionMessage from "../../../components/PrecautionMessage.svelte";
	import {error} from "@sveltejs/kit";
	import CopyButton from "../../../components/CopyButton.svelte";
	import type {Locale} from "$lib/locale";
	import {showError} from "$lib/notifications";

	export let data: PageData;

	let loading: boolean = true;

	let message: string = '';

	let notFound: boolean = false;

	onMount(async () => {

		let currentLocale = await import(`../../../lib/locale/en.json`);

		data.locale = currentLocale.default as Locale;

		let response = await fetch('/api/config', {
			method: 'GET'
		});

		data.config = await response.json();

		try {
			let currentLocale = await import(`../../../lib/locale/${data.config.localeId}.json`);

			data.locale = currentLocale.default as Locale;

			console.log('current locale:', data.locale);

		} catch (e) {
			console.error(`could not load locale '${data.config.localeId}' from locale/${data.config.localeId}.json`)
			showError(data.locale.errors.loadingData);
		}

		try {
			const slugParts = getEncodedUrlSlugParts(data.secretId);

			response = await fetch(`/api/secret/${slugParts.secretId}`, {
				method: 'GET'
			});

			const status = response.status;

			loading = false;

			if (status === 200) {
				data.secret = await response.json();
				message = AES.decrypt(data.secret.payload, slugParts.privateKey).toString(enc.Utf8);

			}
			if (status === 400) {
				notFound = true;

			} else {
				error(500, 'Internal error')
			}

		} catch (e) {
			console.error(e);
			notFound = true;
		}

	});

	async function onRemoveUrl(secretId: string) {
		if (confirm(data.locale.secretUrlPage.removeConfirmMessage)) {
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
					error(500, 'Internal error')
				}

			} catch (e) {
				console.error(e);
				notFound = true;
			}
		}
	}
</script>

<svelte:head>
	<title>{data.locale.secretUrlPage.title}</title>
	<meta name="description" content="Secret page"/>
</svelte:head>

{#if !loading}
	{#if !notFound}
		<div class="text-xl mb-4 text-start">{data.locale.secretUrlPage.title}</div>

		<div id="secret-url" class="text-md mb-5 bg-base-200 rounded p-5 break-all select-all font-mono whitespace-pre-wrap">
			{message}
		</div>

		{#if data.secret.downloadPolicy === SecretDownloadPolicy.OneTime}
			<PrecautionMessage message={data.locale.secretUrlPage.oneTimeDownloadPrecautionMessage}/>

		{:else}
			<div class="columns-2">
				<div class="column-xs ps-1">
					<CopyButton data={message} label={data.locale.secretUrlPage.copyButton}/>
				</div>
				<div class="column-xs text-right pe-1">
					<button class="px-3 py-1.5 ms-3 btn btn-sm hover:bg-red-600 hover:text-primary-content hover:border-red-600 hover:text-white uppercase rounded cursor-pointer text-md"
					   on:click={() => onRemoveUrl(data.secret.id)}>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash3 inline-block" viewBox="0 0 16 16">
							<path d="M6.5 1h3a.5.5 0 0 1 .5.5v1H6v-1a.5.5 0 0 1 .5-.5M11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3A1.5 1.5 0 0 0 5 1.5v1H1.5a.5.5 0 0 0 0 1h.538l.853 10.66A2 2 0 0 0 4.885 16h6.23a2 2 0 0 0 1.994-1.84l.853-10.66h.538a.5.5 0 0 0 0-1zm1.958 1-.846 10.58a1 1 0 0 1-.997.92h-6.23a1 1 0 0 1-.997-.92L3.042 3.5zm-7.487 1a.5.5 0 0 1 .528.47l.5 8.5a.5.5 0 0 1-.998.06L5 5.03a.5.5 0 0 1 .47-.53Zm5.058 0a.5.5 0 0 1 .47.53l-.5 8.5a.5.5 0 1 1-.998-.06l.5-8.5a.5.5 0 0 1 .528-.47M8 4.5a.5.5 0 0 1 .5.5v8.5a.5.5 0 0 1-1 0V5a.5.5 0 0 1 .5-.5"/>
						</svg>

						{data.locale.secretUrlPage.removeButton}
					</button>
				</div>
			</div>
		{/if}

	{:else}
		<div class="text-xl mb-4 text-start">{data.locale.secretNotFoundPage.title}</div>

		<div id="secret-url" class="text-md rounded break-all mb-5">
			<div class="mb-2">{data.locale.secretNotFoundPage.possibleReasonsText}:</div>

			<ul class="ps-6">
				{#each data.locale.secretNotFoundPage.possibleReasonsItems as reason}
					<li class="list-disc mb-1">{reason}</li>
				{/each}
			</ul>
		</div>
	{/if}

{:else}
	{data.locale.messages.loadingTitle}
{/if}