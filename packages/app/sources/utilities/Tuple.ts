// prettier-ignore
type Tuple_<T, Length extends number, Tuple extends unknown[] = []> =
    Tuple["length"] extends Length
        ? Tuple
        : Tuple_<T, Length, [...Tuple, T]>;

// prettier-ignore
export type Tuple<T, Length extends number> =
    Length extends unknown
        ? number extends Length
            ? never
            : Tuple_<T, Length>
        : never;
