<script lang="ts">
	import {onMount} from "svelte";
	import type {PageData} from "./$types";
	import {AES, enc} from "crypto-js";
	import {getEncodedUrlSlugParts} from "$lib/url";
	import {SecretDownloadPolicy} from "$lib/secret";
	import PrecautionMessage from "../../../components/PrecautionMessage.svelte";
	import {error} from "@sveltejs/kit";

	export let data: PageData;

	let loading: boolean = true;

	let message: string = '';

	let notFound: boolean = false;

	onMount(async () => {
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
</script>

<svelte:head>
	<title>{data.config.locale.secretUrlPage.title}</title>
	<meta name="description" content="Secret page"/>
</svelte:head>

{#if !loading}
	{#if !notFound}
		<div class="text-xl mb-2 text-start">{data.config.locale.secretUrlPage.title}</div>

		<div id="secret-url" class="text-md bg-gray-50 mb-5 rounded p-5 break-all whitespace-pre-wrap">
			{message}
		</div>

		{#if data.secret.downloadPolicy === SecretDownloadPolicy.OneTime}
			<PrecautionMessage message={data.config.locale.secretUrlPage.oneTimeDownloadPrecautionMessage}/>
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