export const EPSILON = Number.EPSILON;

export const MAX = Number.MAX_VALUE;

export const MAX_SAFE_INTEGER = Number.MAX_SAFE_INTEGER;

export const MIN = Number.MIN_VALUE;

export const MIN_SAFE_INTEGER = Number.MIN_SAFE_INTEGER;

export const NEGATIVE_INFINITY = Number.NEGATIVE_INFINITY;

export const POSITIVE_INFINITY = Number.POSITIVE_INFINITY;

// rome-ignore lint/suspicious/noShadowRestrictedNames:
export function isFinite(n: number): boolean {
	return Number.isFinite(n);
}

export function isInteger(n: number): boolean {
	return Number.isInteger(n);
}

// rome-ignore lint/suspicious/noShadowRestrictedNames:
export function isNaN(n: number): boolean {
	return Number.isNaN(n);
}

export function parseInteger(string: string, radix?: number): number {
	return Number.parseInt(string, radix);
}

// rome-ignore lint/suspicious/noShadowRestrictedNames:
export function parseFloat(string: string): number {
	return Number.parseFloat(string);
}

export function isSafeInteger(n: number): boolean {
	return Number.isSafeInteger(n);
}
