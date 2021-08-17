This demonstrates a bug and a fix in the `whoami` crate.

To run:

```shell
docker build .
docker run $CONTAINER_ID
```

Notice that the returned value of `whomai::username()` points to invalid memory.

In `Cargo.toml` uncomment the `[patch.crates-io]` section to apply the fix.
