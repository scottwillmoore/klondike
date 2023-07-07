export enum Color {
	Black = 0,
	Red = 1,
}

export namespace Color {
	export function toCharacter(color: Color): string {
		switch (color) {
			case Color.Black:
				return "B";
			case Color.Red:
				return "R";
		}
	}

	// rome-ignore lint/suspicious/noShadowRestrictedNames: Not a problem in a namespace.
	export function toString(color: Color): string {
		switch (color) {
			case Color.Black:
				return "Black";
			case Color.Red:
				return "Red";
		}
	}
}
