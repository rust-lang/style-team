- Start Date: 2016-05-04
- RFC PR: (leave this empty)
- Implementation Issue: (leave this empty)

# Summary
[summary]: #summary

Define the maximum width of a line and the width of indentation.


# Details
[details]: #details

The maximum width of a line is 100 characters. The maximum width of a comment
(including any commenting characters, e.g., `//`, but not including any
indentation) is 80 characters. A comment should be wrapped at the minimum of the
80 character comment limit and the 100 character line limit.

Indentation should use spaces, not tabs. Each level of indentation should be
four characters wide.

Empty lines should have no whitespace, even if lines above and below are
indented.

Example:

```
// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
// tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
// quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
// consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
// cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
// proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
mod foo {
    mod bar {
        // Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
        // tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
        // quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
        // consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
        // cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
        // proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
        fn baz() {
            this_is_a_very_long_function_name_and_the_line_is_long(1 + 2 + 3 + 4 + 5 + 6 + (7 + 8));
        }

        // Note, the above line is empty, no indentation.
        fn qux() {
        }
    }
}

fn main() {
    println!("Hello, world!");
}
```


# Implementation
[implementation]: #implementation

Rustfmt options:

* `max_width` remains defaulted to `100`.
* `ideal_width` is removed.
* A new option, `comment_width`, is added with value `80` and type `usize`.
* `tab_spaces` is renamed to `indent_width` and remains `4`.
* `hard_tabs` remains `false`.

All these options remain configurable. A `comment_width` of 0 means that Rustfmt
will not take into account a maximum comment width and will only be constrained
by the line width.

The only implementation work required is the change from having an `ideal_width`
(which only applies to comments) to `comment_width`. The change is that
`ideal_width` was a line width, including indentation, whereas `comment_width`
is not.

[Note: this level of implementation is something of a grey area - the change is
a tweak to an existing option, but is bordering on a big enough change to
warrant implementation before the RFC is submitted. The reality is that at the
moment comment formatting in Rustfmt is pretty bad and so implementation is a
bit irrelevant. I hope by the time the fmt-rfcs repository is live and the
formal process starts for this PR, comments are in a better state and this
implementation work will be done].


# Rationale
[rationale]: #rationale

Common choices for line width are 80 and 100 characters. Restricting to 100
(rather than a larger number) is good for users with older hardware, reading
code on mobile devices, using terminals, and having two files side-by-side on a
wide screen.

Indentation is commonly two or four spaces. Four seems to be more popular in the
Rust community and is used in the Rust project itself (and pretty much every
other project).

Due to four-space indent and Rust's 'culture' of rightward drift, 100 spaces is
more appropriate than 80. It is widely used (Rust, Servo, most other projects).

Comment lines are usually prose rather than code, therefore there are different
pressures. [Studies](http://baymard.com/blog/line-length-readability)
have shown that wider lines of text are harder to read. Especially for long
comments (e.g., documentation at the head of modules), 100 character-wide text
feels extremely 'heavy'. This de facto limit is adhered to by most of the Rust
project (e.g., [libstd](https://github.com/rust-lang/rust/blob/master/src/libstd/lib.rs)).


# Alternatives
[alternatives]: #alternatives

The style guide currently recommends 99 character wide lines. This allows for
diffs to be 100 characters wide. However, this rule is not followed in practice
by the Rust project (where `make tidy` enforces a 100 character limit).

We could limit the width of comment lines to 80 characters including
indentation. This is probably easier to format manually (text editors usually
have a line wrap function which does this). However, it means deeply indented
comments end up very narrow.

Rustfmt could be made significantly simpler by removing the `hard_tabs` option.
However, it is a popular alternative, so should probably be left as an option.

# Unresolved questions
[unresolved]: #unresolved-questions

None.
