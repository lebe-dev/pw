<script lang="ts">
	import {onMount} from "svelte";
	import type {PageData} from "./$types";
	import {AES, enc} from "crypto-js";
	import {getEncodedUrlSlugParts} from "$lib/url";
	import {SecretDownloadPolicy} from "$lib/secret";
	import PrecautionMessage from "../../../components/PrecautionMessage.svelte";
	import {error} from "@sveltejs/kit";
	import ClipboardJS from "clipboard";

	export let data: PageData;

	let loading: boolean = true;

	let message: string = '';

	let notFound: boolean = false;

	onMount(async () => {
		new ClipboardJS('#copy-to-clipboard');

		let response = await fetch('/api/config', {
			method: 'GET'
		});

		data.config = await response.json();

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
		if (confirm(data.config.locale.secretUrlPage.removeConfirmMessage)) {
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
	<title>{data.config.locale.secretUrlPage.title}</title>
	<meta name="description" content="Secret page"/>
</svelte:head>

{#if !loading}
	{#if !notFound}
		<div class="text-xl mb-4 text-start">{data.config.locale.secretUrlPage.title}</div>

		<div id="secret-url" class="text-md mb-5 bg-base-200 rounded p-5 break-all select-all font-mono whitespace-pre-wrap">
			{message}
		</div>

		{#if data.secret.downloadPolicy === SecretDownloadPolicy.OneTime}
			<PrecautionMessage message={data.config.locale.secretUrlPage.oneTimeDownloadPrecautionMessage}/>

		{:else}
			<div class="columns-2">
				<div class="column-xs ps-1">
					<button id="copy-to-clipboard" data-clipboard-target="#secret-url"
							class="px-3 py-1.5 btn btn-sm hover:btn-accent uppercase rounded cursor-pointer text-md">

						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4 inline-block">
							<path stroke-linecap="round" stroke-linejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
						</svg> {data.config.locale.secretUrlPage.copyButton}
					</button>
				</div>
				<div class="column-xs text-right pe-1">
					<button class="px-3 py-1.5 ms-3 btn btn-sm hover:bg-red-600 hover:text-primary-content hover:border-red-600 hover:text-white uppercase rounded cursor-pointer text-md"
					   on:click={() => onRemoveUrl(data.secret.id)}>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash3 inline-block" viewBox="0 0 16 16">
							<path d="M6.5 1h3a.5.5 0 0 1 .5.5v1H6v-1a.5.5 0 0 1 .5-.5M11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3A1.5 1.5 0 0 0 5 1.5v1H1.5a.5.5 0 0 0 0 1h.538l.853 10.66A2 2 0 0 0 4.885 16h6.23a2 2 0 0 0 1.994-1.84l.853-10.66h.538a.5.5 0 0 0 0-1zm1.958 1-.846 10.58a1 1 0 0 1-.997.92h-6.23a1 1 0 0 1-.997-.92L3.042 3.5zm-7.487 1a.5.5 0 0 1 .528.47l.5 8.5a.5.5 0 0 1-.998.06L5 5.03a.5.5 0 0 1 .47-.53Zm5.058 0a.5.5 0 0 1 .47.53l-.5 8.5a.5.5 0 1 1-.998-.06l.5-8.5a.5.5 0 0 1 .528-.47M8 4.5a.5.5 0 0 1 .5.5v8.5a.5.5 0 0 1-1 0V5a.5.5 0 0 1 .5-.5"/>
						</svg>

						{data.config.locale.secretUrlPage.removeButton}
					</button>
				</div>
			</div>
		{/if}

	{:else}
		<div class="text-xl mb-4 text-start">{data.config.locale.secretNotFoundPage.title}</div>

		<div id="secret-url" class="text-md rounded break-all mb-5">
			<div class="mb-2">{data.config.locale.secretNotFoundPage.possibleReasonsText}:</div>

			<ul class="ps-6">
				{#each data.config.locale.secretNotFoundPage.possibleReasonsItems as reason}
					<li class="list-disc mb-1">{reason}</li>
				{/each}
			</ul>
		</div>
	{/if}

{:else}
	{data.config.locale.messages.loadingTitle}
{/if}