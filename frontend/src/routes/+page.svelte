<script lang="ts">
	import { onMount } from 'svelte';
	import { generateRandomKey, getRandomAdditionalData, getRandomKeyId } from '$lib/encrypt';
	import { AES } from 'crypto-js';
	import { getEncodedUrlSlug, getUrlBaseHost } from '$lib/url';
	import { Secret, SecretDownloadPolicy, SecretTTL } from '$lib/secret';
	import PrecautionMessage from '$lib/components/PrecautionMessage.svelte';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { toast } from 'svelte-sonner';
	import { t } from 'svelte-intl-precompile';
	import { AppConfig } from '$lib/config';
	import { Button } from '$lib/components/ui/button';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import OneTimeDownload from '$lib/components/OneTimeDownload.svelte';
	import SecretLifeTime from '$lib/components/SecretLifeTime.svelte';
	import CustomPassword from '$lib/components/CustomPassword.svelte';

	let inProgress = $state(true);

	let config = $state(new AppConfig());

	let secretStored = $state(false);

	let message: string = $state('');

	let messageLength: number = $state(0);
	let messageTotal: number = $derived(config.messageMaxLength);

	let secretTTL = $state(SecretTTL.OneHour);

	let secretDownloadPolicy = $state(SecretDownloadPolicy.OneTime);

	let oneTimeDownloadMode = $derived(secretDownloadPolicy === SecretDownloadPolicy.OneTime);

	let autoGeneratePassword: boolean = $state(true);
	let customPassword: string = $state('');

	let secretUrl: string = $state('');

	let encryptButtonDisabled = $derived(
		inProgress || message.length === 0 || (!autoGeneratePassword && customPassword === '')
	);

	$inspect('secretTTL', secretTTL);
	$inspect('autoGeneratePassword', autoGeneratePassword);
	$inspect('customPassword', customPassword);
	$inspect('secretTTL', secretTTL);
	$inspect('customPassword', oneTimeDownloadMode);

	onMount(async () => {
		if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
			console.log('user prefers dark mode');
		}

		await loadConfig();
	});

	async function loadConfig() {
		const response = await fetch('/api/config', {
			method: 'GET'
		});

		if (response.status === 200) {
			config = await response.json();
			inProgress = false;
		} else {
			toast.error('Unable to load config');
		}
	}

	function onToggleDownloadPolicy() {
		if (secretDownloadPolicy === SecretDownloadPolicy.OneTime) {
			secretDownloadPolicy = SecretDownloadPolicy.Unlimited;
		} else {
			secretDownloadPolicy = SecretDownloadPolicy.OneTime;
		}
	}

	function onMessageUpdate() {
		messageLength = message.length;
	}

	async function onEncrypt() {
		let key: string = '';

		if (autoGeneratePassword) {
			key = await generateRandomKey();
		} else {
			key = customPassword;
		}

		const ciphertext = AES.encrypt(message, key).toString();

		console.log('TTL:', secretTTL);

		const secret = new Secret();
		secret.id = await getRandomKeyId();
		secret.payload = ciphertext;
		secret.ttl = secretTTL;
		secret.downloadPolicy = secretDownloadPolicy;

		const additionalData = await getRandomAdditionalData();

		if (!autoGeneratePassword) {
			key = '';
		}

		const slug = getEncodedUrlSlug(secret.id, key, additionalData);

		const baseUrl = getUrlBaseHost();

		secretUrl = `${baseUrl}/s/${slug}`;

		console.log('secret url:', secretUrl);

		const response = await fetch('/api/secret', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(secret)
		});

		const status = response.status;

		console.log('status:', status);

		if (status === 200) {
			secretStored = true;
		} else {
			toast.error('Encryption error');
		}
	}
</script>

<svelte:head>
	<title>PW</title>
	<meta name="description" content="Secure share secrets" />
</svelte:head>

<div class="text-center">
	{#if !secretStored}
		<div class="mb-2 select-none text-start text-xl">{$t('homePage.title')}</div>
		<Textarea
			placeholder={$t('homePage.messagePlaceholder')}
			rows={6}
			class="placeholder:text-md mb-2"
			maxlength={messageTotal}
			bind:value={message}
			onkeyup={() => onMessageUpdate(event)}
			autofocus={true}
		></Textarea>

		<div class="mb-5 select-none text-xs">
			<span class={messageLength === messageTotal && messageTotal !== 0 ? 'text-amber-500' : ''}
				>{messageLength} / {messageTotal}</span
			>
		</div>

		<div class="mb-3 select-none">
			{$t('homePage.secretLifetimeTitle')}:
		</div>

		<div class="mb-4 flex flex-row justify-center">
			<div>
				<SecretLifeTime bind:value={secretTTL} bind:disabled={inProgress} />
			</div>
		</div>

		<div class="justify-start">
			<div class="mb-4 flex flex-row justify-center">
				<OneTimeDownload
					checked={true}
					bind:disabled={inProgress}
					click={() => onToggleDownloadPolicy()}
				/>
			</div>

			<div class="flex flex-row justify-center">
				<CustomPassword
					bind:checked={autoGeneratePassword}
					bind:value={customPassword}
					bind:disabled={inProgress}
				/>
			</div>
		</div>

		<div class="mb-9">
			<Button
				size="lg"
				class="uppercase dark:disabled:bg-gray-700"
				disabled={encryptButtonDisabled}
				onclick={() => onEncrypt()}>{$t('homePage.encryptMessageButton')}</Button
			>
		</div>
	{:else}
		<div class="mb-2 text-start text-xl">{$t('homePage.secretUrlTitle')}</div>

		<div id="secret-url" class="text-md mb-5 select-all break-all rounded border border-accent p-5">
			{secretUrl}
		</div>

		{#if secretDownloadPolicy === SecretDownloadPolicy.OneTime}
			<PrecautionMessage message={$t('homePage.lifetime.oneTimeDownloadPrecautionMessage')} />
		{/if}

		<div class="mb-9 mt-4 text-center">
			<CopyButton data={secretUrl} label={$t('homePage.copyButton')} />
		</div>
	{/if}

	<div class="select-none text-xs text-gray-400">
		<a
			href="https://github.com/lebe-dev/pw/releases"
			target="_blank"
			class="hover:text-secondary-foreground hover:dark:text-accent">v1.8.0</a
		>

		<span class="me-1 ms-1">|</span>
		<a
			href={'https://github.com/lebe-dev/pw/blob/main/docs/faq/FAQ.' + $t('id') + '.md'}
			target="_blank"
			class="hover:text-secondary-foreground hover:dark:text-accent"
			>{$t('footerLabels.howItWorks')}</a
		>
		<span class="me-1 ms-1">|</span>
		<a
			href="https://github.com/lebe-dev/pw"
			target="_blank"
			class="hover:text-secondary-foreground hover:dark:text-accent">GITHUB</a
		>
	</div>
</div>
