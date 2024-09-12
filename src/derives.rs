// Beware all ye who enter here, for beyond lay a vast sea of dark creatures and
// uncharted crags.
//
// `_do_derive!` handles each case separately. The possibilities are as follows:
// - A unit struct (easy peasy)
// - A tuple struct (fairly straightforward)
// - A "proper" struct (quite straightforward)
// - An enum (see below)
//
// The struct cases are pretty straightforward, in fact the bulk of each case is
// simply handling the (optional) styling on each argument. Enums, however, are
// trickier, with these cases:
// - A unit variant (easy peasy)
// - A struct variant (quite straightforward)
// - A tuple variant (see below)
//
// The problem arises with tuple variants. You see, for tuple *structs*, we have
// access to `self`, so it's easy to let the user do something like this:
// ```
// derive!(struct Foo(0, 2, 4));
// ```
//
// Any fields provided get substituted into `self.$field`, and any that aren't
// are skipped. Easy, intuitive. Not so for tuple variants.
//
// Because, with enums variants, we don't get `self` (until variant types,
// anyways). So instead we have to force the user to give us an ident for each
// field (unpleasant) and use an `_` to explicitly skip fields - even though for
// struct variants, the user can implicitly ignore fields!
//
// Worst of all, to maintain consistency, tuple structs have to be downgraded to
// this inferior syntax.

#[macro_export]
macro_rules! derive {
    ( Debug for $($t:tt)* ) => {
        $crate::_do_derive!(Debug $($t)*);
    };

    ( Pretty for $($t:tt)* ) => {
        $crate::_do_derive!(Pretty $($t)*);
    };

    ( Debug + Pretty for $($t:tt)* ) => {
        $crate::_do_derive!(Debug $($t)*);
        $crate::_do_derive!(Pretty $($t)*);
    };

    ( Pretty + Debug for $($t:tt)* ) => {
        $crate::_do_derive!(Pretty $($t)*);
        $crate::_do_derive!(Debug $($t)*);
    };

    ( $($t:tt)* ) => {
        $crate::_do_derive!(Debug $($t)*);
        $crate::_do_derive!(Pretty $($t)*);
    };
}

#[macro_export]
macro_rules! _do_derive {
    ( $style_name:ident struct $name:ident $( :: $name_path:ident )*
        $(<
            $( $lt:lifetime ),* $(,)?
            $( $gen:ident $(! $(@ $add_fmt:tt)?)? ),* $(,)?
        >)?
        $(where [$( $where:tt )*] )?
    ) => {
        impl $(<
            $( $lt, )*
            $( $gen $( : $crate::fmt::Format<$crate::fmt::$style_name> $(@ $add_fmt)?)?, )*
        >)? $crate::fmt::Format<$crate::fmt::$style_name> for $name
            $( :: $name_path )*
            $(< $($lt,)* $($gen,)* >)?
        $( where $( $where )* )?
        {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                _: &$crate::fmt::$style_name,
            ) -> $crate::fmt::Result {
                $crate::fmt::Write::write_str(f, stringify!($name))
            }
        }
    };

    ( $style_name:ident struct $name:ident $( :: $name_path:ident )*
        $(<
            $( $lt:lifetime ),* $(,)?
            $( $gen:ident $(! $(@ $add_fmt:tt)?)? ),* $(,)?
        >)?
        $(where [$( $where:tt )*] )?
        ($(
            $( _ $(@ $skip:tt)? )?
            $( $field:ident
                $( as $style:expr )?
            )?
        ),* , $(... $(@ $non_exhaustive:tt)?)?)
    ) => {
        impl $(<
            $( $lt, )*
            $( $gen $( : $crate::fmt::Format<$crate::fmt::$style_name> $(@ $add_fmt)?)?, )*
        >)? $crate::fmt::Format<$crate::fmt::$style_name> for $name
            $( :: $name_path )*
            $(< $($lt,)* $($gen,)* >)?
        $( where $( $where )* )?
        {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::$style_name,
            ) -> $crate::fmt::Result {
                let Self($(
                    $( _ $(@ $skip)? )?
                    $( $field )?
                ),*) = self;

                let mut f = s.dbg_tuple(f, stringify!($name));

                $($( $crate::_if_else!(
                    [$( f.field_styled(
                        &$field,
                        &$style,
                    ) )?]
                    else [ f.field(&$field) ]
                ); )?)*

                $crate::_if_else!(
                    [$( f.non_exhaustive() $($non_exhaustive)? )?]
                    else [ f.finish() ]
                )
            }
        }
    };

    ( $style_name:ident struct $name:ident $( :: $name_path:ident )*
        $(<
            $( $lt:lifetime ),* $(,)?
            $( $gen:ident $(! $(@ $add_fmt:tt)?)? ),* $(,)?
        >)?
        $(where [$( $where:tt )*] )?
        ($(
            $( _ $(@ $skip:tt)? )?
            $( $field:ident
                $( as $style:expr )?
            )?
        ),*)
    ) => {
        impl $(<
            $( $lt, )*
            $( $gen $( : $crate::fmt::Format<$crate::fmt::$style_name> $(@ $add_fmt)?)?, )*
        >)? $crate::fmt::Format<$crate::fmt::$style_name> for $name
            $( :: $name_path )*
            $(< $($lt,)* $($gen,)* >)?
        $( where $( $where )* )?
        {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::$style_name,
            ) -> $crate::fmt::Result {
                let Self($(
                    $( _ $(@ $skip)? )?
                    $( $field )?
                ),*) = self;

                let mut f = s.dbg_tuple(f, stringify!($name));

                $($( $crate::_if_else!(
                    [$( f.field_styled(
                        &$field,
                        &$style,
                    ) )?]
                    else [ f.field(&$field) ]
                ); )?)*

                f.finish()
            }
        }
    };

    ( $style_name:ident struct $name:ident $( :: $name_path:ident )*
        $(<
            $( $lt:lifetime ),* $(,)?
            $( $gen:ident $(! $(@ $add_fmt:tt)?)? ),* $(,)?
        >)?
        $(where [$( $where:tt )*] )?
        {$(
            $field:ident
            $( as $style:expr )?
        ),* , $(... $(@ $non_exhaustive:tt)?)?}
    ) => {
        impl $(<
            $( $lt, )*
            $( $gen $( : $crate::fmt::Format<$crate::fmt::$style_name> $(@ $add_fmt)?)?, )*
        >)? $crate::fmt::Format<$crate::fmt::$style_name> for $name
            $( :: $name_path )*
            $(< $($lt,)* $($gen,)* >)?
        $( where $( $where )* )?
        {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::$style_name,
            ) -> $crate::fmt::Result {
                let mut f = s.dbg_struct(f, stringify!($name));
                $( $crate::_if_else!(
                    [$( f.field_styled(
                        stringify!($field),
                        &self.$field,
                        &$style,
                    ) )?]
                    else [ f.field(stringify!($field), &self.$field) ]
                );)*

                $crate::_if_else!(
                    [$( f.non_exhaustive() $(@ $non_exhaustive)? )?]
                    else [ f.finish() ]
                )
            }
        }
    };

    ( $style_name:ident struct $name:ident $( :: $name_path:ident )*
        $(<
            $( $lt:lifetime ),* $(,)?
            $( $gen:ident $(! $(@ $add_fmt:tt)?)? ),* $(,)?
        >)?
        $(where [$( $where:tt )*] )?
        {$(
            $field:ident
            $( as $style:expr )?
        ),*}
    ) => {
        impl $(<
            $( $lt, )*
            $( $gen $( : $crate::fmt::Format<$crate::fmt::$style_name> $(@ $add_fmt)?)?, )*
        >)? $crate::fmt::Format<$crate::fmt::$style_name> for $name
            $( :: $name_path )*
            $(< $($lt,)* $($gen,)* >)?
        $( where $( $where )* )?
        {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::$style_name,
            ) -> $crate::fmt::Result {
                let mut f = s.dbg_struct(f, stringify!($name));
                $( $crate::_if_else!(
                    [$( f.field_styled(
                        stringify!($field),
                        &self.$field,
                        &$style,
                    ) )?]
                    else [ f.field(stringify!($field), &self.$field) ]
                );)*

                f.finish()
            }
        }
    };

    ( $style_name:ident enum $name:ident $( :: $name_path:ident )*
        $(<
            $( $lt:lifetime ),* $(,)?
            $( $gen:ident $(! $(@ $add_fmt:tt)?)? ),* $(,)?
        >)?
        $(where [$( $where:tt )*] )?
        {$( $variant:ident
            $(( $(
                $( _ $(@ $skip:tt)? )?
                $( $tup_field:ident
                    $( as $tup_style:expr )?
                )?
            ),* $( , ... $(@ $tup_non_exhaustive:tt)? )? ))?

            $({ $(
                $sct_field:ident
                $( as $sct_style:expr )?
            ),* $( , $(... $(@ $sct_non_exhaustive:tt)?)? )? })?
        ),* $(,)?}
    ) => {
        impl $(<
            $( $lt, )*
            $( $gen $( : $crate::fmt::Format<$crate::fmt::$style_name> $(@ $add_fmt)?)?, )*
        >)? $crate::fmt::Format<$crate::fmt::$style_name> for $name
            $( :: $name_path )*
            $(< $($lt,)* $($gen,)* >)?
        $( where $( $where )* )?
        {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::$style_name,
            ) -> $crate::fmt::Result {
                match self {$(
                    Self::$variant
                        $(( $(
                            $( _ $(@ $skip)? )?
                            $( $tup_field )?
                        ),* ))?

                        $({ $(
                            $sct_field,
                        )* .. })?
                    => {
                        $crate::_if_else!([
                            $({
                                let mut f = s.dbg_tuple(f, stringify!($variant));

                                $($( $crate::_if_else!(
                                    [$( f.field_styled(
                                        &$tup_field,
                                        &$tup_style,
                                    ) )?]
                                    else [ f.field(&$tup_field) ]
                                ); )?)*

                                $crate::_if_else!(
                                    [$($( f.non_exhaustive() $(@ $tup_non_exhaustive)? )?)?]
                                    else [ f.finish() ]
                                )
                            })?

                            $({
                                let mut f = s.dbg_struct(f, stringify!($variant));

                                $( $crate::_if_else!(
                                    [$( f.field_styled(
                                        stringify!($sct_field),
                                        &$sct_field,
                                        &$sct_style,
                                    ) )?]
                                    else [ f.field(stringify!($sct_field), &$sct_field) ]
                                ); )*

                                $crate::_if_else!(
                                    [$($( f.non_exhaustive() $(@ $sct_non_exhaustive)? )?)?]
                                    else [ f.finish() ]
                                )
                            })?
                        ] else [
                            $crate::fmt::Write::write_str(f, stringify!($variant))
                        ])
                    }
                )*}
            }
        }
    };
}
