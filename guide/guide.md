# Rust Style Guide

## Formatting conventions

These formatting conventions are a work in progress, and may do anything they
like, up to and including eating your laundry.

### Indentation

Use spaces, not tabs.

### Blank lines

Separate items and statements by either zero or one blank lines (i.e., one or
two newlines). E.g,

```rust
fn foo() {
    let x = ...;
    
    let y = ...;
    let z = ...;
}

fn bar() {}
fn baz() {}
```

Formatting tools should make the bounds on blank lines configurable: there
should be separate minimum and maximum numbers of newlines between both
statements and (top-level) items (i.e., four options). As described above, the
defaults for both statements and items should be minimum: 1, maximum: 2.


### [Module-level items](items.md)
### [Statements](statements.md)
### [Expressions](expressions.md)
### [Types](types.md)


### Comments

The following guidelines are recommendations only, a mechanical formatter should
not change comments except to move them within a file. To be clear this means
changing the whitespace before a line comment or the whitespace before or after
a block comment.

Prefer line comments (`//`) to block comments (`/* ... */`).

When using line comments there should be a single space after the opening sigil.

When using single-line block comments there should be a single space after the
opening sigil and before the closing sigil. Multi-line block comments should
have a newline after the opening sigil and before the closing sigil.

Prefer to put a comment on its own line. Where a comment follows code, there
should be a single space before it. Where a block comment is inline, there
should be surrounding whitespace as if it were an identifier or keyword. There
should be no trailing whitespace after a comment. Examples:

```rust
// A comment on an item.
struct Foo { ... }

fn foo() {} // A comment after an item.

pub fn foo(/* a comment before an argument */ x: T) {...}
```

Comments should usually be complete sentences. Start with a capital letter, end
with a period (`.`). An inline block comment may be treated as a note without
punctuation.

Source lines which are entirely a comment should be limited to 80 characters
in length (including comment sigils, but excluding indentation) or the maximum
width of the line (including comment sigils and indentation), whichever is
smaller:

```rust
// This comment goes up to the ................................. 80 char margin.

{
    // This comment is .............................................. 80 chars wide.
}

{
    {
        {
            {
                {
                    {
                        // This comment is limited by the ......................... 100 char margin.
                    }
                }
            }
        }
    }
}
```

#### Doc comments

Prefer line comments (`///`) to block comments (`//* ... */`).

Prefer outer doc comments (`///` or `//*`), only use inner doc comments (`//!`
and `//*!`) to write module-level or crate-level documentation.


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

## [Guiding principles and rationale](principles.md)

## [Non-formatting conventions](advice.md)

## [Cargo.toml conventions](cargo.md)
