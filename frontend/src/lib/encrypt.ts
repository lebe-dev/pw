export async function getRandomKeyId() {
	return generateRandomString(8);
}

export async function getRandomAdditionalData() {
	return generateRandomString(8);
}

export async function generateRandomKey() {
	return getRandomHexDataWithLength(32);
}

export async function getRandomHexDataWithLength(length: number) {
	const key = await globalThis.crypto.subtle.generateKey(
		{
			name: 'AES-GCM',
			length: length * 8
		},
		true,
		['encrypt', 'decrypt']
	);

	const exportedKeyBuffer = await globalThis.crypto.subtle.exportKey('raw', key);
	const randomKey = new Uint8Array(exportedKeyBuffer);

	return buf2hex(randomKey);
}

export function buf2hex(buffer: Uint8Array): string {
	return Array.from(buffer, (byte) => {
		return ('0' + (byte & 0xff).toString(16)).slice(-2);
	}).join('');
}

const CHARSET = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';

// Largest multiple of the charset size that fits in a byte (4 * 62 = 248). Bytes at or
// above it are discarded instead of taken modulo 62, which would otherwise make the
// first six characters of the charset ~25% more likely than the rest.
const MAX_UNBIASED_BYTE = Math.floor(256 / CHARSET.length) * CHARSET.length;

// Drawn from the Web Crypto CSPRNG rather than Math.random: these strings become the
// secret id and the additional data carried in the sharing URL, so a caller who can
// predict them can reach a secret they were never given the link to.
function generateRandomString(length: number): string {
	let randomString = '';

	while (randomString.length < length) {
		const bytes = new Uint8Array(length - randomString.length);
		globalThis.crypto.getRandomValues(bytes);

		for (const byte of bytes) {
			if (byte >= MAX_UNBIASED_BYTE) continue;
			randomString += CHARSET[byte % CHARSET.length];
		}
	}

	return randomString;
}
