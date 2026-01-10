import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
	getEncodedUrlSlug,
	getEncodedUrlSlugParts,
	SecretUrlSlugParts,
	getUrlBaseHost
} from './url';
import { SecretContentType } from './secret';

describe('url utilities', () => {
	describe('getEncodedUrlSlug', () => {
		it('should encode text secret correctly', () => {
			const result = getEncodedUrlSlug('secret123', SecretContentType.Text, 'abc123key', 'data456');
			const expected = btoa('secret123|text|abc123key|data456');
			expect(result).toBe(expected);
			expect(result).toBe('c2VjcmV0MTIzfHRleHR8YWJjMTIza2V5fGRhdGE0NTY=');
		});

		it('should encode file secret correctly', () => {
			const result = getEncodedUrlSlug('file789', SecretContentType.File, 'xyz789key', 'moredata');
			const expected = btoa('file789|file|xyz789key|moredata');
			expect(result).toBe(expected);
		});

		it('should handle empty strings', () => {
			const result = getEncodedUrlSlug('', SecretContentType.Text, '', '');
			expect(result).toBe(btoa('|text||'));
		});

		it('should handle special characters in secretId', () => {
			const result = getEncodedUrlSlug(
				'secret-with-dashes_and_underscores',
				SecretContentType.Text,
				'key',
				'data'
			);
			const decoded = atob(result);
			expect(decoded).toBe('secret-with-dashes_and_underscores|text|key|data');
		});

		it('should produce base64 encoded string', () => {
			const result = getEncodedUrlSlug('id', SecretContentType.Text, 'key', 'data');
			expect(result).toMatch(/^[A-Za-z0-9+/]+=*$/);
		});

		it('should handle numbers in parameters', () => {
			const result = getEncodedUrlSlug('123', SecretContentType.Text, '456', '789');
			const decoded = atob(result);
			expect(decoded).toBe('123|text|456|789');
		});

		it('should differentiate between Text and File content types', () => {
			const textResult = getEncodedUrlSlug('id', SecretContentType.Text, 'key', 'data');
			const fileResult = getEncodedUrlSlug('id', SecretContentType.File, 'key', 'data');
			expect(textResult).not.toBe(fileResult);
			expect(atob(textResult)).toContain('|text|');
			expect(atob(fileResult)).toContain('|file|');
		});
	});

	describe('getEncodedUrlSlugParts', () => {
		it('should decode text secret correctly', () => {
			const encoded = btoa('secret123|text|abc123key|data456');
			const parts = getEncodedUrlSlugParts(encoded);

			expect(parts).toBeInstanceOf(SecretUrlSlugParts);
			expect(parts.secretId).toBe('secret123');
			expect(parts.secretContentType).toBe('text');
			expect(parts.privateKey).toBe('abc123key');
			expect(parts.additionalData).toBe('data456');
		});

		it('should decode file secret correctly', () => {
			const encoded = btoa('file789|file|xyz789key|moredata');
			const parts = getEncodedUrlSlugParts(encoded);

			expect(parts.secretContentType).toBe('file');
			expect(parts.secretId).toBe('file789');
			expect(parts.privateKey).toBe('xyz789key');
			expect(parts.additionalData).toBe('moredata');
		});

		it('should handle empty parts', () => {
			const encoded = btoa('|text||');
			const parts = getEncodedUrlSlugParts(encoded);

			expect(parts.secretId).toBe('');
			expect(parts.secretContentType).toBe('text');
			expect(parts.privateKey).toBe('');
			expect(parts.additionalData).toBe('');
		});

		it('should handle parts with pipe characters in data (edge case)', () => {
			const encoded = btoa('id|text|key|data|with|pipes');
			const parts = getEncodedUrlSlugParts(encoded);

			expect(parts.secretId).toBe('id');
			expect(parts.secretContentType).toBe('text');
			expect(parts.privateKey).toBe('key');
			expect(parts.additionalData).toBe('data');
		});

		it('should handle special characters', () => {
			const encoded = btoa('special-id_123|file|key-456|data_789');
			const parts = getEncodedUrlSlugParts(encoded);

			expect(parts.secretId).toBe('special-id_123');
			expect(parts.secretContentType).toBe('file');
			expect(parts.privateKey).toBe('key-456');
			expect(parts.additionalData).toBe('data_789');
		});

		it('should return SecretUrlSlugParts instance', () => {
			const encoded = btoa('id|text|key|data');
			const parts = getEncodedUrlSlugParts(encoded);

			expect(parts.constructor.name).toBe('SecretUrlSlugParts');
			expect(parts).toHaveProperty('secretId');
			expect(parts).toHaveProperty('secretContentType');
			expect(parts).toHaveProperty('privateKey');
			expect(parts).toHaveProperty('additionalData');
		});
	});

	describe('SecretUrlSlugParts class', () => {
		it('should have default empty string values', () => {
			const parts = new SecretUrlSlugParts();

			expect(parts.secretId).toBe('');
			expect(parts.secretContentType).toBe('');
			expect(parts.privateKey).toBe('');
			expect(parts.additionalData).toBe('');
		});

		it('should allow setting properties', () => {
			const parts = new SecretUrlSlugParts();
			parts.secretId = 'test-id';
			parts.secretContentType = 'file';
			parts.privateKey = 'test-key';
			parts.additionalData = 'test-data';

			expect(parts.secretId).toBe('test-id');
			expect(parts.secretContentType).toBe('file');
			expect(parts.privateKey).toBe('test-key');
			expect(parts.additionalData).toBe('test-data');
		});
	});

	describe('getUrlBaseHost', () => {
		beforeEach(() => {
			delete (global as any).window;
			global.window = {} as any;
		});

		afterEach(() => {
			vi.restoreAllMocks();
		});

		it('should return protocol and hostname without port for default HTTP', () => {
			global.window.location = {
				protocol: 'http:',
				hostname: 'example.com',
				port: '80'
			} as any;

			expect(getUrlBaseHost()).toBe('http://example.com');
		});

		it('should return protocol and hostname without port for default HTTPS', () => {
			global.window.location = {
				protocol: 'https:',
				hostname: 'example.com',
				port: '443'
			} as any;

			expect(getUrlBaseHost()).toBe('https://example.com');
		});

		it('should include custom port', () => {
			global.window.location = {
				protocol: 'http:',
				hostname: 'localhost',
				port: '3000'
			} as any;

			expect(getUrlBaseHost()).toBe('http://localhost:3000');
		});

		it('should include custom HTTPS port', () => {
			global.window.location = {
				protocol: 'https:',
				hostname: 'example.com',
				port: '8443'
			} as any;

			expect(getUrlBaseHost()).toBe('https://example.com:8443');
		});

		it('should handle empty port', () => {
			global.window.location = {
				protocol: 'https:',
				hostname: 'example.com',
				port: ''
			} as any;

			expect(getUrlBaseHost()).toBe('https://example.com');
		});

		it('should handle IP addresses', () => {
			global.window.location = {
				protocol: 'http:',
				hostname: '192.168.1.1',
				port: '8080'
			} as any;

			expect(getUrlBaseHost()).toBe('http://192.168.1.1:8080');
		});

		it('should handle localhost', () => {
			global.window.location = {
				protocol: 'http:',
				hostname: 'localhost',
				port: '5173'
			} as any;

			expect(getUrlBaseHost()).toBe('http://localhost:5173');
		});

		it('should handle localhost without custom port (HTTP)', () => {
			global.window.location = {
				protocol: 'http:',
				hostname: 'localhost',
				port: '80'
			} as any;

			expect(getUrlBaseHost()).toBe('http://localhost');
		});

		it('should handle IPv6 addresses', () => {
			global.window.location = {
				protocol: 'http:',
				hostname: '::1',
				port: '3000'
			} as any;

			expect(getUrlBaseHost()).toBe('http://::1:3000');
		});
	});

	describe('round-trip encoding/decoding', () => {
		it('should encode and decode text secret correctly', () => {
			const original = {
				secretId: 'test-secret-123',
				contentType: SecretContentType.Text,
				encryptionKey: 'abcdef1234567890',
				additionalData: 'xyz789'
			};

			const encoded = getEncodedUrlSlug(
				original.secretId,
				original.contentType,
				original.encryptionKey,
				original.additionalData
			);

			const decoded = getEncodedUrlSlugParts(encoded);

			expect(decoded.secretId).toBe(original.secretId);
			expect(decoded.secretContentType).toBe('text');
			expect(decoded.privateKey).toBe(original.encryptionKey);
			expect(decoded.additionalData).toBe(original.additionalData);
		});

		it('should encode and decode file secret correctly', () => {
			const original = {
				secretId: 'file-456',
				contentType: SecretContentType.File,
				encryptionKey: 'filekey123',
				additionalData: 'filedata456'
			};

			const encoded = getEncodedUrlSlug(
				original.secretId,
				original.contentType,
				original.encryptionKey,
				original.additionalData
			);

			const decoded = getEncodedUrlSlugParts(encoded);

			expect(decoded.secretId).toBe(original.secretId);
			expect(decoded.secretContentType).toBe('file');
			expect(decoded.privateKey).toBe(original.encryptionKey);
			expect(decoded.additionalData).toBe(original.additionalData);
		});

		it('should handle multiple round-trips', () => {
			const original = {
				secretId: 'multi-test',
				contentType: SecretContentType.Text,
				encryptionKey: 'key123',
				additionalData: 'data456'
			};

			for (let i = 0; i < 5; i++) {
				const encoded = getEncodedUrlSlug(
					original.secretId,
					original.contentType,
					original.encryptionKey,
					original.additionalData
				);

				const decoded = getEncodedUrlSlugParts(encoded);

				expect(decoded.secretId).toBe(original.secretId);
				expect(decoded.privateKey).toBe(original.encryptionKey);
				expect(decoded.additionalData).toBe(original.additionalData);
			}
		});

		it('should preserve special characters in round-trip', () => {
			const original = {
				secretId: 'id-with_special-chars_123',
				contentType: SecretContentType.File,
				encryptionKey: 'key-456_abc',
				additionalData: 'data_789-xyz'
			};

			const encoded = getEncodedUrlSlug(
				original.secretId,
				original.contentType,
				original.encryptionKey,
				original.additionalData
			);

			const decoded = getEncodedUrlSlugParts(encoded);

			expect(decoded.secretId).toBe(original.secretId);
			expect(decoded.privateKey).toBe(original.encryptionKey);
			expect(decoded.additionalData).toBe(original.additionalData);
		});
	});
});
