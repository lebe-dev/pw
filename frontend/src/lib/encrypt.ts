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
	const key = await window.crypto.subtle.generateKey(
		{
			name: 'AES-GCM',
			length: length * 8
		},
		true,
		['encrypt', 'decrypt']
	);

	const exportedKeyBuffer = await window.crypto.subtle.exportKey('raw', key);
	const randomKey = new Uint8Array(exportedKeyBuffer);

	return buf2hex(randomKey);
}

export function buf2hex(buffer: Uint8Array): string {
	return Array.from(buffer, (byte) => {
		return ('0' + (byte & 0xff).toString(16)).slice(-2);
	}).join('');
}

function generateRandomString(length: number): string {
	const charset = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
	let randomString = '';
	for (let i = 0; i < length; i++) {
		const randomIndex = Math.floor(Math.random() * charset.length);
		randomString += charset[randomIndex];
	}
	return randomString;
}
