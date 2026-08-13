export async function fileToBase64(file: File): Promise<string> {
	return new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.readAsDataURL(file);
		reader.onload = () => {
			const base64 = reader.result as string;
			const base64Content = base64.split(',')[1];
			resolve(base64Content);
		};
		// FileReader hands out a DOMException (or nothing at all), which is not an Error —
		// wrap it so callers can rely on the rejection reason being one.
		reader.onerror = () =>
			reject(new Error(reader.error?.message ?? `Failed to read file: ${file.name}`));
	});
}

export function base64ToBlob(base64: string, contentType: string): Blob {
	// atob() yields latin1 characters only, so each one is a single code point.
	const byteArray = Uint8Array.from(atob(base64), (char) => char.codePointAt(0) ?? 0);
	return new Blob([byteArray], { type: contentType });
}
