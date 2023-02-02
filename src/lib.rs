#[macro_export]
#[doc(hidden)]
macro_rules! __internal_matrix_match {
    (
        ($first:expr, $sec:expr) ; $patSs:tt =>
        $( $patF:pat => $exs:tt;)+
    ) => {{
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

/// Some doc comment
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
                (En::A, true) ; true ,  false  =>
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
                En::B(a)   =>    a   ,  80   , a * b ;
                En::C      =>    5   ,  90   , 6     ;
            ),
            80
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
}
