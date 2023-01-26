# Rust code formatting RFCs

This repository exists to decide on a code style for Rust code, to be enforced
by the Rustfmt tool. Accepted RFCs live in the `text` directory and form a
specification for formatting tools.

The [Rust style guide](guide/guide.md) in this repository documents this style,
including examples.

## The formatting RFC process

See [RFC 1607](https://github.com/rust-lang/rfcs/pull/1607) for more details.
Where this process is under-specified, see the process for [Rust RFCs](https://github.com/rust-lang/rfcs).

* Open a GitHub issue on the fmt-rfcs repo for discussion. This discussion should
  define the style in as much detail as possible using rules and examples.
  - Search for existing issues and RFCs that may have already covered the topic.
    To discourage endless bikeshedding, the style team will close new discussions
    on old topics unless they provide fresh information (such as feedback based
    on having an implementation) or alternatives that were not previously
    considered.
* When discussion has reached a fixed point, the issue will be put into a final
  comment period (FCP).
* Reach consensus on the issue.
* Implement the style in rustfmt (behind an option if it is not the current
  default).
* Submit a PR to fmt-rfcs updating the style guide with the new style and the
  examples using the newly updated Rustfmt. The PR should include the default
  values for options to enforce the guideline, and which non-default options
  should be kept.
* If discussion is brief and the PR closely matches the original issue, it will
  be merged. If there are changes, then we will have an FCP for the PR too.
* Implementation in Rustfmt can then be finished (including any changes due to
  discussion of the PR), and default options are set.


### Scope of the process

This process is specifically limited to formatting style guidelines which can be
enforced by Rustfmt with its current architecture. Guidelines that cannot be
enforced by Rustfmt without a large amount of work are out of scope, even if
they only pertain to formatting.


### FCP

FCP will last for approximately two weeks and will be announced in This Week in
Rust.


### Decision and post-decision process

The style team will make the ultimate decision on accepting or closing a style
issue. Decisions should be by consensus. Most discussion should take place on
the issue comment thread, a decision should ideally be made when consensus is
reached on the thread. Any additional discussion amongst the style team will be
summarised on the thread.


## Guiding principles

When deciding on style guidelines, discussion should be steered by the following
principles (in priority order):

* readability
    - scan-ability
    - avoiding misleading formatting
    - accessibility - readable and editable by users using the the widest
      variety of hardware, including non-visual accessibility interfaces
    - readability of code when quoted in rustc error messages

* aesthetics
    - sense of 'beauty'
    - consistent with other languages/tools

* specifics
    - compatibility with version control practices - preserving diffs,
      merge-friendliness, etc.
    - preventing right-ward drift
    - minimising vertical space

* application
    - ease of manual application
    - ease of implementation (in Rustfmt, and in other tools/editors/code generators)
    - internal consistency
    - simplicity of formatting rules

To see how these principles were decided, see
[issue 4](https://github.com/rust-lang-nursery/fmt-rfcs/issues/4).
