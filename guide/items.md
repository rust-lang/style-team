### Function definitions

In Rust, one finds functions by searching for `fn [function-name]`; It's
important that you style your code so that it's very searchable in this way.

The proper ordering and spacing is:

```rust
[pub] [unsafe] [extern ["ABI"]] fn foo(arg1: i32, arg2: i32) -> i32 {
    ...
}
```

Avoid comments within the signature itself.

### Tuples and tuple structs

Write the type list as you would a parameter list to a function.

Build a tuple or tuple struct as you would call a function.

#### Single-line

```rust
struct Bar(Type1, Type2);

let x = Bar(11, 22);
let y = (11, 22, 33);
```

### Enums

In the declaration, put each variant on its own line.

Format each variant accordingly as either a `struct`, `tuple struct`, or ident,
which doesn't require special formatting.

```rust
enum FooBar {
    First(u32),
    Second,
    Error {
        err: Box<Error>,
        line: u32,
    },
}
```

### Structs and Unions

Struct names follow on the same line as the `struct` keyword, with the opening
brace on the same line when it fits within the right margin. All struct fields
are indented once and end with a trailing comma. The closing brace is not
indented and appears on its own line.

```rust
struct Foo {
    a: A,
    b: B,
}
```

If and only if the type of a field does not fit within the right margin, it is
pulled down to its own line and indented again.

```rust
struct Foo {
    a: A,
    long_name: 
        LongType,
}
```

The same guidelines are used for untagged union declarations.

```rust
union Foo {
    a: A,
    b: B,
    long_name: 
        LongType,
}
```

### Extern crate

`extern crate foo;`

Use spaces around keywords, no spaces around the semi-colon.

### Modules

```rust
mod foo {
}
```

```rust
mod foo;
```

Use spaces around keywords and before the opening brace, no spaces around the
semi-colon.

### macro\_rules!

Use `{}` for the full definition of the macro.

```rust
macro_rules! foo {
}
```

### Generics

TODO


### `where` clauses

These rules apply for `where` clauses on any item.

A `where` clause may immediately follow a closing bracket of any kind.
Otherwise, it must start a new line, with no indent. Each component of a `where`
clause must be on its own line and be block indented. There should be a trailing
comma, unless the clause is terminated with a semicolon. If the `where` clause
is followed by a block (or assignment), the block should be started on a new
line. Examples:

```
fn function<T, U>(args)
where
    T: Bound,
    U: AnotherBound,
{
    body
}

fn foo<T>(
    args
) -> ReturnType
where
    T: Bound,
{
    body
}

fn foo<T, U>(
    args,
) where
    T: Bound,
    U: AnotherBound,
{
    body
}

fn foo<T, U>(
    args
) -> ReturnType
where
    T: Bound,
    U: AnotherBound;  // Note, no trailing comma.

type Foo<T>
where
    T: Bound
= Bar<T>;
```

If a `where` clause is very short, we recommend using an inline bound on the
type parameter.


If a component of a `where` clause is long, it may be broken before `+` and
further block indented. Each bound should go on its own line. E.g.,

```
impl<T: ?Sized, Idx> IndexRanges<Idx> for T
where
    T: Index<Range<Idx>, Output = Self::Output>
        + Index<RangeTo<Idx>, Output = Self::Output>
        + Index<RangeFrom<Idx>, Output = Self::Output>
        + Index<RangeInclusive<Idx>, Output = Self::Output>
        + Index<RangeToInclusive<Idx>, Output = Self::Output> + Index<RangeFull>
```

#### Option - `where_single_line`

`where_single_line` is `false` by default. If `true`, then a where clause with
exactly one component may be formatted on a single line if the rest of the
item's signature is also kept on one line. In this case, there is no need for a
trailing comma and if followed by a block, no need for a newline before the
block. E.g.,

```
// May be single-lined.
fn foo<T>(args) -> ReturnType
where T: Bound {
    body
}

// Must be multi-lined.
fn foo<T>(
    args
) -> ReturnType
where
    T: Bound,
{
    body
}
```
