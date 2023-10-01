export function getEncodedUrlSlug(secretId: string, encryptionKey: string, additionalData: string): string {
    return btoa(`${secretId}|${encryptionKey}|${additionalData}`)
}


export class SecretUrlSlugParts {
    secretId: string = '';
    privateKey: string = '';
    additionalData: string = '';
}

export function getEncodedUrlSlugParts(input: string): SecretUrlSlugParts {
    const decodedBase64 = atob(input);
    const parts = decodedBase64.split('|');
    const urlSlugParts = new SecretUrlSlugParts();
    urlSlugParts.secretId = parts[0];
    urlSlugParts.privateKey = parts[1];
    urlSlugParts.additionalData = parts[2];
    return urlSlugParts
}

export function getUrlBaseHost(): string {
    const protocol = window.location.protocol;
    const hostname = window.location.hostname;
    const port = window.location.port;

    if (port !== '' && port !== '80' && port !== '443') {
        return `${protocol}//${hostname}:${port}`

    } else {
        return `${protocol}//${hostname}`
    }
}