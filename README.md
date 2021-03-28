# targeted_log

This macro is intended to be used with the log crate. This macro
generates macros that applies some specific target to the common
logging macros (`info!`, `error!`...). The idea behind this
is preventing to explicitly specifying the `target:` parameter on each
call to these functions, by letting a macro do that on your
behalf. For example, if we want to add the target "potato" to all the
calls of a specific module we could do:

```rust
use targeted_log::targeted_log;

targeted_log!("potato");
fn do_some_work() {
    tginfo!("Hey! I'm doing some work");
    tgwarn!("Warning! This is a warning!");
}
```

This code will be equivalent to:

```rust
use log::{info, warn};

fn do_some_work() {
    info!(target: "potato", "Hey! I'm doing some work");
    warn!(target: "potato", "Warning! This is a warning!");
}
```

You can also add a format specifier to the target itself, to make it a bit more flexible:
```edition2018
use targeted_log::targeted_log;
///
targeted_log!("Log {} {}");
fn do_some_work() {
    tginfo!(@ 1, 2; "Hey! I'm doing some work");
    tgwarn!(@ 3, 4; "Warning! This is a warning!");
}
```

Is equivalent to doing:

```edition2018
use log::{info, warn};
///
fn do_some_work() {
    info!(target: &format!("Log {} {}", 1, 2), "Hey! I'm doing some work");
    warn!(target: &format!("Log {} {}", 3, 4), "Warning! This is a warning!");
}
```

Of course, calling this macro more than once per module will cause
a conflict between names.  For that cases, when we want to use
multiple logging targets within one module, we can specify the
prefix for the macros we're going to define. For example, this
calls:

```rust
use targeted_log::targeted_log;
targeted_log!("app", app_);
targeted_log!("server", srv_);
```

Will generate macros like `app_debug, app_info, srv_warn, srv_info`...
