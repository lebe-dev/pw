import { SecretContentType } from './secret';

export function getEncodedUrlSlug(
	secretId: string,
	secretContentType: SecretContentType,
	encryptionKey: string,
	additionalData: string
): string {
	let contentType: string = 'text';

	if (secretContentType === SecretContentType.File) {
		contentType = 'file';
	}

	return btoa(`${secretId}|${contentType}|${encryptionKey}|${additionalData}`);
}

export class SecretUrlSlugParts {
	secretId: string = '';
	secretContentType: string = '';
	privateKey: string = '';
	additionalData: string = '';
}

export function getEncodedUrlSlugParts(input: string): SecretUrlSlugParts {
	const decodedBase64 = atob(input);
	const parts = decodedBase64.split('|');
	const urlSlugParts = new SecretUrlSlugParts();
	urlSlugParts.secretId = parts[0];
	urlSlugParts.secretContentType = parts[1];
	urlSlugParts.privateKey = parts[2];
	urlSlugParts.additionalData = parts[3];
	return urlSlugParts;
}

export function getUrlBaseHost(): string {
	const protocol = window.location.protocol;
	const hostname = window.location.hostname;
	const port = window.location.port;

	if (port !== '' && port !== '80' && port !== '443') {
		return `${protocol}//${hostname}:${port}`;
	} else {
		return `${protocol}//${hostname}`;
	}
}
