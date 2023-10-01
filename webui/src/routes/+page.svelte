<script lang="ts">
	import {onMount} from "svelte";
	import type {PageData} from "./$types";
	import {generateRandomKey, getRandomAdditionalData, getRandomKeyId} from "$lib/encrypt";
	import {AES} from "crypto-js";
	import {getEncodedUrlSlug, getUrlBaseHost} from "$lib/url";
	import {Secret, SecretDownloadPolicy, SecretTTL} from "$lib/secret";
	import ClipboardJS from "clipboard";
	import PrecautionMessage from "../components/PrecautionMessage.svelte";

	let loading: boolean = true;

	let secretStored = false;

	let message: string = '';

	let messageLength: number = 0;
	let messageTotal: number = 0;

	let secretTTL: SecretTTL = SecretTTL.OneHour;
	let secretDownloadPolicy: SecretDownloadPolicy = SecretDownloadPolicy.Unlimited;

	let secretUrl: string = '';

	export let data: PageData;

	onMount(async () => {

		new ClipboardJS('#copy-to-clipboard');

		const response = await fetch('/api/config', {
			method: 'GET'
		});

		data.config = await response.json();

		console.log('config:', data.config);

		messageTotal = data.config.messageMaxLength;

		loading = false;
	});

	function onToggleDownloadPolicy() {
		console.log(`current: ${secretDownloadPolicy}`);

		if (secretDownloadPolicy === SecretDownloadPolicy.OneTime) {
			secretDownloadPolicy = SecretDownloadPolicy.Unlimited

		} else {
			secretDownloadPolicy = SecretDownloadPolicy.OneTime
		}

		console.log(`after: ${secretDownloadPolicy}`);
	}

	function onMessageUpdate() {
		messageLength = message.length;
	}

	async function onEncrypt() {
		const key = await generateRandomKey();
		console.log('KEY:', key, 'length:', key.length);

		const ciphertext = AES.encrypt(message, key).toString();

		const secret = new Secret();
		secret.id = await getRandomKeyId();
		secret.payload = ciphertext;
		secret.ttl = secretTTL;
		secret.downloadPolicy = secretDownloadPolicy;

		console.log('secret:', secret);

		const additionalData = await getRandomAdditionalData();

		const slug = getEncodedUrlSlug(secret.id, key, additionalData);

		const baseUrl = getUrlBaseHost();

		secretUrl = `${baseUrl}/s/${slug}`;

		console.log('secret url:', secretUrl);

		const response = await fetch('/api/secret', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(secret),
		});

		const status = response.status;

		console.log('status:', status);

		if (status === 200) {
			secretStored = true;
		}
	}
</script>

<svelte:head>
	<title>PW</title>
	<meta name="description" content="Secure share secrets" />
</svelte:head>

{#if !loading}

	<div class="text-center">
		{#if !secretStored}

			<div class="text-xl mb-2 text-start">{data.config.locale.homePage.title}</div>
			<textarea class="w-full border-2 rounded border-gray-600 outline-0 p-3"
					  placeholder="The data will be encrypted in the browser"
					  rows="5"
					  maxlength={messageTotal}
					  bind:value={message}
					  on:keyup={onMessageUpdate}
					  autofocus={true}/>

			<div class="text-xs text-gray-500 mb-5">
				<span class={messageLength === messageTotal && messageTotal !== 0 ? 'text-red-600' : ''}>{messageLength} / {messageTotal}</span>
			</div>

			<div class="mb-3">
				{data.config.locale.homePage.secretLifetimeTitle}:
			</div>

			<div class="mb-4">
				<label for="ttl-one-hour" class="me-3">
					<input id="ttl-one-hour" name="ttl" type="radio" checked={true}
						   on:click={() => secretTTL = SecretTTL.OneHour}> {data.config.locale.homePage.lifetime.oneHour}
				</label>

				<label for="ttl-two-hours" class="me-3">
					<input id="ttl-two-hours" name="ttl" type="radio"
						   on:click={() => secretTTL = SecretTTL.TwoHours}> {data.config.locale.homePage.lifetime.twoHours}
				</label>

				<label for="ttl-one-day">
					<input id="ttl-one-day" name="ttl" type="radio"
						   on:click={() => secretTTL = SecretTTL.OneDay}> {data.config.locale.homePage.lifetime.oneDay}
				</label>
			</div>

			<div class="mb-7">
				<label for="one-time-link">
					<input id="one-time-link" type="checkbox" on:click={onToggleDownloadPolicy}> {data.config.locale.homePage.lifetime.oneTimeDownload}
				</label>
			</div>

			<div class="mb-9">
				<button disabled={messageLength === 0} on:click={onEncrypt}
						class="px-3 py-2 w-64 bg-gray-800 hover:bg-gray-700 disabled:bg-gray-500 text-white rounded disabled:pointer-events-none">{data.config.locale.homePage.encryptMessageButton}</button>
			</div>

		{:else}

			<div class="text-xl mb-2 text-start">{data.config.locale.homePage.secretUrlTitle}</div>

			<div id="secret-url" class="text-md bg-gray-50 mb-5 rounded p-5 break-all">
				{secretUrl}
			</div>

			{#if secretDownloadPolicy === SecretDownloadPolicy.OneTime}
				<PrecautionMessage message={data.config.locale.homePage.lifetime.oneTimeDownloadPrecautionMessage}/>
			{/if}

			<div class="mb-9 text-center">
				<button id="copy-to-clipboard" data-clipboard-target="#secret-url"
						class="px-3 py-1.5 bg-gray-800 hover:bg-gray-700 disabled:bg-gray-500 text-white rounded disabled:pointer-events-none">

					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4 inline-block me-0.5 mb-1">
						<path stroke-linecap="round" stroke-linejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
					</svg>

					{data.config.locale.homePage.copyButton}
				</button>
			</div>

		{/if}

		<div class="text-gray-400 text-sm">
			v1.2.0 <span class="ms-1 me-1">|</span> <a href={'https://github.com/lebe-dev/pw/blob/main/docs/faq/FAQ.' + data.config.locale.id + '.md'} target="_blank">{data.config.locale.footerLabels.howItWorks}</a> <span class="ms-1 me-1">|</span> <a href="https://github.com/lebe-dev/pw" target="_blank">GITHUB</a>
		</div>
	</div>

{:else}
	{data.config.locale.messages.loadingTitle}
{/if}