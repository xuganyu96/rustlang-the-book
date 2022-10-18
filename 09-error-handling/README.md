# Summary

1. Irrecoverable errors are raised using the `panic!` macro
2. Recoverable errors are handled using the `Result<T, E>` enum, which has two variants `Ok<T>` and `Err<E>`
3. The most straightforward way to handle `Result` is to use the `match` statement
4. `unwrap` and `expect` can be used to handle `Result`. They will panic upon receiving `Result::Err`
5. The `?` operator will force early return when applied to `Option<T>` or `Result<T, E>`; be sure to make the function signature compatible
