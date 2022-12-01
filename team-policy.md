# Style Team Policy Document

This is a living document tracking the active policies of the style team. It is intended to fill a similar role to the books that many[^1] teams[^2] maintain[^3] independently[^4] with the intent that eventually these can be integrated into a central policy repository for all teams in the rust-lang organization.

**Original RFC**: https://rust-lang.github.io/rfcs/3309-style-team.html

## Aims: Evolving the Rust Style over time

1. Selecting styling for new Rust constructs
1. Evolving the existing style over the course of Rust editions (without breaking backwards compatibility)
1. Defining mechanisms to evolve the Rust style while taking backwards compatibility into account, such as via Rust editions or similar mechanisms

## Domains

1. [Style Guide]
1. [T-style team repo](https://github.com/rust-lang/fmt-rfcs/)
1. [#T-style] and #T-style/private zulip streams
1. rust-lang issues with the `T-style` or `T-style-nominated` labels

## Membership

* Lead: Caleb Cartwright ([@calebcartwright](https://github.com/calebcartwright))
* Jane Losare-Lusby ([@yaahc](https://github.com/yaahc))
* Josh Triplett ([@joshtriplett](https://github.com/joshtriplett))
* Michael Goulet ([@compiler-errors](https://github.com/compiler-errors))

## Meetings

The style team meets weekly on wednesdays at 11:30am PST for a weekly video sync. Agendas are posted in the [#T-style] zulip stream. 

Unless otherwise noted, all of our meetings are open to the public for anyone to attend. You will find the timing and event details on our [style team calendar](TODO). We publish notes and minutes in written form in this github repository.

## Membership

The Rust style team shall have at least 3 members and at most 8. If the team has fewer than 3 members it shall seek new members as its primary focus.

Members of the style team are nominated by existing members. All existing members of the team must affirmatively agree to the addition of a member, with zero objections; if there is any objection to a nomination, the new member will not be added. In addition, the team lead or another team member will check with the moderation team regarding any person nominated for membership, to provide an avenue for awareness of concerns or red flags.

## Style Guide Evolution

The Rust style guide will generally match the latest version of the Rust style; the style team does not plan to maintain multiple branches of the style guide for different editions, in part because formatting for new constructs will apply to any edition supporting those constructs.

Whenever possible, style decisions should be made before a new construct is stabilized. However, style decisions shall not be considered a blocker for stabilization.

[Style Guide]: https://github.com/rust-lang/fmt-rfcs/blob/master/guide/guide.md
[#T-style]: https://rust-lang.zulipchat.com/#narrow/stream/346005-t-style

[^1]: https://rust-lang.github.io/compiler-team/
[^2]: https://lang-team.rust-lang.org/
[^3]: https://std-dev-guide.rust-lang.org/
[^4]: https://rust-lang.github.io/types-team/