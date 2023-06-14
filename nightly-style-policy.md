Initial PR(s) implementing new syntax filed against rust-lang/rust should
generally leave the rustfmt subtree untouched. In cases that that need to
modify rustfmt (for example, to fix compiler errors that come from adding
new variants to AST representation), they should modify rustfmt in such a
way to keep existing formatting verbatim.

Rustfmt is allowed to implement nightly-only formatting behavior without that
syntax being specified by the style guide. The initial authors of PRs
implementing new features in rust-lang/rust are encouraged, but not
required, to open a PR against
[rustfmt](https://github.com/rust-lang/rustfmt) suggesting an initial
formatting behavior, or formatting may later be implemented as a PR by anyone,
pending approval of the implementation from T-rustfmt. T-style should be
notified to approve the interim style proposed by these PRs, but this decision
is not binding and may be revisited until the feature is stabilized and the
formatting is codified in the style guide. 

Much like breaking nightly feature changes in the Rust compiler, any changes
to formatting behavior for nightly syntax should be made cautiously and with
thorough consideration to avoid churn. Changes should not be done unnecessarily,
and should take into account the feature's adoption
and readiness for stabilization. However, changes may be done until the feature
is stabilized for various reasons: new understanding of the feature's usage in
the language, recommendation from T-style, or changes in the implementation of
the feature.

Feature stabilization should be blocked on confirmation and codification of
formatting behavior. At this point, T-style may also propose alternative
formatting at the time of stabilization, with any breaking changes weighted
according to the principle stated above.