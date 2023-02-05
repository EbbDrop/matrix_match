//! This crate provides a macro to easily match on two values at the same time.
//!
//! The [`matrix_match`] macro transforms a matrix of possible results into `match`
//! statements rust understands. See the its documentation for examples and specifics.

#![deny(missing_docs)]
#![deny(unsafe_code)]

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_matrix_match {
    (
        ($first:expr, $sec:expr) ; $patSs:tt =>
        $( $patF:pat => $exs:tt;)+
    ) => {{
        // Extracting to a variable in case one of the patterns shadows the name.
        let sec = $sec;
        match $first {
            $( $patF => {
                $crate::__internal_matrix_match!(@secmatch (sec); $patSs; $exs)
            } )*
        }
    }};

    (@secmatch
        ($val:expr);
        ($($pat:pat),+);
        ($($ex:expr),+)
    ) => {{
        #[allow(unused_variables)]
        match $val {
            $(
            $pat => {$ex}
            )*
        }
    }};
}

/// Macro to match on a pair of values.
///
///
/// # Usage
/// ```rust
/// # use matrix_match::matrix_match;
/// # #[allow(non_camel_case_types)]
/// # enum A { a, b}
/// # #[allow(non_camel_case_types)]
/// # enum B { a, b}
/// # let a = A::a;
/// # let b = B::b;
/// matrix_match!(
///     (a, b)    ; B::a , B::b =>
///     A::a     => "aa" , "ab"  ;
///     A::b     => "ba" , "bb"  )
/// # ;
/// ```
/// First matches the first value to the patterns on the left. Then the second value gets matched
/// to the patterns at the top. The expression at the intersection of both matches is what is
/// eventually ran and possibly returned.
///
/// Destructuring is also possible inside a `matrix_match`:
/// ```rust
/// # use matrix_match::matrix_match;
/// # let a = Some(());
/// # let b = Ok(());
/// matrix_match!(
///     (a, b)    ; Ok(v) , Err(e) =>
///     Some(s)  => s     , s       ;
///     None     => v     , e       )
/// # ;
/// ```
/// The same name can be used in separate columns or separate rows.
/// A variable in a column will shadow any variable in a row with the same name!
///
/// # Full Example
///
/// ```rust
/// # use matrix_match::matrix_match;
/// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// enum Light {
///     Red,
///     Orange,
///     Green,
/// }
///
/// fn next(light: Light, car_waiting: bool) -> Light {
///     use Light::*;
///     matrix_match!(
///         (light, car_waiting) ; true  , false =>
///         Red                 => Green , Red    ;
///         Orange              => Red   , Red    ;
///         Green               => Green , Orange ;
///     )
/// }
///
/// # fn main() {
/// assert_eq!(next(Light::Red, true ), Light::Green);
/// assert_eq!(next(Light::Red, false), Light::Red);
///
/// assert_eq!(next(Light::Orange, true ), Light::Red);
/// assert_eq!(next(Light::Orange, false), Light::Red);
///
/// assert_eq!(next(Light::Green, true ), Light::Green);
/// assert_eq!(next(Light::Green, false), Light::Orange);
/// # }
///
/// ```
#[macro_export]
macro_rules! matrix_match {
    (
        ($first:expr, $sec:expr) ; $($patS:pat),+    =>
        $( $patF:pat            => $($ex:expr),+);+ $(;)?
    ) => {{
        $crate::__internal_matrix_match!(($first, $sec) ; ($($patS),*) =>
        $( $patF => ($($ex),*);)*)
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn basic_use() {
        #[allow(dead_code)]
        enum En {
            A,
            B,
            C,
        }

        assert_eq!(
            matrix_match!(
                (En::B, false) ; true ,  false  =>
                En::A         => 1    ,   2     ;
                En::B         => 3    ,   4     ;
                En::C         => 5    ,   6     ;
            ),
            4
        );

        assert_eq!(
            matrix_match!(
                (En::A, true)  ; true ,  false =>
                En::A         => 1    ,   2     ;
                En::B         => 3    ,   4     ;
                En::C         => 5    ,   6
            ),
            1
        );
    }

    #[test]
    fn complex_match() {
        #[allow(dead_code)]
        enum En {
            A,
            B(u32),
            C,
        }

        assert_eq!(
            matrix_match!(
                (En::B(2), 5) ; 0..=3,  b  =>
                En::A      =>    1   ,  2     ;
                En::B(a)   =>    a   ,  a * b ;
                En::C      =>    5   ,  6     ;
            ),
            2 * 5
        );

        assert_eq!(
            matrix_match!(
                (En::A, 2)    ; 0..=3,  b  =>
                En::A      =>    1   ,  2     ;
                En::B(a)   =>    a   ,  a * b ;
                En::C      =>    5   ,  6     ;
            ),
            1
        );

        assert_eq!(
            matrix_match!(
                (En::B(5), 2) ; 0..=3, b  =>
                En::A      =>    1   , 2     ;
                En::B(a)   =>    a   , a * b ;
                En::C      =>    5   , 6     ;
            ),
            5
        );

        assert_eq!(
            matrix_match!(
                (En::B(5), 4) ; 0..=3, b @ 4 , b  =>
                En::A      =>    1   ,  40   , 2     ;
                En::B(a)   =>    a   ,  b   , a * b ;
                En::C      =>    5   ,  90   , 6     ;
            ),
            4
        );
    }

    #[test]
    fn complex_expr() {
        assert_eq!(
            matrix_match!(
                (Some(true), 5) ; b =>
                None          =>  { let a = 0; a}  ;
                Some(f)       =>  if f { b } else {b * 2}  ;
            ),
            5
        );

        assert_eq!(
            matrix_match!(
                (Some(false), 5) ;         b               =>
                None          =>  { let a = 0; a}          ;
                Some(f)       =>  if f { b } else {b * 2}  ;
            ),
            5 * 2
        );
    }

    #[test]
    fn big() {
        for a in 0..=8 {
            for b in 0..=9 {
                assert_eq!(
                    matrix_match!(
                        (a, b) ;  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, _ =>
                        0     =>   0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0  ;
                        1     =>   0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 0  ;
                        2     =>   0,  2,  4,  6,  8, 10, 12, 14, 16, 18, 0  ;
                        3     =>   0,  3,  6,  9, 12, 15, 18, 21, 24, 27, 0  ;
                        4     =>   0,  4,  8, 12, 16, 20, 24, 28, 32, 36, 0  ;
                        5     =>   0,  5, 10, 15, 20, 25, 30, 35, 40, 45, 0  ;
                        6     =>   0,  6, 12, 18, 24, 30, 36, 42, 48, 54, 0  ;
                        7     =>   0,  7, 14, 21, 28, 35, 42, 49, 56, 63, 0  ;
                        8     =>   0,  8, 16, 24, 32, 40, 48, 56, 64, 72, 0  ;
                        _     =>   0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0  ;
                    ),
                    a * b
                );
            }
        }
    }

    #[test]
    fn full() {
        #[allow(dead_code)]
        pub enum Eb {
            A(bool),
            C,
        }

        #[allow(dead_code)]
        pub enum Ea {
            A(bool, bool),
            B(u32),
        }

        let test = |(a, b, res): (Ea, Eb, &str)| {
            assert_eq!(
                matrix_match!(
                    (a, b)           ; Eb::A(true)        , Eb::A(false)       , Eb::C           =>
                    Ea::A(true,  b) => "aa".to_string()   , b.to_string()      , (!b).to_string() ;
                    Ea::A(false, b) => "afat".to_string() , "afaf".to_string() , (b as u8 + 4).to_string()  ;
                    Ea::B(i)        => (i+4).to_string()  , (i*2).to_string()  , match i { 3 => "abcd".to_string(), _ => "cdef".to_string() }  ;
                ),
                res.to_string()
            );
        };

        [
            (Ea::A(true, true), Eb::A(true), "aa"),
            (Ea::A(true, true), Eb::A(false), "true"),
            (Ea::A(true, true), Eb::C, "false"),
            (Ea::A(true, false), Eb::A(true), "aa"),
            (Ea::A(true, false), Eb::A(false), "false"),
            (Ea::A(true, false), Eb::C, "true"),
            (Ea::A(false, true), Eb::A(true), "afat"),
            (Ea::A(false, true), Eb::A(false), "afaf"),
            (Ea::A(false, true), Eb::C, "5"),
            (Ea::A(false, false), Eb::A(true), "afat"),
            (Ea::A(false, false), Eb::A(false), "afaf"),
            (Ea::A(false, false), Eb::C, "4"),
            (Ea::B(2), Eb::A(true), "6"),
            (Ea::B(2), Eb::A(false), "4"),
            (Ea::B(2), Eb::C, "cdef"),
            (Ea::B(3), Eb::A(true), "7"),
            (Ea::B(3), Eb::A(false), "6"),
            (Ea::B(3), Eb::C, "abcd"),
        ]
        .into_iter()
        .for_each(test)
    }
}
