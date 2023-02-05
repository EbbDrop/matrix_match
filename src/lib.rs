// Generates a single match statement using tt-munching and Push-down Accumulation. These techniques are needed because of
// limitations of the macro system:
// - All repetitions in a loop need to have the same length
// - Macros need to output valid rust code
// We run in to the first limitation because the matrix can have a different number of rows and columns.
// This can be overcome by calling a inner macro for every outer repetitions with just a singe token tree
// with the data of the inner repetitions, that macro can then do it's own repetition of a different length.
// Sadly this can't be used here since the inner macro would need to produce match arms witch are not
// valid rust code. The solution here is to use tt-munching and store the arms as input to the next macro,
// since macro input does not need to be valid rust code.
//
// Input:
// All the inputs except the first have trailing commas, this seems to help with matching tt
//
// - match arguments `(expr, expr)`
// - a "backup" of the column patterns `(pat, pat, ...,)`
// - a list if lists of results reaming to be processed `((expr, expr, ...,), (expr, expr, ...,), ...,)`
// - the row patterns `(pat, pat, ...,)`
// - the column patterns currently being processed `(pat, pat, ...,)`
// - the results currently being processed `(expr, expr, ...,)`
// - the arms produced so far
#[macro_export]
#[doc(hidden)]
macro_rules! __internal_matrix_match_single {
    // Creates a new arms by taking the first patterns and the first expr. Only the column paterns
    // and the expression gets removed.
    (
        $args:tt;
        $culumn_pats_backup:tt;
        $remaining_expresions:tt;
        ($first_row_pat:tt, $($rest_row_pats:tt,)*);
        ($first_col_pat:tt, $($rest_col_pats:tt,)*);
        ($first_currrent_expr:tt, $($rest_current_exprs:tt,)*);
        $($arms:tt)*
    ) => {
            $crate::__internal_matrix_match_single!(
                $args;
                $culumn_pats_backup;
                $remaining_expresions;
                ($first_row_pat, $($rest_row_pats,)*);
                ($($rest_col_pats,)*);
                ($($rest_current_exprs,)*);
                $($arms)*
                ($first_row_pat, $first_col_pat) => { $first_currrent_expr }
            )
    };

    // Ran when the current column patterns and the current expressions run out. This macro takes
    // the cull column patterns "backup" and adds copies it back to the current patterns, while also
    // taking the first list of results for the remaining results and makes it current.
    (
        $args:tt;
        ($($culumn_pats_backup:tt,)*);
        ($first_remaining_expr:tt, $($rest_remaining_expr:tt,)*);
        ($first_row_pat:tt, $($rest_row_pats:tt,)*);
        ();
        ();
        $($arms:tt)*
    ) => {
            $crate::__internal_matrix_match_single!(
                $args;
                ($($culumn_pats_backup,)*);
                ($($rest_remaining_expr,)*);
                ($($rest_row_pats,)*);
                ($($culumn_pats_backup,)*);
                $first_remaining_expr;
                $($arms)*
            )
    };

    // Ran when the current column patterns, the current expressions AND the remaining expressions run
    // out. Takes the match arms produced and puts them in a match expression.
    (
        $args:tt;
        $_culumn_pats_backup:tt;
        ();
        ($_last_row_pat:tt,);
        ();
        ();
        $($arms:tt)*
    ) => {
        #[allow(unused_variables)]
        match $args {
            $($arms)*
        }
    };

    (
        $_args:tt; $_culumn_pats_backup:tt; $_rest_remaining_expr:tt;
        ($first_row_pat:tt, $($_rest_row_pats:tt,)*);
        ();
        ($_first_currrent_expr:tt, $($_rest_current_exprs:tt,)*);
        $($_arms:tt)*
    ) => {
        $crate::compile_error!(concat!("The row with the patern `", stringify!($first_row_pat), "` has to many results!"))
    };

    (
        $_args:tt; $_culumn_pats_backup:tt; $_rest_remaining_expr:tt;
        ($first_row_pat:tt, $($_rest_row_pats:tt,)*);
        ($_first_col_pat:tt, $($_rest_col_pats:tt,)*);
        ();
        $($_arms:tt)*
    ) => {
        $crate::compile_error!(concat!("The row with the patern `", stringify!($first_row_pat), "` has to little results!"))
    };
}

/// Some doc comment
#[macro_export]
macro_rules! matrix_match_single {
    (
        ($first:expr, $sec:expr) ; $($patS:pat),+    =>
        $( $patF:pat            => $($ex:expr),+);+ $(;)?
    ) => {{
            $crate::__internal_matrix_match_single!(
                ($first, $sec);
                ($($patS,)*);
                ($(($($ex,)*),)*);
                (_dummy, $($patF,)*);
                ();
                ();
            )
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_matrix_match_double {
    (
        ($first:expr, $sec:expr) ; $patSs:tt =>
        $( $patF:pat => $exs:tt;)+
    ) => {{
        let sec = $sec;
        match $first {
            $( $patF => {
                $crate::__internal_matrix_match_double!(@secmatch (sec); $patSs; $exs)
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

/// Some doc comment
#[macro_export]
macro_rules! matrix_match_double {
    (
        ($first:expr, $sec:expr) ; $($patS:pat),+    =>
        $( $patF:pat            => $($ex:expr),+);+ $(;)?
    ) => {{
        $crate::__internal_matrix_match_double!(($first, $sec) ; ($($patS),*) =>
        $( $patF => ($($ex),*);)*)
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! matrix_match {
    (
        ($first:expr, $sec:expr) ; $($patS:pat),+    =>
        $( $patF:pat            => $($ex:expr),+);+ $(;)?
    ) => {{
        $crate::matrix_match_single!(
            ($first, $sec) ; $($patS),+    =>
            $( $patF      => $($ex),+);+;
        )
    }};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! matrix_match {
    (
        ($first:expr, $sec:expr) ; $($patS:pat),+    =>
        $( $patF:pat            => $($ex:expr),+);+ $(;)?
    ) => {{
        $crate::matrix_match_double!(
            ($first, $sec) ; $($patS),+    =>
            $( $patF      => $($ex),+);+;
        )
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

        #[allow(dead_code)]
        pub enum Ea {
            A(bool),
            C,
        }

        #[allow(dead_code)]
        pub enum Eb {
            A(bool, bool),
            B(u32),
        }

        assert_eq!(
            matrix_match_double!(
                (Eb::B(4), Ea::A(false)) ; Ea::A(true)        , Ea::A(false)       , Ea::C           =>
                Eb::A(true,  b)         => "aa".to_string()   , b.to_string()      , (!b).to_string() ;
                Eb::A(false, b)         => "afat".to_string() , "afaf".to_string() , (b as u8 + 4).to_string()  ;
                Eb::B(i)                => (i+4).to_string()  , (i*2).to_string()  , match i { 3 => "abcd".to_string(), _ => "cdef".to_string() }  ;
            ),
            "8".to_string()
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
                        (a, b) ; 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, _ =>
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
}
