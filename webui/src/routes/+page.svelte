<script lang="ts">
	import {onMount} from "svelte";
	import type {PageData} from "./$types";
	import {generateRandomKey, getRandomAdditionalData, getRandomKeyId} from "$lib/encrypt";
	import {AES} from "crypto-js";
	import {getEncodedUrlSlug, getUrlBaseHost} from "$lib/url";
	import {Secret, SecretDownloadPolicy, SecretTTL} from "$lib/secret";
	import PrecautionMessage from "../components/PrecautionMessage.svelte";
	import {showError} from "$lib/notifications";
	import RadioButton from "../components/RadioButton.svelte";
	import CheckBox from "../components/CheckBox.svelte";
	import CopyButton from "../components/CopyButton.svelte";
	import {t} from 'svelte-intl-precompile'

	let secretStored = false;

	let message: string = '';

	let messageLength: number = 0;
	let messageTotal: number = 0;

	let secretTTL: SecretTTL = SecretTTL.OneHour;
	let secretDownloadPolicy: SecretDownloadPolicy = SecretDownloadPolicy.OneTime;

	let secretUrl: string = '';

	let checkBoxColorClass: string = 'text-accent';
	let checkBoxAdditionalClasses: string = '';

	export let data: PageData;

	onMount(async () => {

		if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
			console.log('prefer dark mode')
		}

		const response = await fetch('/api/config', {
			method: 'GET'
		});

		if (response.status === 200) {
			data.config = await response.json();
			console.log('config:', data.config);
			messageTotal = data.config.messageMaxLength;

		} else {
			showError('Unable to load config');
		}
	});

	function onToggleDownloadPolicy() {
		if (secretDownloadPolicy === SecretDownloadPolicy.OneTime) {
			checkBoxColorClass = 'text-warning';
			checkBoxAdditionalClasses = 'text-warning';
			secretDownloadPolicy = SecretDownloadPolicy.Unlimited

		} else {
			checkBoxColorClass = 'text-accent';
			checkBoxAdditionalClasses = '';
			secretDownloadPolicy = SecretDownloadPolicy.OneTime
		}
	}

	function onMessageUpdate() {
		messageLength = message.length;
	}

	async function onEncrypt() {
		const key = await generateRandomKey();

		const ciphertext = AES.encrypt(message, key).toString();

		const secret = new Secret();
		secret.id = await getRandomKeyId();
		secret.payload = ciphertext;
		secret.ttl = secretTTL;
		secret.downloadPolicy = secretDownloadPolicy;

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

		} else {
			showError('Encryption error');
		}
	}
</script>

<svelte:head>
	<title>PW</title>
	<meta name="description" content="Secure share secrets" />
</svelte:head>

<div id="content-x" class="text-center">
	{#if !secretStored}

		<div class="text-xl mb-2 text-start select-none">{$t('homePage.title')}</div>
		<textarea class="w-full border-2 rounded border-accent bg-secondary outline-0 p-3"
				  placeholder={$t('homePage.messagePlaceholder')}
				  rows="5"
				  maxlength={messageTotal}
				  bind:value={message}
				  on:keyup={onMessageUpdate}
				  autofocus={true}/>

		<div class="text-xs mb-5 select-none">
			<span class={messageLength === messageTotal && messageTotal !== 0 ? 'text-red-600' : ''}>{messageLength} / {messageTotal}</span>
		</div>

		<div class="mb-3 select-none">
			{$t('homePage.secretLifetimeTitle')}:
		</div>

		<div class="flex flex-row gap-0 text-center justify-center mb-4">
			<div>
				<RadioButton enabled={secretTTL === SecretTTL.OneHour}
							 toggle={() => secretTTL = SecretTTL.OneHour}
							 text={$t('homePage.lifetime.oneHour')}/>

				<RadioButton enabled={secretTTL === SecretTTL.TwoHours}
							 toggle={() => secretTTL = SecretTTL.TwoHours}
							 text={$t('homePage.lifetime.twoHours')}/>
			</div>

			<div class="text-left">
				<RadioButton enabled={secretTTL === SecretTTL.OneDay}
							 toggle={() => secretTTL = SecretTTL.OneDay}
							 text={$t('homePage.lifetime.oneDay')}/>

				<RadioButton enabled={secretTTL === SecretTTL.OneWeek}
							 toggle={() => secretTTL = SecretTTL.OneWeek}
							 text={$t('homePage.lifetime.oneWeek')}/>
			</div>
		</div>

		<div class="mb-7">
			<CheckBox enabled={secretDownloadPolicy === SecretDownloadPolicy.OneTime}
					  toggle={onToggleDownloadPolicy}
					  checkBoxColorClass={checkBoxColorClass}
					  componentAdditionalClasses={checkBoxAdditionalClasses}
					  text={$t('homePage.lifetime.oneTimeDownload')}/>
		</div>

		<div class="mb-9">
			<button disabled={messageLength === 0} on:click={onEncrypt}
					class="px-3 py-2 w-64 btn btn-md btn-neutral hover:btn-accent rounded uppercase disabled:pointer-events-none">{$t('homePage.encryptMessageButton')}</button>
		</div>

	{:else}

		<div class="text-xl mb-2 text-start">{$t('homePage.secretUrlTitle')}</div>

		<div id="secret-url" class="text-md mb-5 border border-accent rounded p-5 select-all break-all">
			{secretUrl}
		</div>

		{#if secretDownloadPolicy === SecretDownloadPolicy.OneTime}
			<PrecautionMessage message={$t('homePage.lifetime.oneTimeDownloadPrecautionMessage')}/>
		{/if}

		<div class="mb-9 text-center mt-4">
			<CopyButton data={secretUrl} label={$t('homePage.copyButton')}/>
		</div>

	{/if}

	<div class="text-gray-400 text-sm select-none">
		v1.6.0 <span class="ms-1 me-1">|</span>
		<a href={'https://github.com/lebe-dev/pw/blob/main/docs/faq/FAQ.' + $t('id') + '.md'}
		   target="_blank" class="hover:text-accent">{$t('footerLabels.howItWorks')}</a>
		<span class="ms-1 me-1">|</span> <a href="https://github.com/lebe-dev/pw"
											target="_blank" class="hover:text-accent">GITHUB</a>
	</div>
</div>

<style>
	#content-x {
		min-width: 700px !important;
	}
</style>