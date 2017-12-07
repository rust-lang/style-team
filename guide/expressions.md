## Expressions

### Blocks

A block expression should have a newline after the initial `{` and before the
terminal `}`. Any qualifier before the block (e.g., `unsafe`) should always be
on the same line as the opening brace, and separated with a single space. The
contents of the block should be block indented:

```rust
fn block_as_stmt() {
    a_call();

    {
        a_call_inside_a_block();

        // a comment in a block
        the_value
    }
}

fn block_as_expr() {
    let foo = {
        a_call_inside_a_block();

        // a comment in a block
        the_value
    };
}

fn unsafe_block_as_stmt() {
    a_call();

    unsafe {
        a_call_inside_a_block();

        // a comment in a block
        the_value
    }
}
```

If a block has an attribute, it should be on its own line:

```rust
fn block_as_stmt() {
    #[an_attribute]
    {
        #![an_inner_attribute]

        // a comment in a block
        the_value
    }
}
```

Avoid writing comments on the same line as the braces.

An empty block should be written as `{}`.

A block may be written on a single line if:

* it is either
  - used in expression position (not statement position)
  - is an unsafe block in statement position
* contains a single-line expression and no statements
* contains no comments

A single line block should have spaces after the opening brace and before the
closing brace.

Examples:

```rust
fn main() {
    // Single line
    let _ = { a_call() };
    let _ = unsafe { a_call() };

    // Not allowed on one line
    // Statement position.
    {
        a_call()
    }

    // Contains a statement
    let _ = {
        a_call();
    };
    unsafe {
        a_call();
    }

    // Contains a comment
    let _ = {
        // A comment
    };
    let _ = {
        // A comment
        a_call()
    };

    // Multiple lines
    let _ = {
        a_call();
        another_call()
    };
    let _ = {
        a_call(
            an_argument,
            another_arg,
        )
    };
}
```


### Closures

Don't put any extra spaces before the first `|` (unless the closure is prefixed
by `move`), but put a space between the second `|` and the expression of the
closure. Between the `|`s, you should use function definition syntax, however,
elide types where possible.

Use closures without the enclosing `{}`, if possible. Add the `{}` when you
have a return type, when there are statements before the final expression, or
when you need to split it into multiple lines; examples:

```rust
|arg1, arg2| expr

move |arg1: i32, arg2: i32| -> i32 { expr1; expr2 }
```

### Array literals

For simple array literals, avoid line breaking, no spaces around square
brackets, contents of the array should be separated by commas and spaces. If
using the repeating initialiser, there should be a space after the semicolon
only. Apply the same rules if using the `vec!` or similar macros (always use
square brackets here). Examples:

```rust
fn main() {
    [1, 2, 3];
    vec![a, b, c, d];
    let a = [42; 10];
}
```

If a line must be broken, prefer breaking only after the `;`, if possible.
Otherwise, follow the rules below for function calls. In any case, the contents
of the initialiser should be block indented and there should be line breaks
after the opening bracket and before the closing bracket:

```rust
fn main() {
    [
        a_long_expression();
        1234567890
    ]
    let x = [
        an_expression,
        another_expression,
        a_third_expression,
    ];
}
```


### Array accesses, indexing, and slicing.

No spaces around the square brackets, avoid breaking lines if possible, never
break a line between the target expression and the opening bracket. If the
indexing expression covers multiple lines, then it should be block indented and
there should be newlines after the opening brackets and before the closing
bracket. However, this should be avoided where possible.

Examples:

```rust
fn main() {
    foo[42];
    &foo[..10];
    bar[0..100];
    foo[4 + 5 / bar];
    a_long_target[
        a_long_indexing_expression
    ];
}
```

### Unary operations

Do not include a space between a unary op and its operand (i.e., `!x`, not
`! x`). However, there must be a space after `&mut`. Avoid line-breaking
between a unary operator and its operand.

### Binary operations

Do include spaces around binary ops (i.e., `x + 1`, not `x+1`) (including `=`).

For comparison operators, because for `T op U`, `&T op &U` is also implemented:
if you have `t: &T`, and `u: U`, prefer `*t op u` to `t op &u`. In general,
within expressions, prefer dereferencing to taking references.

Use parentheses liberally, do not necessarily elide them due to precedence.
Tools should not automatically insert or remove parentheses. Do not use spaces
to indicate precedence.

If line-breaking, put the operator on a new line and block indent. E.g.,

```rust
foo + bar + baz
    + qux + whatever
```

### Control flow

Do not include extraneous parentheses for `if` and `while` expressions.

```rust
if true {
}
```

is better than

```rust
if (true) {
}
```

Do include extraneous parentheses if it makes an arithmetic or logic expression
easier to understand (`(x * 15) + (y * 20)` is fine)

### Function calls

Do not put a space between the function name, and the opening parenthesis.

Do not put a space between an argument, and the comma which follows.

Do put a space between an argument, and the comma which precedes it.

#### Single-line calls

Do not put a space between the function name and open paren, between the open
paren and the first argument, or between the last argument and the close paren.

Do not put a comma after the last argument.

```rust
foo(x, y, z)
```

### Method calls

Follow the function rules for calling.

#### Single-line

Do not put any spaces around the `.`.

```rust
x.foo().bar().baz(x, y, z);
```

### Casts (`as`)

Put spaces before and after `as`:

```rust
let cstr = "Hi\0" as *const str as *const [u8] as *const std::os::raw::c_char;
```


### Match

Prefer not to line-break inside the discriminant expression. There must always
be a line break after the opening brace and before the closing brace. The match
arms must be block indented once:

```rust
match foo {
    // arms
}

let x = match foo.bar.baz() {
    // arms
};
```

Use a trailing comma for a match arm if and only if not using a block. 

Avoid splitting the left-hand side (before the `=>`) of a match arm where
possible. If the right-hand side of the match arm is kept on the same line,
never use a block (unless the block is empty).

If the right-hand side consists of multiple statements or has line comments or
the start of the line cannot be fit on the same line as the left-hand side, use
a block.

The body of a block arm should be block indented once.

Examples:

```rust
match foo {
    foo => bar,
    a_very_long_patten | another_pattern if an_expression() => {
        no_room_for_this_expression()
    }
    foo => {
        // A comment.
        an_expression()
    }
    foo => {
        let a = statement();
        an_expression()
    }
    bar => {}
    // Trailing comma on last item.
    foo => bar,
}
```

If the body is a single expression with no line comments is a *combinable
expression* (see below for details), then it may be started on the same line as
the right-hand side. If not, then it must be in a block. Example,

```rust
match foo {
    // A combinable expression.
    foo => a_function_call(another_call(
        argument1,
        argument2,
    )),
    // A non-combinable expression
    bar => {
        a_function_call(
            another_call(
                argument1,
                argument2,
            ),
            another_argument,
        )
    }
}
```

#### Line-breaking

Where it is possible to use a block form on the right-hand side and avoid
breaking the left-hand side, do that. E.g.

```rust
    // Assuming the following line does done fit in the max width
    a_very_long_pattern | another_pattern => ALongStructName {
        ...
    },
    // Prefer this
    a_very_long_pattern | another_pattern => {
        ALongStructName {
            ...
        }
    }
    // To splitting the pattern.
```

Never break after `=>` without using the block form of the body.

If the left-hand side must be split and there is an `if` clause, break before
the `if` and block indent. In this case, always use a block body and start the
body on a new line:

```rust
    a_very_long_pattern | another_pattern
        if expr =>
    {
        ...
    }
```

If required to break the pattern, put each clause of the pattern on its own
line, breaking before the `|`. If there is an `if` clause, then you must use the
above form:

```rust
    a_very_long_pattern
    | another_pattern
    | yet_another_pattern
    | a_forth_pattern => {
        ...
    }
    a_very_long_pattern
    | another_pattern
    | yet_another_pattern
    | a_forth_pattern
        if expr =>
    {
        ...
    }
```

If every clause in a pattern is *small*, but does not fit on one line, then the
pattern may be formatted across multiple lines with as many clauses per line as
possible. Again break before a `|`:

```rust
    foo | bar | baz
    | qux => {
        ...
    }
```

We define a pattern clause to be *small* if it matches the following grammar:

```
[small, ntp]:
    - single token
    - `&[single-line, ntp]`

[small]:
    - `[small, ntp]`
    - unary tuple constructor `([small, ntp])`
    - `&[small]`
```

E.g., `&&Some(foo)` matches, `Foo(4, Bar)` does not.


### Combinable expressions

TODO (#61)


### Ranges

Do not put spaces in ranges, e.g., `0..10`, `x..=y`, `..x.len()`, `foo..`.

When writing a range with both upper and lower bounds, if the line must be
broken, break before the range operator and block indent the second line:

```rust
a_long_expression
    ..another_long_expression
```

For the sake of indicating precedence, we recommend that if either bound is a
compound expression, then use parentheses around it, e.g., `..(x + 1)`,
`(x.f)..(x.f.len())`, or `0..(x - 10)`.
