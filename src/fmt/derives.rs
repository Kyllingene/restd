#[macro_export]
macro_rules! derive {
    ( Debug for $($t:tt)* ) => {
        $crate::debug!($($t)*);
    };

    ( Pretty for $($t:tt)* ) => {
        $crate::pretty!($($t)*);
    };

    ( Debug + Pretty for $($t:tt)* ) => {
        $crate::debug!($($t)*);
        $crate::pretty!($($t)*);
    };

    ( Pretty + Debug for $($t:tt)* ) => {
        $crate::pretty!($($t)*);
        $crate::debug!($($t)*);
    };

    ( $($t:tt)* ) => {
        $crate::debug!($($t)*);
        $crate::pretty!($($t)*);
    };
}

#[macro_export]
macro_rules! debug {
    ( struct $name:ident $(< $($gens:tt)* >)?; ) => {
        impl $crate::fmt::Format<$crate::fmt::Debug> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                _: &$crate::fmt::Debug,
            ) -> $crate::fmt::Result {
                $crate::fmt::Write::write_str(f, stringify!($name))
            }
        }
    };

    (
        struct $name:ident $(< $($gens:tt)* >)? ($(
            $( _ $(@ $skip:tt)? )?
            $( $field:ident
                $( as
                    $style:ident
                        $($(:: $(@ $sty_gens_colon:tt)?)? < $($sty_gens:tt)* >)?
                        $(:: $assoc:ident)*
                        $(( $($tpl_args:tt)* ))?
                        $({ $($sct_args:tt)* })?
                )?
            )?
        ),* $(,)? $(... $(@$non_exhaustive:tt)?)?)
    ) => {
        impl $crate::fmt::Format<$crate::fmt::Debug> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::Debug,
            ) -> $crate::fmt::Result {
                let Self($(
                    $( _ $(@ $skip)? )?
                    $( $field )?
                ),*) = self;

                let mut f = s.dbg_tuple(f, stringify!($name));

                $($( $crate::_if_else!(
                    [$( f.field_styled(
                        &$field,
                        &$style
                            $($(:: $(@ $sty_gens_colon)?)? < $($sty_gens)* > )?
                            $(:: $assoc)?
                            $(( $($tpl_args)* ))?
                            $({ $($sct_args)* })?
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

    (
        struct $name:ident $(< $($gens:tt)* >)? {$(
            $field:ident
            $( as
                $style:ident
                    $($(:: $(@ $sty_gens_colon:tt)?)? < $($sty_gens:tt)* >)?
                    $(:: $assoc:ident)*
                    $(( $($tpl_args:tt)* ))?
                    $({ $($sct_args:tt)* })?
            )?
        ),* $(,)? $(... $(@ $non_exhaustive:tt)?)?}
    ) => {
        impl $crate::fmt::Format<$crate::fmt::Debug> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::Debug,
            ) -> $crate::fmt::Result {
                let mut f = s.dbg_struct(f, stringify!($name));
                $( $crate::_if_else!(
                    [$( f.field_styled(
                        stringify!($field),
                        &self.$field,
                        &$style
                            $($(:: $(@ $sty_gens_colon)?)? < $($sty_gens)* >)?
                            $(:: $assoc)*
                            $(( $($tpl_args)* ))?
                            $({ $($sct_args)* })?
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

    ( enum $name:ident $(< $($gens:tt)* >)? {$(
        $variant:ident
            $(( $(
                $( _ $(@ $skip:tt)?)?
                $( $tup_field:ident
                    $( as
                        $tup_style:ident
                            $($(:: $(@ $tup_gens_colon:tt)?)? < $($tup_gens:tt)* >)?
                            $(:: $tup_assoc:ident)*
                            $(( $($tup_tpl_args:tt)* ))?
                            $({ $($tup_sct_args:tt)* })?
                    )?
                )?
            ),* $(,)? $(... $(@ $tup_non_exhaustive:tt)?)? ))?

            $({ $(
                $sct_field:ident
                $( as
                    $sct_style:ident
                        $($(:: $(@ $sct_gens_colon:tt)?)? < $($sct_gens:tt)* >)?
                        $(:: $sct_assoc:ident)*
                        $(( $($sct_tpl_args:tt)* ))?
                        $({ $($sct_sct_args:tt)* })?
                )?
            ),* $(,)? $(... $(@ $sct_non_exhaustive:tt)?)? })?
    ),* $(,)?} ) => {
        impl $crate::fmt::Format<$crate::fmt::Debug> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::Debug,
            ) -> $crate::fmt::Result {
                match self {$(
                    Self::$variant
                        $(($( $($tup_field)? $(_ $(@ $skip)?)? ),*))?
                        $({$( $sct_field, )* ..})?
                    => {
                        $crate::_if_else!([
                            $({
                                let mut f = s.dbg_tuple(f, stringify!($variant));
                                $($( $crate::_if_else!(
                                    [$(
                                        f.field_styled(
                                            &$tup_field,
                                            &$tup_style
                                                $($(:: $($tup_gens_colon)?)? < $($tup_gens)* >)?
                                                $(:: $tup_assoc)?
                                                $(( $($tup_tpl_args)* ))?
                                                $({ $($tup_sct_args)* })?
                                        )
                                    )?]
                                    else [f.field(&$tup_field)]
                                ); )?)*

                                $crate::_if_else!(
                                    [$( f.non_exhaustive() $(@ $tup_non_exhaustive)? )?]
                                    else [ f.finish() ]
                                )
                            })?

                            $({
                                let mut f = s.dbg_struct(f, stringify!($variant));
                                $($crate::_if_else!(
                                    [$(
                                        f.field_styled(
                                            stringify!($sct_field),
                                            &$sct_field,
                                            &$sct_style
                                                $($(:: $($sct_gens_colon)?)? < $($sct_gens)* >)?
                                                $(:: $sct_assoc)?
                                                $(( $($sct_tpl_args)* ))?
                                                $({ $($sct_sct_args)* })?
                                        )
                                    )?]
                                    else [f.field(stringify!($sct_field), &$sct_field)]
                                );)*

                                $crate::_if_else!(
                                    [$( f.non_exhaustive() $(@ $sct_non_exhaustive)? )?]
                                    else [ f.finish() ]
                                )
                            })?
                        ] else [
                            $crate::fmt::Write::write_str(f, stringify!($variant))
                        ])
                    },
                )*}
            }
        }
    };
}

#[macro_export]
macro_rules! pretty {
    ( struct $name:ident $(< $($gens:tt)* >)?; ) => {
        impl $crate::fmt::Format<$crate::fmt::Pretty> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                _: &$crate::fmt::Pretty,
            ) -> $crate::fmt::Result {
                $crate::fmt::Write::write_str(f, stringify!($name))
            }
        }
    };

    (
        struct $name:ident $(< $($gens:tt)* >)? ($(
            $( _ $(@ $skip:tt)? )?
            $( $field:ident
                $( as
                    $style:ident
                        $($(:: $(@ $sty_gens_colon:tt)?)? < $($sty_gens:tt)* >)?
                        $(:: $assoc:ident)*
                        $(( $($tpl_args:tt)* ))?
                        $({ $($sct_args:tt)* })?
                )?
            )?
        ),* $(,)? $(... $(@$non_exhaustive:tt)?)?)
    ) => {
        impl $crate::fmt::Format<$crate::fmt::Pretty> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::Pretty,
            ) -> $crate::fmt::Result {
                let Self($(
                    $( _ $(@ $skip)? )?
                    $( $field )?
                ),*) = self;

                let mut f = s.dbg_tuple(f, stringify!($name));

                $($( $crate::_if_else!(
                    [$( f.field_styled(
                        &$field,
                        &$style
                            $($(:: $(@ $sty_gens_colon)?)? < $($sty_gens)* > )?
                            $(:: $assoc)?
                            $(( $($tpl_args)* ))?
                            $({ $($sct_args)* })?
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

    (
        struct $name:ident $(< $($gens:tt)* >)? {$(
            $field:ident
            $( as
                $style:ident
                    $($(:: $(@ $sty_gens_colon:tt)?)? < $($sty_gens:tt)* >)?
                    $(:: $assoc:ident)*
                    $(( $($tpl_args:tt)* ))?
                    $({ $($sct_args:tt)* })?
            )?
        ),* $(,)? $(... $(@ $non_exhaustive:tt)?)?}
    ) => {
        impl $crate::fmt::Format<$crate::fmt::Pretty> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::Pretty,
            ) -> $crate::fmt::Result {
                let mut f = s.dbg_struct(f, stringify!($name));
                $( $crate::_if_else!(
                    [$( f.field_styled(
                        stringify!($field),
                        &self.$field,
                        &$style
                            $($(:: $(@ $sty_gens_colon)?)? < $($sty_gens)* >)?
                            $(:: $assoc)*
                            $(( $($tpl_args)* ))?
                            $({ $($sct_args)* })?
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

    ( enum $name:ident $(< $($gens:tt)* >)? {$(
        $variant:ident
            $(( $(
                $( _ $(@ $skip:tt)?)?
                $( $tup_field:ident
                    $( as
                        $tup_style:ident
                            $($(:: $(@ $tup_gens_colon:tt)?)? < $($tup_gens:tt)* >)?
                            $(:: $tup_assoc:ident)*
                            $(( $($tup_tpl_args:tt)* ))?
                            $({ $($tup_sct_args:tt)* })?
                    )?
                )?
            ),* $(,)? $(... $(@ $tup_non_exhaustive:tt)?)? ))?

            $({ $(
                $sct_field:ident
                $( as
                    $sct_style:ident
                        $($(:: $(@ $sct_gens_colon:tt)?)? < $($sct_gens:tt)* >)?
                        $(:: $sct_assoc:ident)*
                        $(( $($sct_tpl_args:tt)* ))?
                        $({ $($sct_sct_args:tt)* })?
                )?
            ),* $(,)? $(... $(@ $sct_non_exhaustive:tt)?)? })?
    ),* $(,)?} ) => {
        impl $crate::fmt::Format<$crate::fmt::Pretty> for $name $(< $($gens)* >)? {
            fn fmt(
                &self,
                f: &mut dyn $crate::fmt::Write,
                s: &$crate::fmt::Pretty,
            ) -> $crate::fmt::Result {
                match self {$(
                    Self::$variant
                        $(($( $($tup_field)? $(_ $(@ $skip)?)? ),*))?
                        $({$( $sct_field, )* ..})?
                    => {
                        $crate::_if_else!([
                            $({
                                let mut f = s.dbg_tuple(f, stringify!($variant));
                                $($( $crate::_if_else!(
                                    [$(
                                        f.field_styled(
                                            &$tup_field,
                                            &$tup_style
                                                $($(:: $($tup_gens_colon)?)? < $($tup_gens)* >)?
                                                $(:: $tup_assoc)?
                                                $(( $($tup_tpl_args)* ))?
                                                $({ $($tup_sct_args)* })?
                                        )
                                    )?]
                                    else [f.field(&$tup_field)]
                                ); )?)*

                                $crate::_if_else!(
                                    [$( f.non_exhaustive() $(@ $tup_non_exhaustive)? )?]
                                    else [ f.finish() ]
                                )
                            })?

                            $({
                                let mut f = s.dbg_struct(f, stringify!($variant));
                                $($crate::_if_else!(
                                    [$(
                                        f.field_styled(
                                            stringify!($sct_field),
                                            &$sct_field,
                                            &$sct_style
                                                $($(:: $($sct_gens_colon)?)? < $($sct_gens)* >)?
                                                $(:: $sct_assoc)?
                                                $(( $($sct_tpl_args)* ))?
                                                $({ $($sct_sct_args)* })?
                                        )
                                    )?]
                                    else [f.field(stringify!($sct_field), &$sct_field)]
                                );)*

                                $crate::_if_else!(
                                    [$( f.non_exhaustive() $(@ $sct_non_exhaustive)? )?]
                                    else [ f.finish() ]
                                )
                            })?
                        ] else [
                            $crate::fmt::Write::write_str(f, stringify!($variant))
                        ])
                    },
                )*}
            }
        }
    };
}
