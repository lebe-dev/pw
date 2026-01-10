import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
	buf2hex,
	getRandomKeyId,
	getRandomAdditionalData,
	generateRandomKey,
	getRandomHexDataWithLength
} from './encrypt';

describe('encrypt utilities', () => {
	describe('buf2hex', () => {
		it('should convert empty Uint8Array to empty string', () => {
			const buffer = new Uint8Array([]);
			expect(buf2hex(buffer)).toBe('');
		});

		it('should convert single byte 0x00 to "00"', () => {
			const buffer = new Uint8Array([0x00]);
			expect(buf2hex(buffer)).toBe('00');
		});

		it('should convert single byte 0xff to "ff"', () => {
			const buffer = new Uint8Array([0xff]);
			expect(buf2hex(buffer)).toBe('ff');
		});

		it('should convert multiple bytes correctly', () => {
			const buffer = new Uint8Array([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
			expect(buf2hex(buffer)).toBe('0123456789abcdef');
		});

		it('should handle zero-padding correctly', () => {
			const buffer = new Uint8Array([0x0a, 0x0f]);
			expect(buf2hex(buffer)).toBe('0a0f');
		});

		it('should convert all zeros correctly', () => {
			const buffer = new Uint8Array([0x00, 0x00, 0x00]);
			expect(buf2hex(buffer)).toBe('000000');
		});

		it('should handle single digit hex values with padding', () => {
			const buffer = new Uint8Array([0x01, 0x02, 0x03, 0x04, 0x05]);
			expect(buf2hex(buffer)).toBe('0102030405');
		});

		it('should handle large arrays', () => {
			const buffer = new Uint8Array(256);
			for (let i = 0; i < 256; i++) {
				buffer[i] = i;
			}
			const result = buf2hex(buffer);
			expect(result).toHaveLength(512);
			expect(result).toMatch(/^[0-9a-f]{512}$/);
		});

		it('should handle mix of high and low values', () => {
			const buffer = new Uint8Array([0x00, 0xff, 0x7f, 0x80, 0x01]);
			expect(buf2hex(buffer)).toBe('00ff7f8001');
		});

		it('should produce lowercase hex characters', () => {
			const buffer = new Uint8Array([0xab, 0xcd, 0xef]);
			const result = buf2hex(buffer);
			expect(result).toBe('abcdef');
			expect(result).not.toContain('A');
			expect(result).not.toContain('B');
		});
	});

	describe('getRandomKeyId', () => {
		afterEach(() => {
			vi.restoreAllMocks();
		});

		it('should return 8 character string', async () => {
			const result = await getRandomKeyId();
			expect(result).toHaveLength(8);
		});

		it('should return alphanumeric characters only', async () => {
			const result = await getRandomKeyId();
			expect(result).toMatch(/^[a-zA-Z0-9]{8}$/);
		});

		it('should return different values on subsequent calls', async () => {
			const results = new Set<string>();
			for (let i = 0; i < 100; i++) {
				results.add(await getRandomKeyId());
			}
			expect(results.size).toBeGreaterThan(90);
		});

		it('should use Math.random for randomness', async () => {
			const mockRandom = vi.spyOn(Math, 'random');
			mockRandom.mockReturnValue(0.5);

			const result = await getRandomKeyId();

			expect(mockRandom).toHaveBeenCalled();
			expect(result).toHaveLength(8);
		});

		it('should generate consistent output with mocked Math.random', async () => {
			let callCount = 0;
			const values = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
			vi.spyOn(Math, 'random').mockImplementation(() => {
				return values[callCount++ % values.length];
			});

			const result1 = await getRandomKeyId();
			const result2 = await getRandomKeyId();

			expect(result1).toHaveLength(8);
			expect(result2).toHaveLength(8);
		});
	});

	describe('getRandomAdditionalData', () => {
		afterEach(() => {
			vi.restoreAllMocks();
		});

		it('should return 8 character string', async () => {
			const result = await getRandomAdditionalData();
			expect(result).toHaveLength(8);
		});

		it('should return alphanumeric characters only', async () => {
			const result = await getRandomAdditionalData();
			expect(result).toMatch(/^[a-zA-Z0-9]{8}$/);
		});

		it('should return different values on subsequent calls', async () => {
			const results = new Set<string>();
			for (let i = 0; i < 100; i++) {
				results.add(await getRandomAdditionalData());
			}
			expect(results.size).toBeGreaterThan(90);
		});

		it('should be independent from getRandomKeyId', async () => {
			const keyId = await getRandomKeyId();
			const additionalData = await getRandomAdditionalData();

			expect(keyId).not.toBe(additionalData);
		});
	});

	describe('Web Crypto API functions', () => {
		beforeEach(() => {
			const mockGenerateKey = vi.fn();
			const mockExportKey = vi.fn();

			mockGenerateKey.mockImplementation(async (algorithm: any) => {
				return {
					type: 'secret',
					algorithm: { name: 'AES-GCM', length: algorithm.length }
				};
			});

			mockExportKey.mockImplementation(async (format: string, key: any) => {
				const length = key.algorithm.length / 8;
				return new Uint8Array(Array.from({ length }, (_, i) => i % 256)).buffer;
			});

			global.window = {
				crypto: {
					subtle: {
						generateKey: mockGenerateKey,
						exportKey: mockExportKey
					}
				}
			} as any;
		});

		afterEach(() => {
			vi.restoreAllMocks();
		});

		describe('generateRandomKey', () => {
			it('should generate 64-character hex key (32 bytes)', async () => {
				const key = await generateRandomKey();
				expect(key).toHaveLength(64);
			});

			it('should only contain hex characters', async () => {
				const key = await generateRandomKey();
				expect(key).toMatch(/^[0-9a-f]{64}$/);
			});

			it('should call Web Crypto API with correct parameters', async () => {
				await generateRandomKey();
				expect(window.crypto.subtle.generateKey).toHaveBeenCalledWith(
					{ name: 'AES-GCM', length: 256 },
					true,
					['encrypt', 'decrypt']
				);
			});

			it('should call exportKey with correct parameters', async () => {
				await generateRandomKey();
				expect(window.crypto.subtle.exportKey).toHaveBeenCalled();
			});

			it('should generate different keys on subsequent calls', async () => {
				const mockExportKey = vi.fn();
				mockExportKey
					.mockResolvedValueOnce(new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8]).buffer)
					.mockResolvedValueOnce(new Uint8Array([9, 10, 11, 12, 13, 14, 15, 16]).buffer);

				(window.crypto.subtle.exportKey as any) = mockExportKey;

				const key1 = await generateRandomKey();
				const key2 = await generateRandomKey();

				expect(key1).not.toBe(key2);
			});
		});

		describe('getRandomHexDataWithLength', () => {
			it('should generate hex string of correct length for 1 byte', async () => {
				const result = await getRandomHexDataWithLength(1);
				expect(result).toHaveLength(2);
				expect(result).toMatch(/^[0-9a-f]{2}$/);
			});

			it('should generate hex string of correct length for 16 bytes', async () => {
				const result = await getRandomHexDataWithLength(16);
				expect(result).toHaveLength(32);
				expect(result).toMatch(/^[0-9a-f]{32}$/);
			});

			it('should generate hex string of correct length for 32 bytes', async () => {
				const result = await getRandomHexDataWithLength(32);
				expect(result).toHaveLength(64);
				expect(result).toMatch(/^[0-9a-f]{64}$/);
			});

			it('should generate hex string of correct length for 64 bytes', async () => {
				const result = await getRandomHexDataWithLength(64);
				expect(result).toHaveLength(128);
				expect(result).toMatch(/^[0-9a-f]{128}$/);
			});

			it('should call Web Crypto API with correct bit length', async () => {
				const length = 16;
				await getRandomHexDataWithLength(length);

				expect(window.crypto.subtle.generateKey).toHaveBeenCalledWith(
					{ name: 'AES-GCM', length: length * 8 },
					true,
					['encrypt', 'decrypt']
				);
			});

			it('should handle custom length of 8 bytes', async () => {
				const result = await getRandomHexDataWithLength(8);
				expect(result).toHaveLength(16);
				expect(result).toMatch(/^[0-9a-f]{16}$/);
			});

			it('should convert exported key buffer to hex correctly', async () => {
				const mockExportKey = vi.fn().mockResolvedValue(
					new Uint8Array([0x0a, 0x0b, 0x0c, 0x0d]).buffer
				);

				(window.crypto.subtle.exportKey as any) = mockExportKey;

				const result = await getRandomHexDataWithLength(4);
				expect(result).toBe('0a0b0c0d');
			});
		});

		describe('error handling', () => {
			it('should propagate errors from generateKey', async () => {
				const mockGenerateKey = vi.fn().mockRejectedValue(new Error('Crypto not available'));
				(window.crypto.subtle.generateKey as any) = mockGenerateKey;

				await expect(generateRandomKey()).rejects.toThrow('Crypto not available');
			});

			it('should propagate errors from exportKey', async () => {
				const mockExportKey = vi.fn().mockRejectedValue(new Error('Export failed'));
				(window.crypto.subtle.exportKey as any) = mockExportKey;

				await expect(generateRandomKey()).rejects.toThrow('Export failed');
			});
		});
	});

	describe('integration tests', () => {
		beforeEach(() => {
			const mockGenerateKey = vi.fn();
			const mockExportKey = vi.fn();

			let keyCounter = 0;
			mockGenerateKey.mockImplementation(async (algorithm: any) => {
				return {
					type: 'secret',
					algorithm: { name: 'AES-GCM', length: algorithm.length },
					id: keyCounter++
				};
			});

			mockExportKey.mockImplementation(async (format: string, key: any) => {
				const length = key.algorithm.length / 8;
				const offset = key.id || 0;
				return new Uint8Array(Array.from({ length }, (_, i) => (i + offset) % 256)).buffer;
			});

			global.window = {
				crypto: {
					subtle: {
						generateKey: mockGenerateKey,
						exportKey: mockExportKey
					}
				}
			} as any;
		});

		afterEach(() => {
			vi.restoreAllMocks();
		});

		it('should generate valid hex from random bytes', async () => {
			const key = await generateRandomKey();
			const bytes = new Uint8Array(
				key.match(/.{2}/g)!.map((byte) => parseInt(byte, 16))
			);

			expect(bytes).toHaveLength(32);
			expect(buf2hex(bytes)).toBe(key);
		});

		it('should work with different byte lengths', async () => {
			const lengths = [1, 8, 16, 32, 64];

			for (const length of lengths) {
				const hex = await getRandomHexDataWithLength(length);
				expect(hex).toHaveLength(length * 2);
				expect(hex).toMatch(/^[0-9a-f]+$/);
			}
		});
	});
});
