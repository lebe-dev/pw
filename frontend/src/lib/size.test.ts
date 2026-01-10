import { describe, it, expect } from 'vitest';
import { getPrettySize } from './size';

describe('getPrettySize', () => {
	describe('bytes (Б)', () => {
		it('should format 0 bytes', () => {
			expect(getPrettySize('0', 'KB', 'MB', 'GB')).toBe('0 Б');
		});

		it('should format 500 bytes', () => {
			expect(getPrettySize('500', 'KB', 'MB', 'GB')).toBe('500 Б');
		});

		it('should format 1000 bytes (boundary)', () => {
			expect(getPrettySize('1000', 'KB', 'MB', 'GB')).toBe('1000 Б');
		});

		it('should format 1 byte', () => {
			expect(getPrettySize('1', 'KB', 'MB', 'GB')).toBe('1 Б');
		});

		it('should format 999 bytes', () => {
			expect(getPrettySize('999', 'KB', 'MB', 'GB')).toBe('999 Б');
		});
	});

	describe('kilobytes', () => {
		it('should format 1001 bytes as KB', () => {
			expect(getPrettySize('1001', 'KB', 'MB', 'GB')).toBe('1.0 KB');
		});

		it('should format 5000 bytes as KB', () => {
			expect(getPrettySize('5000', 'KB', 'MB', 'GB')).toBe('5.0 KB');
		});

		it('should format 500000 bytes as KB', () => {
			expect(getPrettySize('500000', 'KB', 'MB', 'GB')).toBe('500.0 KB');
		});

		it('should format 999999 bytes (boundary)', () => {
			expect(getPrettySize('999999', 'KB', 'MB', 'GB')).toBe('1000.0 KB');
		});

		it('should format 10000 bytes as KB', () => {
			expect(getPrettySize('10000', 'KB', 'MB', 'GB')).toBe('10.0 KB');
		});
	});

	describe('megabytes', () => {
		it('should format 1000000 bytes as MB', () => {
			expect(getPrettySize('1000000', 'KB', 'MB', 'GB')).toBe('1.0 MB');
		});

		it('should format 5000000 bytes as MB', () => {
			expect(getPrettySize('5000000', 'KB', 'MB', 'GB')).toBe('5.0 MB');
		});

		it('should format 500000000 bytes as MB', () => {
			expect(getPrettySize('500000000', 'KB', 'MB', 'GB')).toBe('500.0 MB');
		});

		it('should format 999999999 bytes (boundary)', () => {
			expect(getPrettySize('999999999', 'KB', 'MB', 'GB')).toBe('1000.0 MB');
		});

		it('should format 50000000 bytes as MB', () => {
			expect(getPrettySize('50000000', 'KB', 'MB', 'GB')).toBe('50.0 MB');
		});
	});

	describe('gigabytes', () => {
		it('should format 1000000000 bytes as GB', () => {
			expect(getPrettySize('1000000000', 'KB', 'MB', 'GB')).toBe('1.0 GB');
		});

		it('should format 5000000000 bytes as GB', () => {
			expect(getPrettySize('5000000000', 'KB', 'MB', 'GB')).toBe('5.0 GB');
		});

		it('should format 1500000000000 bytes as GB', () => {
			expect(getPrettySize('1500000000000', 'KB', 'MB', 'GB')).toBe('1500.0 GB');
		});

		it('should format 10000000000 bytes as GB', () => {
			expect(getPrettySize('10000000000', 'KB', 'MB', 'GB')).toBe('10.0 GB');
		});
	});

	describe('boundary testing', () => {
		it('should test boundary at 1000-1001', () => {
			expect(getPrettySize('1000', 'KB', 'MB', 'GB')).toBe('1000 Б');
			expect(getPrettySize('1001', 'KB', 'MB', 'GB')).toBe('1.0 KB');
		});

		it('should test boundary at 999999-1000000', () => {
			expect(getPrettySize('999999', 'KB', 'MB', 'GB')).toBe('1000.0 KB');
			expect(getPrettySize('1000000', 'KB', 'MB', 'GB')).toBe('1.0 MB');
		});

		it('should test boundary at 999999999-1000000000', () => {
			expect(getPrettySize('999999999', 'KB', 'MB', 'GB')).toBe('1000.0 MB');
			expect(getPrettySize('1000000000', 'KB', 'MB', 'GB')).toBe('1.0 GB');
		});
	});

	describe('custom labels', () => {
		it('should handle custom KB label', () => {
			expect(getPrettySize('5000', 'кБ', 'MB', 'GB')).toBe('5.0 кБ');
		});

		it('should handle custom MB label', () => {
			expect(getPrettySize('5000000', 'KB', 'МБ', 'GB')).toBe('5.0 МБ');
		});

		it('should handle custom GB label', () => {
			expect(getPrettySize('5000000000', 'KB', 'MB', 'ГБ')).toBe('5.0 ГБ');
		});

		it('should handle all custom labels', () => {
			expect(getPrettySize('5000', 'кБ', 'МБ', 'ГБ')).toBe('5.0 кБ');
			expect(getPrettySize('5000000', 'кБ', 'МБ', 'ГБ')).toBe('5.0 МБ');
			expect(getPrettySize('5000000000', 'кБ', 'МБ', 'ГБ')).toBe('5.0 ГБ');
		});
	});

	describe('edge cases', () => {
		it('should handle string with leading zeros', () => {
			expect(getPrettySize('005000', 'KB', 'MB', 'GB')).toBe('5.0 KB');
		});

		it('should handle negative values', () => {
			expect(getPrettySize('-500', 'KB', 'MB', 'GB')).toBe('-500 Б');
		});

		it('should handle decimal strings (parseInt truncates)', () => {
			expect(getPrettySize('1234.56', 'KB', 'MB', 'GB')).toBe('1.2 KB');
		});

		it('should handle very large number', () => {
			expect(getPrettySize('999999999999', 'KB', 'MB', 'GB')).toBe('1000.0 GB');
		});

		it('should handle empty-like values', () => {
			const result = getPrettySize('', 'KB', 'MB', 'GB');
			expect(result).toMatch(/NaN|0/);
		});
	});
});
