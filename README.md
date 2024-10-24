# Outer macro pattern

Rust macros normally transform code they apply to, and are isolated. A macro
cannot affect shared state reliably, as compilation order is not guaranteed. So
you can't use a Rust macro to register anything (structs, functions, etc).

The outer macro pattern gets around this by using *two* macros. Inner macros
don't do anything except mark items for registration. They do not transform
code (though they could). The outer macro is defined on a `mod` statement, and
its effect is to collect information inside the module that is marked; register
them as some kind of state. You can then use this information to do what you
like (generate code that depends on the registrations).

The outer macro pattern has some limitations:

* It only gets applied to the contents of inline `mod` statements. Non-inline
  `mod` statements are not currently supported by (stable) Rust. So users have
  to use a nested mod statement.

* It's not straightforward to register things defined in another module.

The [`macro_magic`](https://docs.rs/macro_magic/latest/macro_magic/index.html)
crate can be used to work around these limitations.

## History

I couldn't find any written resources on the outer macro pattern for Rust, only
references to [this video](https://www.youtube.com/watch?v=aEWbZxNCH0A). The
video is quite helpful, but I wanted to understand the idea in code, which is
why this repository exists.