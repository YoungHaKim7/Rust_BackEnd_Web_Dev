# Result

```

$ cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/a01_surrealdb01`

[src/main.rs:51] created = [
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "2n9jzggbvna18yxw5t53",
            ),
        },
    },
]
[src/main.rs:58] updated = Some(
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "jaime",
            ),
        },
    },
)
[src/main.rs:62] people = [
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "2n9jzggbvna18yxw5t53",
            ),
        },
    },
    Record {
        id: Thing {
            tb: "person",
            id: String(
                "jaime",
            ),
        },
    },
]
[src/main.rs:69] groups = Response(
    {
        0: (
            Stats {
                execution_time: Some(
                    36.846µs,
                ),
            },
            Ok(
                Array(
                    Array(
                        [
                            Object(
                                Object(
                                    {
                                        "count": Number(
                                            Int(
                                                2,
                                            ),
                                        ),
                                        "marketing": Bool(
                                            true,
                                        ),
                                    },
                                ),
                            ),
                        ],
                    ),
                ),
            ),
        ),
    },
)
```
