declare const type: unique symbol;

export type NewType<T, TType> = T & { [type]: TType };
