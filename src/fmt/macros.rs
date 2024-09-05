#[cfg(any(feature = "std", test))]
mod with_std {
    #[macro_export]
    macro_rules! format {
        ( $(
            $el:expr
            $( => $style:ident $(::$path:ident)*
                $(< $($gens:tt)* >)?
                $(( $($args:tt)* ))?
            )?
        ),* $(,)? ) => {{
            let mut s = ::std::string::String::new();
            $(
                $crate::fmt::Format::<
                    $crate::_if_else!(
                        [$( $style $(::$path)* $(< $($gens)* >)? )?]
                        else [$crate::fmt::Display]
                    )
                >::fmt(&$el, $crate::fmt::Formatter::new(
                    &mut s,
                    $crate::_if_else!([$($(( $($args)* ))?)?] else [&()])
                )).expect(concat!(
                    "failed to format argument `",
                    stringify!($el),
                    "`",
                    $(
                        " (with style `",
                        stringify!($style),
                        stringify!($(::$path)*),
                        "`)",
                    )?
                ));
            )*

            s
        }};
    }

    #[macro_export]
    macro_rules! print {
        ( $( $el:expr $( => $style:path )? ),* $(,)? ) => {{
            let mut s = ::std::io::stdout().lock();
            $(
                $crate::fmt::Format::<
                    $crate::_if_else!([$($style)?] else [$crate::fmt::Display])
                >::fmt(&$el, $crate::fmt::Style::style(&mut s)).expect(concat!(
                    "failed to format argument `",
                    stringify!($el),
                    "`",
                    $(
                        " (with style `",
                        stringify!($style),
                        "`)",
                    )?
                ));
            )*
        }};
    }

    #[macro_export]
    macro_rules! println {
        ( $( $el:expr $( => $style:path )? ),* $(,)? ) => {
            $crate::print!($( $el $( => $style )?, )* '\n')
        };
    }

    #[macro_export]
    macro_rules! eprint {
        ( $( $el:expr $( => $style:path )? ),* $(,)? ) => {{
            let mut s = ::std::io::stderr().lock();
            $(
                $crate::fmt::Format::<
                    $crate::_if_else!([$($style)?] else [$crate::fmt::Display])
                >::fmt(&$el, $crate::fmt::Style::style(&mut s)).expect(concat!(
                    "failed to format argument `",
                    stringify!($el),
                    "`",
                    $(
                        " (with style `",
                        stringify!($style),
                        "`)",
                    )?
                ));
            )*
        }};
    }

    #[macro_export]
    macro_rules! eprintln {
        ( $( $el:expr $( => $style:path )? ),* $(,)? ) => {
            $crate::eprint!($( $el $( => $style )?, )* '\n')
        };
    }
}

#[macro_export]
macro_rules! _if_else {
    ( [] else [$($t:tt)*] ) => { $($t)* };
    ( [$($t:tt)*] else [$($_:tt)*] ) => { $($t)* };
}
