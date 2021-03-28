pub use paste as _paste;

/// This macro is intended to be used with the log crate. This macro
/// generates macros that applies some specific target to the common
/// logging macros (`info!`, `error!`...). The idea behind
/// this is preventing to explicitly specifying the target: parameter
/// on each call to these functions, by letting a macro do that on
/// your behalf. For example, if we want to add the target "potato" to
/// all the calls of a specific module we could do:
///
/// ```edition2018
/// use targeted_log::targeted_log;
///
/// targeted_log!("potato");
/// fn do_some_work() {
///     tginfo!("Hey! I'm doing some work");
///     tgwarn!("Warning! This is a warning!");
/// }
/// ```
///
/// This code will be equivalent to:
///
/// ```edition2018
/// use log::{info, warn};
///
/// fn do_some_work() {
///     info!(target: "potato", "Hey! I'm doing some work");
///     warn!(target: "potato", "Warning! This is a warning!");
/// }
/// ```
///
/// You can also add a format specifier to the target itself, to make it a bit more flexible:
/// ```edition2018
/// use targeted_log::targeted_log;
///
/// targeted_log!("Log {} {}");
/// fn do_some_work() {
///     tginfo!(@ 1, 2; "Hey! I'm doing some work");
///     tgwarn!(@ 3, 4; "Warning! This is a warning!");
/// }
/// ```
///
/// Is equivalent to doing:
///
/// ```edition2018
/// use log::{info, warn};
///
/// fn do_some_work() {
///     info!(target: &format!("Log {} {}", 1, 2), "Hey! I'm doing some work");
///     warn!(target: &format!("Log {} {}", 3, 4), "Warning! This is a warning!");
/// }
/// ```
///
/// Of course, calling this macro more than once per module will cause
/// a conflict between names.  For that cases, when we want to use
/// multiple logging targets within one module, we can specify the
/// prefix for the macros we're going to define. For example, this
/// calls:
///
/// ```edition2018
/// use targeted_log::targeted_log;
/// targeted_log!("app", app_);
/// targeted_log!("server", srv_);
/// ```
///
/// Will generate macros like `app_debug, app_info, srv_warn, srv_info`...
#[macro_export]
macro_rules! targeted_log {
    // Internal branches for applying format to the target.  If no
    // argument is provided, the first branch is selected and the
    // format specifier is not processed at all.  Otherwise, it falls
    // into the second branch, which effectively calls the format!
    // macro.
    (@fmt_tgt $tgt:expr) => {
	$tgt
    };

    (@fmt_tgt $tgt:expr, $($arg_target:expr),+) => {
	&format!($tgt, $($arg_target),+)
    };

    // This branch is for internal use only.  Generates a macro with
    // name `$implname` that calls the macro `$fun` with the target
    // `$tgt`. The $d parameter is a workaround for nesting macros,
    // and define repetitions in binding patterns correctly. It MUST
    // be set to $. See
    // https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
    (@internal $tgt:expr, $fun:ident, $implname:ident, $d: tt) => {
	#[allow(unused)]
	macro_rules! $implname {
	    ($d(@ $d($d arg_target:expr),+;)? $d($d arg:expr),+) => {
		log::$fun!(target: $crate::targeted_log!(@fmt_tgt $tgt $d(, $d($d arg_target),+)?), $d($d arg),+);
	    };
	}
    };

    // This branch will generate the macros for the given logging functions
    // `$impl` prefixed with `$prefix` and the specified target `$tgt`
    ($tgt:expr, $prefix:ident, [$($impl:ident),*]) => {
	$crate::_paste::paste! {
	    $(
		$crate::targeted_log!(@internal $tgt, $impl, [<$prefix $impl>], $);
	    )*
	}

    };

    // This branch will generate the macros prefixed with `$prefix` and the
    // specified target `$tgt`.
    ($tgt:expr, $prefix:ident) => {
	$crate::targeted_log!($tgt, $prefix, [error, warn, info, debug, trace]);
    };

    // This branch will generate the macros prefixed with "tg" and the
    // specified target `$tgt`.
    ($tgt:expr) => {
	$crate::targeted_log!($tgt, tg);
    }
}
