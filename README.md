# Rust code formatting RFCs

This repository exists to decide on a code style for Rust code, to be enforced
by the Rustfmt tool. Accepted RFCs live in the `text` directory and form a
specification for formatting tools.

## The formatting RFC process

See [RFC 1607](https://github.com/rust-lang/rfcs/pull/1607) for more details.
Where this process is under-specified, see the process for [Rust RFCs](https://github.com/rust-lang/rfcs).

* If there is no single, obvious style, then open a GitHub issue on the
  fmt-rfcs repo for initial discussion. This initial discussion should identify
  which Rustfmt options are required to enforce the guideline.
* Implement the style in rustfmt (behind an option if it is not the current
  default). In exceptional circumstances (such as where the implementation would
  require very deep changes to rustfmt), this step may be skipped.
* Write an RFC formalising the formatting convention and referencing the
  implementation, submit as a PR to fmt-rfcs. The RFC should include the default
  values for options to enforce the guideline and which non-default options
  should be kept.
* The RFC PR will be triaged by the style team and either assigned to a team
  member for [shepherding](https://github.com/rust-lang/rfcs#the-role-of-the-shepherd),
  or closed.
* When discussion has reached a fixed point, the RFC PR will be put into a final
  comment period (FCP).
* After FCP, the RFC will either be accepted and merged or closed.
* Implementation in Rustfmt can then be finished (including any changes due to
  discussion of the RFC), and defaults are set.


### Scope of the process

This process is specifically limited to formatting style guidelines which can be
enforced by Rustfmt with its current architecture. Guidelines that cannot be
enforced by Rustfmt without a large amount of work are out of scope, even if
they only pertain to formatting.


### Size of RFCs

RFCs should be self-contained and coherent, whilst being as small as possible to
keep discussion focused. For example, an RFC on 'arithmetic and logic
expressions' is about the right size; 'expressions' would be too big, and
'addition' would be too small.


### When is a guideline ready for RFC?

The purpose of the style RFC process is to foster an open discussion about style
guidelines. Therefore, RFC PRs should be made early rather than late. It is
expected that there may be more discussion and changes to style RFCs than is
typical for Rust RFCs. However, at submission, RFC PRs should be completely
developed and explained to the level where they can be used as a specification.

A guideline should usually be implemented in Rustfmt **before** an RFC PR is
submitted. The RFC should be used to select an option to be the default
behaviour, rather than to identify a range of options. An RFC can propose a
combination of options (rather than a single one) as default behaviour. An RFC
may propose some reorganisation of options.

Usually a style should be widely used in the community before it is submitted as
an RFC. Where multiple styles are used, they should be covered as alternatives
in the RFC, rather than being submitted as multiple RFCs. In some cases, a style
may be proposed without wide use (we don't want to discourage innovation),
however, it should have been used in *some* real code, rather than just being
sketched out.


### Triage

RFC PRs are triaged by the style team. An RFC may be closed during triage (with
feedback for the author) if the style team think it is not specified in enough
detail, has too narrow or broad scope, or is not appropriate in some way (e.g.,
applies to more than just formatting). Otherwise, the PR will be assigned a
shepherd as for other RFCs.


### FCP

FCP will last for two weeks (assuming the team decide to meet every two weeks)
and will be announced in the style team sub-team report.


### Decision and post-decision process

The style team will make the ultimate decision on accepting or closing a style
RFC PR. Decisions should be by consensus. Most discussion should take place on
the PR comment thread, a decision should ideally be made when consensus is
reached on the thread. Any additional discussion amongst the style team will be
summarised on the thread.

If an RFC PR is accepted, it will be merged. An issue for implementation will be
filed in the appropriate place (usually the Rustfmt repository) referencing the
RFC. If the style guide needs to be updated, then an issue for that should be
filed on the Rust repository.

The author of an RFC is not required to implement the guideline. If you are
interested in working on the implementation for an 'active' RFC, but cannot
determine if someone else is already working on it, feel free to ask (e.g. by
leaving a comment on the associated issue).
  