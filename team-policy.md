# Style Team Policy Document

This is a living document tracking the active policies of the style team. It is intended to fill a similar role to the books that many[^1] teams[^2] maintain[^3] independently[^4]. If in the future Rust establishes central linking/indexing/aggregating of policies, these will need to appear there.

* **Original RFC**: https://rust-lang.github.io/rfcs/3309-style-team.html
* **Charter**: https://github.com/rust-lang/style-team/blob/HEAD/charter.md

## Style Guide Evolution

The Rust style guide will generally match the latest version of the Rust style; the style team does not plan to maintain multiple branches of the style guide for different editions, in part because formatting for new constructs will apply to any edition supporting those constructs.

Whenever possible, style decisions should be made before a new construct is stabilized. However, style decisions shall not be considered a blocker for stabilization.

## Policy Review Process

To maintain the effectiveness and relevance of our policies, the style team implements a policy review process. The following guidelines outline how we conduct policy reviews:

1. **Associated Review Dates**: Each policy should have a designated review date documented alongside the policy. These review dates are also added to the team's backlog for easy tracking and planning.
2. **Consideration Factors**: When selecting review dates for policies, we consider several factors. These include:
  1. **Confidence Level**: If a policy has recently changed or is relatively new, we opt for a shorter review period. This allows us to course correct quickly if needed.
  2. **Stability and Effectiveness**: Policies that have remained unchanged for a considerable period and have proven effective can have longer review periods. This prevents unnecessary time spent on policies that are working well.
3. **Integration into Regular Meetings**: Policy reviews are treated as regular agenda items during our team meetings. They are discussed and evaluated alongside other topics of importance.
4. Review Process: Policy reviews should use a discussion approach that encourages each team member to provide feedback regarding the policy under review and creates a constructive environment where everyone is heard equally. Our current process is a *round*, in which each person is heard once before anyone responds or chimes in a second time. We start with the prompt, "How effective has this policy been in your experience? Is it serving its intended purpose? How has it worked well, and how can it be improved? Are there other issues with the policy that we should consider?".
5. **Regular Review Cadence**: We review our policies regularly to identify any issues before they escalate into urgent problems proactively. By conducting these reviews systematically, we ensure that our policies remain up-to-date and aligned with our evolving needs.
  1. **Default disposition**: In organizations without a policy review process, policies often remain in effect by default until explicitly rescinded. The policy review process partly reverses that default disposition: policies require regular review and must be renewed to remain in effect. (Such review and renewal still consider all of these other factors, such as the degree of previously established confidence in the policy and its stability over time.)

By adhering to this policy review process, we aim to maintain a high standard of quality and relevance in our style team policies.

## Operational Roles

The style team has various operational roles to delegate related and ongoing work to specific individuals, to clarify on our operations, and to coordinate sharing feedback with the members in those roles to enable iterative self improvement.

### Operational Lead

* Current Holder: [@calebcartwright](https://github.com/calebcartwright)
* Next Feedback Date: TODO

In order to stay in touch with where we want to be heading in the future, we need leadership. A team leader is paying attention to the team’s operations in relation to the team’s aim. What needs to be done, who agreed to do it. What is in the future to decide?

Responsibilities:

* Pays attention to the operation of the team
* Pays attention to the team members
* Reports wider context to the team

### Facilitator

* Current Holder: [@yaahc](https://github.com/yaahc)
* Next Feedback Date: Wednesday 2023-02-01

In order to be present with each other, we need a good facilitator.  The facilitator runs meetings according to the format of meetings and decision making adopted by the group. Leader and facilitator are separate roles because facilitation and overseeing operations are different skill sets. They can be held by the same individual.

Responsibilities:

* Facilitates team meetings.
* Pays attention to equivalence[^5] during meetings.
* Supports planning of the agenda.
* Distinguishes facilitator voice from team member voice.

### Scribe

* Current Holder: [@calebcartwright](https://github.com/calebcartwright)
* Next Feedback Date: TODO

In order to manage continuity with the team’s past, we need to have written records. The scribe manages the notes during the meeting, makes sure the minutes are distributed and accessible.

Responsibilities:

* Makes sure meeting minutes are taken, approved, and stored.
* Keeps track of all documents of the team.
* Default interpreter of meeting minutes in cases of disagreement.
* Supports planning the agenda from the backlog.
    * Tracking when policies and roles are due for review.

## Meetings

The style team meets weekly on Wednesdays at 12:30pm PST [(everytimezone link)](https://everytimezone.com/s/3f88a253) for a weekly video sync. Agendas are posted in the #T-style zulip stream. 

Unless otherwise noted, all of our meetings are public and open for anyone to attend. You will find the timing and event details on our [style team calendar](https://calendar.google.com/calendar/embed?src=d0564ed914a41cf4915bd5ebe6e2e4ec0ee1293fdc1d09d6f5bdb27d4f91c083%40group.calendar.google.com&ctz=America%2FLos_Angeles). We publish notes and minutes in written form in this github repository.

### Agenda, Backlog, and Minutes

The style team stores its agendas, backlog, and meeting minutes in a single live hackmd document (rotating as necessary when the documents reach their length limit).

The agenda should be prepared in advanced of each meeting by the facilitator with the assistance of the scribe and the lead. The lead is responsible for looking forward and adding agenda items such as new requests from other teams and users or new priorities and goals. The scribe is responsible for looking backwards and adding agenda items from existing work such as items from the backlog or policies that are due for review. The agenda proposal is then presented at the beginning of each meeting by the facilitator for the rest of the team to consent or object too.

Consent not needed to add to items to the backlog, anybody is welcome to add items to the backlog, but consent needed to move from backlog to agenda (since agendas themselves require consent). An item being present on the backlog does not represent a commitment by the style team.

#### Interrupting the Agenda

The style team acknowledges that urgent needs may arise during the meeting. To address these needs, participants may interrupt the meeting by speaking up and immediately saying "Point of Order." This signals to the meeting facilitator and other participants that the following point is not an out-of-order interruption but rather an urgent need that must take priority. This tool is designed for meta-level concerns that take priority over the current discussion, such as the scribe interrupting the meeting to address issues preventing the recording of minutes (and stopping the discussion until they can resume recording minutes), or a team member interrupting a discussion to raise questions of time bounds or team scope.

The meeting facilitator will then pause the current discussion, allow the participant to raise the point of order to state their concern and allow the team to address that point of order. After resolving the urgent matter, the facilitator will resume the previous discussion or return to the agenda. This policy is intended to ensure that the team can address urgent issues promptly without unnecessarily interrupting the flow of the meeting.

#### Inform / Explore / Decide agenda item classification

Agenda items are labeled according to their desired outcome. There are three possible outcomes for any given agenda item, inform, explore, or decide. Each of these outcomes builds upon the previous outcomes. In order to explore an item it must first be understood, and in order to make a decision one must both understand and explore the item first. Inform is used for things such as status updates, and usually involves first a report, followed by a round of clarifying questions. Explore is used for situations where one would like feedback on a potential issue or proposal, and usually involves reaction rounds. Decide is used when the team must reach a decision as a group, and it is usually achieved via a consensus round.

We identify the desired outcome in advance to avoid aimless discussions with unclear goals. The desired outcome can change during discussion as new information becomes available. It is the facilitator's responsibility to notice when this happens and formally make the change in the desired outcome with consent from the rest of the team. Extra clarity can be gained if, for every agenda item, we end by measuring whether we have achieved the desired outcome. Facilitators can make it a habit to pause before moving to a new agenda item by assessing whether the desired outcome has been achieved and by asking the scribe to read out loud what has been written in the notes.

The agenda backlog and minutes document is structured according to the following template:

```md
# T-style Minutes

Meeting Link: 

## Action Items

### Pending

* owner: bullet point list of items that are in progress and assigned to a specific person

### Completed

- [ ] owner: check list of items that are completed or assumed to be complete
- [x] owner: items are checked off once they've been reviewed in a meeting, confirmed complete, and given any relevant final status updates.
- [x] owner: after a meeting the items that were checked off are moved into the `#### Completed Action Items` section of the meeting they were reviewed in by the scribe
### Granularity

Action items should not generally list individual feedback/response comments within GitHub threads. An action item should generally be at the level of a GitHub PR, and people can look at the GitHub PR to find out its current status. That status may affect whether the action item is blocked, or who needs to look at it, or whether there are aspects of the action item to discuss in a meeting.

## Backlog

* Bullet point list of items that have not been started or assigned yet

## Next Meeting Date

### Attendance

### Agenda

* (inform) bullet point list of proposed agenda items (labeled either inform, explore, or decide)
* Review Action Items
* Meeting Check-out
    * Do not record in minutes, exceptions can be made with consent of team

### Minutes

#### Individual Agenda Items

Notes related to the given agenda item

#### Completed Action Items

## Previous Meeting Dates <repeats>

```
### Meeting Check-out

The style team wraps up the content part of our meetings 5-10 minutes before team members have to leave to make room for regular meeting evaluations. Meeting evaluations are an integral part of every meeting. We end the meeting with one or two rounds on:

* “What worked well in the meeting?”
* “What could be improved in future meetings?”
* “Is there anything you are carrying out of the meeting that you’d like to get off your chest now?”

Meeting evaluations are an opportunity to learn from our meetings. We can either talk about content, process, or interpersonal dynamics. We utilize meeting evaluations to help ourselves inhabit a growth mindset. Our goal is to have meetings which are well-run, refreshing, connecting, and energizing. We achieve this goal by giving space for people to speak up about both the positive and negative aspects of how we're working together so that we can prioritize continuous improvement and positive connections.

Meeting check-outs are considered private and internal to the style team and are not recorded as part of our minutes. Exceptions to this can be made via an operational consent decision by the team, and are often useful in cases such as when new backlog or action items come up during the check-out.

[Style Guide]: https://github.com/rust-lang/rust/tree/HEAD/src/doc/style-guide/src
[#T-style]: https://rust-lang.zulipchat.com/#narrow/stream/346005-t-style

[^1]: https://rust-lang.github.io/compiler-team/
[^2]: https://lang-team.rust-lang.org/
[^3]: https://std-dev-guide.rust-lang.org/
[^4]: https://rust-lang.github.io/types-team/
[^5]: Equality, making sure everyone's voices and feedback are receiving equitable attention.
