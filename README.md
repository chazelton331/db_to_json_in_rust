### Running The App

To run this application, invoke:

```
cargo run
```

Compiled with

```
rustc 1.12.0-beta.2 (389dad798 2016-08-24)
```

##### TODO

1. Add `signed_up_at` timestamp to the `User` struct, and add it to the response.

```
signed_up_at:   chrono::DateTime<UTC>, // ??
```

2. Add `Option` for values that can be `null` and add an example to `seeds.sql`
