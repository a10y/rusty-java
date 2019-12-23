# Rusty Java

Playing around with Gradle, JNI, and the [jni crate](https://docs.rs/jni/0.14.0/jni/) to author native libraries in
Rust for consumption by Java code.

## Running

This uses the gradle application plugin, so simple as

```
$ ./gradlew run
```

## Up Next

- [ ] Rust calling JVM methods
- [ ] JMH benchmarks to evaluate JNI overhead
- [ ] Apache Arrow for in-memory sharing b/w Java + Rust
