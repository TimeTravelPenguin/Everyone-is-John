# Everyone is John

[![Deploy to GitHub Pages](https://github.com/TimeTravelPenguin/Everyone-is-John/actions/workflows/rust.yml/badge.svg)](https://github.com/TimeTravelPenguin/Everyone-is-John/actions/workflows/rust.yml)

This project is a tracking tool to aid in playing the game, _Everyone is John_, with tools
for variations.

**Everyone is John** is a was original created by _Michael Sullivan_. Many resources have
come and gone online, so I will provide a brief description of the game information and
rules.

More information can be found [here](https://rtwolf.github.io/Everyone-is-John/).

Features:

- [ ] Player/Voices tracking: track willpower, points, and obsession goal accomplishments.
- [ ] Add simple notes on NPCs.
- [ ] An item inventory to track collected (or removed) items.

## The Game

Everyone is John (EiJ) is a simple roleplaying game that requires three or more people. It
is very flexible can can be modified to fit you and your friends' interests and play
style.

One player, called the _Game Master_, or _GM_, runs the game. All other players are known
as the _Voices_ inside of John's head. It is the goal of the voices to maintain control
over John in order to maximise completing certain individual goals.

## Voices

Voices have three important attributes defining their character sheet:

- **Willpower**: a pool of points that the Voice can spend to take control of John or
  improve its chance of success in any given action. An initial amount of 10 willpower is
  a good starting point for most voices.
- **Skills**: each voice usually has 2 or 3 skills. A skill is a short descriptive
  attribute that helps aid in the success of tasks performed by that voice. For example,
  a simple skill such as "is very pursuasive" might make it easier to perform certain
  tasks when NPCs are involved. In some cases, it might be fun to sacrifice some willpower
  for an additional skill.
- **Obsessions**: Obsessions come in three levels, each describing the difficulty of a
  task that a voice aims to accomplish. Level 1 represents tasks that are easy to
  accomplish, like "Pigging out on candy". Level 2 obsessions are more difficult or risky,
  such as "dressing up in women's clothing and hitting on strangers". Finally, Level 3 is
  reserved for tasks that are nearly impossible to accomplish, such as "blowing up
  buildings". By completing obsessions, voices earn points towards winning the game.
  Harder obsessions contribute more to winning than smaller ones.

If playing against the other voices with the goal of winning, you may wish to keep you
character sheet details a secret!

## John

John is a totally insane man in Minneapolis (or wherever you deep appropriate). He is
controlled by the Voices in his head -- one at a time, of course.

John is not terribly competent; he has difficulty with a lot of things that you and I
might take for granted. Whenever John attempts anything that an ordinary person might have
any chance of failure at, he needs to roll for success.

The Voice who is currently in control of John does the rolling. If that Voice has a skill
that covers the challenge, the Voice needs to roll a 3 or higher on the single d6. If it
doesn't have a skill, it needs to roll a 6. However, before the roll, the Voice can spend
any number of Willpower points to get a +1 per point spent on the die roll. This can make
success automatic.

Becoming the active Voice is a bit of a challenge, though. Whenever John wakes up or gets
hurt, a test for control of John happens. Also, whenever the currently active Voice fails
a roll or completes its obsession, a test for control of John happens.

When a test for control of John occurs, all of the Voices who are interested
simultaneously bid one or more Willpower points. Voices don't have to bid if they don't
want to (and you can hold out zero tokens if you want to fake out the other Voices). The
highest number of Willpower becomes the active Voice. If multiple people bid the same
highest amount, then they roll off to see who becomes active.

The Voice who becomes active loses the amount of Willpower it bids. All others keep their
bids. It's perfectly acceptable for the previously active Voice to win a bid and remain
active.

Whenever John wakes up, the struggle for control of John happens _before_ the GM describes
the situation John wakes up into.

John is pretty easily distracted. Whenever nothing exciting is happening for ten minutes
or more (such as on a bus ride or the like), the GM should roll a die. On a roll of 4 or
higher, John goes to sleep and wakes up whenever (prompting a struggle for control of
John). When John naps like this, all of the Voices gain one Willpower.

## Playing

Play begins with John waking up in the morning (and a test for control of him). The GM
then describes the circumstances into which he wakes up. With John, you never can tell. He
could be in a gutter somewhere, or he could be in a palatial estate, or anywhere in
between. The Voices really have no idea how they ended up here.

At this point, the Voices should start working on fulfilling their obsessions and avoiding
getting John killed. Inevitably, their Willpowers will decrease. Once all of the Voices
are out of Willpower, John sinks back into sleep, and the game session is over.

At this point, the Voices reveal their obsessions and count up how many times each was
filled. Note that it counts for you if someone else is Active and fulfills your obsession.

Now, multiply the number of times you fulfilled your obsession by your obsession's rank
(1, 2, or 3). That's the number of points you have. Highest number of points wins the
game, and, generally speaking, is the GM of the next game of Everyone is John.

## Variations

EiJ is very flexible. Rules can be added, removed, or modified as necessary. For example,
instead of each voice bidding to take control of John, with the highest bidder winning,
the amount wagered could instead in a random weighted roll.

For example, with three voices who each wager 1, 2, and 3 willpower, a 1d6 is rolled. On
rolling 1, voice 1 wins. On rolling a 2 or 3, voice 2 wins. For rolling 4, 5, or 6, voice
3 wins.

This might be difficult to do when playing regularly in-person, but this tracker comes
with this function built in!
