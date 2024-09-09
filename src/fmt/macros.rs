#[macro_export]
macro_rules! format_args {
    () => {
        $crate::fmt::args::Arguments(&[])
    };

    ($(
        $(  $el_id:ident   )?
        $(  $el_li:literal )?
        $({ $el_ex:expr   })?

        $( as
            $style:ident
                $(:: $(@ $gens_colon:tt)?)?
                $(< $($gens:tt)* >)?
                $(:: $assoc:ident)*
                $(( $($tpl_args:tt)* ))?
                $({ $($sct_args:tt)* })?
        )?
    ),+ $(,)?) => {
        $crate::fmt::args::Arguments(&[$(
            $crate::fmt::args::Var::new(
                $(&$el_id,)?
                $(&$el_li,)?
                $(&$el_ex,)?

                $crate::_if_else!(
                    [$(
                        &$style
                            $(:: $($gens_colon)?)?
                            $(< $($gens)* >)?
                            $(:: $assoc)*
                            $(( $($tpl_args)* ))?
                            $({ $($sct_args)* })?
                    )?]
                    else
                    [&$crate::fmt::Display]
                )
            ),
        )*])
    };
}

#[macro_export]
macro_rules! format_args_nl {
    () => {
        $crate::fmt::args::Arguments(&[])
    };

    ($(
        $(  $el_id:ident   )?
        $(  $el_li:literal )?
        $({ $el_ex:expr   })?

        $( as
            $style:ident
                $(:: $(@ $gens_colon:tt)?)?
                $(< $($gens:tt)* >)?
                $(:: $assoc:ident)*
                $(( $($tpl_args:tt)* ))?
                $({ $($sct_args:tt)* })?
        )?
    ),+ $(,)?) => {
        $crate::fmt::args::Arguments(&[$(
            $crate::fmt::args::Var::new(
                $(&$el_id,)?
                $(&$el_li,)?
                $(&$el_ex,)?

                $crate::_if_else!(
                    [$(
                        &$style
                            $(:: $($gens_colon)?)?
                            $(< $($gens)* >)?
                            $(:: $assoc)*
                            $(( $($tpl_args)* ))?
                            $({ $($sct_args)* })?
                    )?]
                    else
                    [&$crate::fmt::Display]
                )
            ),
        )*
            $crate::fmt::args::Var::new(&'\n', &$crate::fmt::Display),
        ])
    };
}

#[macro_export]
macro_rules! write {
    ($dst:expr, $($t:tt)*) => {
        $dst.write_args(&$crate::format_args!($($t)*))
    };
}

#[macro_export]
macro_rules! writeln {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, '\n')
    };

    ($dst:expr, $($t:tt)*) => {
        $dst.write_args(&$crate::format_args_nl!($($t)*))
    };
}

#[cfg(any(feature = "std", test))]
mod with_std {
    #[macro_export]
    macro_rules! format {
        ($($t:tt)*) => {
            $crate::fmt::_format($crate::format_args!($($t)*))
        };
    }

    #[macro_export]
    macro_rules! print {
        ($($t:tt)*) => {
            $crate::fmt::_print($crate::format_args!($($t)*))
        };
    }

    #[macro_export]
    macro_rules! println {
        ($($t:tt)*) => {
            $crate::fmt::_print($crate::format_args_nl!($($t)*))
        };
    }

    #[macro_export]
    macro_rules! eprint {
        ($($t:tt)*) => {
            $crate::fmt::_eprint($crate::format_args!($($t)*))
        };
    }

    #[macro_export]
    macro_rules! eprintln {
        ($($t:tt)*) => {
            $crate::fmt::_eprint($crate::format_args_nl!($($t)*))
        };
    }

    #[macro_export]
    macro_rules! dbg {
        ($($x:expr),+ $(,)?) => {($(
            match $x {
                x => {
                    $crate::eprintln!(
                        "[",
                        { ::core::file!() },
                        ":",
                        { ::core::line!() },
                        ":",
                        { ::core::column!() },
                        "] ",
                        { ::core::stringify!($x) },
                        " = ",
                        x as $crate::fmt::Pretty(0)
                    );
                    x
                }
            }
        ),+)};
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _if_else {
    ( [] else [$($t:tt)*] ) => { $($t)* };
    ( [$($t:tt)*] else [$($_:tt)*] ) => { $($t)* };
}
