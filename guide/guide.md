# Rust Style Guide

## Formatting conventions

These formatting conventions are a work in progress, and may do anything they
like, up to and including eating your laundry.

### Indentation

Use spaces, not tabs.

### [Module-level items](items.md)
### [Statements](statements.md)
### [Expressions](expressions.md)

### Doc comments

Prefer outer doc comments (`///` or `//*`), only use inner doc comments (`//!`
and `/*!`) to write module-level documentation.

### Attributes

Put each attribute on its own line, indented to the indentation of its item.
In the case of inner attributes (`#!`), indent it to the inner indentation (the
indentation of the item + 1). Prefer outer attributes, where possible.

For attributes with argument lists, format like functions.

```rust
#[repr(C)]
#[foo(foo, bar)]
struct CRepr {
    #![repr(C)]
    x: f32,
    y: f32,
}
```

## [Non-formatting conventions](advice.md)

## [Cargo.toml conventions](cargo.md)
