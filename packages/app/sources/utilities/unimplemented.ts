// https://doc.rust-lang.org/std/macro.unimplemented.html
// https://github.com/LinusU/ts-unimplemented

export class UnimplementedError extends Error {}

export function unimplemented(): never {
	throw new UnimplementedError();
}
