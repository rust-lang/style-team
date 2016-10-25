- Start Date: 2016-10-25
- RFC Issue: #3
- RFC PR: 
- Implementation Issue: (leave this empty)

# Summary
[summary]: #summary

Customisation of Rustfmt should be allowed (via a `rustfmt.toml` file), but not
encouraged.


# Details
[details]: #details

Users may create a `rustfmt.toml` or `.rustfmt.toml` in their project directory
or any parent of that directory. Rustfmt will read options from the first toml
file (scanning up the directory tree). If an option is not present in the file,
Rustfmt uses the default option (as specified by the code style process RFCs).
The set of options available is not specified at this time.

Customisation will be documented, but explicitly discouraged.

All official Rust projects which use Rustfmt *must* use the default style and
*must not* include a `rustfmt.toml`.

# Implementation
[implementation]: #implementation

Rustfmt already allows customisation via `rustfmt.toml`. There is nothing more
to implement here. Over time, I expect the number of options to be reduced.

See [config.rs](https://github.com/rust-lang-nursery/rustfmt/blob/master/src/config.rs)
for the definition of the config file.

# Rationale
[rationale]: #rationale

Beyond the advantage of automatically formatting code, there is a large
advantage in all code being formatted the same way. This holds within a single
project, and between projects - reading code in a new project is easier if one
does not have to adjust to a new code style at the same time. Since Rust
encourages sharing small crates between projects, this is likely to be a bigger
advantage in Rust than, say, C++.

However, code style is an intensely subjective matter and many programmers feel
strongly about it. Furthermore, the Rust community encourages diversity,
individualism, and democracy as part of its culture, and tends not to accept
dictated decisions. It is therefore likely that if Rustfmt only enforced a
single style, it would be rejected by a significant proportion of the community.

We believe there is more benefit in a large proportion of the community using
Rustfmt in any configuration, than in a small proportion using it in a single
configuration. We also believe that the best approach to encouraging use of the
default style is to lead by example and exert cultural pressure, rather than
enforcement by coercion.

# Alternatives
[alternatives]: #alternatives

No customisation is an alternative. However, we believe this will reduce
Rustfmt's usage and users will either fork Rustfmt or move to a more flexible
formatting tool.

A number of possible 'speed bumps' have been proposed (most are described in
https://github.com/rust-lang-nursery/fmt-rfcs/issues/3). These aim to add a
technical barrier to customisation as well as a social/cultural one. We decided
that any of these measures would be annoying without being effective, and instead
we should rely on cultural pressure.

# Unresolved questions
[unresolved]: #unresolved-questions

How should we control the options Rustfmt makes available? I feel that the
current set of options is too large (catering for some very niche formatting,
and imposing a cost in complexity and hugely expanded scope for bugs), and is
not very coherent. We have considered having RFCs include a set of alternatives
that Rustfmt will support, however, it has been suggested that this might make
the alternatives too official and is perhaps beyond the scope of the style RFC
process.
