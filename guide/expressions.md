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

```
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
`! x`).

### Binary operations

Do include spaces around binary ops (i.e., `x + 1`, not `x+1`) (including `=`).

For comparison operators, because for `T op U`, `&T op &U` is also implemented:
if you have `t: &T`, and `u: U`, prefer `*t op u` to `t op &u`. In general,
within expressions, prefer dereferencing to taking references.

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

### Methods

Follow the function rules for calling.

#### Single-line

Do not put any spaces around the `.`.

```rust
x.foo().bar().baz(x, y, z);
```

### as

Put spaces before and after `as`:

```rust
let cstr = "Hi\0" as *const str as *const [u8] as *const std::os::raw::c_char;
```