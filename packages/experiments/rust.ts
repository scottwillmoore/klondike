/*
enum A {
    A = 0,
    B = 1,
    C = 2,
}
*/

enum ADiscriminant {
	A = 0,
	B = 1,
	C = 2,
}

// type A = {
// 	discriminant: ADiscriminant;
// };

type A = ADiscriminant;

let a: A;
a = ADiscriminant.A;
a = ADiscriminant.B;
a = ADiscriminant.C;

/*
enum B {
    A,
    B(u32),
    C(u32, u32),
}
*/

enum BDiscriminant {
	A = 0,
	B = 1,
	C = 2,
}

type BA = {
	discriminant: BDiscriminant.A;
};

type BB = {
	discriminant: BDiscriminant.B;
	0: number;
};

// type BB = {
// 	discriminant: BDiscriminant.B;
// } & [number];

type BC = {
	discriminant: BDiscriminant.C;
	0: number;
	1: number;
};

// type BC = {
// 	discriminant: BDiscriminant.C;
// } & [number, number];

type B = BA | BB | BC;

let b: B;
b = { discriminant: BDiscriminant.A };
b = { discriminant: BDiscriminant.B, 0: 0 };
b = { discriminant: BDiscriminant.C, 0: 0, 1: 0 };

/*
enum C {
    A,
    B {
        a: u32,
    },
    C {
        a: u32,
        b: u32,
    },
}
*/

enum CDiscriminant {
	A = 0,
	B = 1,
	C = 2,
}

type CA = {
	discriminant: CDiscriminant.A;
};

type CB = {
	discriminant: CDiscriminant.B;
	a: number;
};

type CC = {
	discriminant: CDiscriminant.C;
	a: number;
	b: number;
};

type C = CA | CB | CC;

let c: C;
c = { discriminant: CDiscriminant.A };
c = { discriminant: CDiscriminant.B, a: 0 };
c = { discriminant: CDiscriminant.C, a: 0, b: 0 };

/*
struct D {
    a: u32,
    b: u32,
}
*/

type D = {
	a: number;
	b: number;
};

const d: D = { a: 0, b: 0 };

// There is room for improvement in some of these examples.
// - `ADiscriminant` can just become `A`.
// - `discriminant` should be a `unique symbol`.
// - Tuples might be better encoded as `{} & []`.
