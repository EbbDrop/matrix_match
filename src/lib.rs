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
                        (a, b) ;   0,  1,  2,  3,  4,  5,  6,  7,  8,  9, _ =>
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
