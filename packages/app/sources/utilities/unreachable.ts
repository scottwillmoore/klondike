// https://doc.rust-lang.org/std/macro.unreachable.html
// https://github.com/LinusU/ts-unreachable

export class UnreachableError extends Error {}

export function unreachable(): never {
	throw new UnreachableError();
}
