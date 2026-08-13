import { describe, it, expect, vi, afterEach } from 'vitest';
import { fileToBase64, base64ToBlob } from './file';

describe('fileToBase64', () => {
	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('should return the payload of the data URL without the prefix', async () => {
		const file = new File(['hello'], 'hello.txt', { type: 'text/plain' });

		expect(await fileToBase64(file)).toBe(btoa('hello'));
	});

	it('should handle an empty file', async () => {
		const file = new File([], 'empty.txt', { type: 'text/plain' });

		expect(await fileToBase64(file)).toBe('');
	});

	it('should reject with an Error carrying the reader message when reading fails', async () => {
		stubFailingFileReader(new DOMException('permission denied', 'NotReadableError'));

		const file = new File(['hello'], 'hello.txt', { type: 'text/plain' });

		await expect(fileToBase64(file)).rejects.toThrow(new Error('permission denied'));
	});

	it('should reject with an Error naming the file when the reader reports no error', async () => {
		stubFailingFileReader(null);

		const file = new File(['hello'], 'broken.txt', { type: 'text/plain' });

		await expect(fileToBase64(file)).rejects.toThrow(new Error('Failed to read file: broken.txt'));
	});
});

describe('base64ToBlob', () => {
	it('should decode the payload into a blob of the given content type', async () => {
		const blob = base64ToBlob(btoa('hello'), 'text/plain');

		expect(blob.type).toBe('text/plain');
		expect(await blob.text()).toBe('hello');
	});

	it('should preserve binary bytes outside the ASCII range', async () => {
		const bytes = Uint8Array.from([0x00, 0x7f, 0x80, 0xff]);
		const base64 = btoa(String.fromCharCode(...bytes));

		const blob = base64ToBlob(base64, 'application/octet-stream');

		expect(new Uint8Array(await blob.arrayBuffer())).toEqual(bytes);
	});

	it('should decode an empty payload into an empty blob', async () => {
		expect((await base64ToBlob('', 'text/plain').text()).length).toBe(0);
	});
});

// Replaces FileReader with one that always fails, so the rejection path can be exercised.
function stubFailingFileReader(error: DOMException | null) {
	vi.stubGlobal(
		'FileReader',
		class {
			error = error;
			result: string | null = null;
			onload: (() => void) | null = null;
			onerror: (() => void) | null = null;

			readAsDataURL() {
				queueMicrotask(() => this.onerror?.());
			}
		}
	);
}
