---
title: Brainstorming a Ranking Method
published: 2024-04-14
tagline: for naturally organizing media collections
tags:
  - software
  - brainstorming
---

I like to catalogue and rate/rank the media I consume. Movies, games, books,
podcasts, whatever. There are some really good sites out there for _most_ of
these things, like Letterboxd or Backloggd. That's where most of my ratings
live. Couldn't find a good site for podcasts, so I made a
massively-overengineered spreadsheet. I found that spreadsheet fun to make and
it was really nice to have any feature I could dream up. So now I'm considering
overengineering all of it together into one big thing.

This has also been partially spurred with my growing dissatisfaction with
assigning number ratings to things directly. It feels pretty hard to stay
consistent over time, especially if my thoughts on something change over time. A
game that I played a decade ago might have been a 9/10 to me back then, but then
I'll play something so much better that it redefines what I consider a 10/10,
and my whole scale has to shift to compensate. I'll also find myself comparing
items of similar scores and tweak things up or down. I find myself relying on
these comparisons more and more.

So I thought, why not switch to an entirely comparison-based system, and
optionally derive scores afterwards?

If I want scores afterwards, I can assign score bands to the output ordering, or
apply some ordering->score function.

There's just one problem. I can't find anyone that's done this before. Is it
even a good idea? Who knows! Time to get tinkering.

My requirements (to start):

- compare entries in pairs to generate an ordering (or at least tiers)
- support partial information (increasing accuracy with more comparisons)
- support ties in comparisons
- support possibly conflicting data/graph cycles (A > B > C > A)
- allow "re-ranking" entries (wiping all previous comparisons involving that entry)
- order comparisons were made in should not matter significantly
- able to select yet-uncompared entries that would provide the most information

My first thought was to apply a competitive game rank algorithm, like Elo,
Glicko, or TrueSkill. They're purpose-built for this kind of thing: taking
comparisons (game outcomes) as inputs and providing scores as outputs. The more
advanced algorithms even have fancy features like accounting for changes in
variance over time. Unfortunately, I don't think this is an ideal fit, but maybe
a good backup. These algorithms are strongly effected by the order comparisons
were made. Additionally, there is no real way to nullify previous comparisons.
Even if I kept a running database of all previous comparisons to re-run the
algorithm every time, the ordering issue mentioned above would mean results
would shift more than expected.

My next thought is that this could be represented as a bidirectional graph.
Nodes of the graph are entries, and edges are comparisons between them, with the
directionality indicating comparison results. I could then apply a topological
sort to derive an ordering from the graph. This sounds great, and I find graph
theory fun. I could easily add/remove edges to get a new graph, and the order
edges were added would be (nearly) irrelevant. Additionally, it's a data
structure that could be easily fit into a database. Unfortunately, the graph
would potentially have cycles/contradictions, which violates the base
assumptions of most topological sort algorithms I'm aware of.

A common solution to this contradiction/cycle problem is to apply a Minimum
Feedback Arc Set, which can be used to remove all contradictions/cycles from a
graph, and then apply a topological sort. Unfortunately, this does mean data is
discarded. It may be better to disallow contradictions in the first place.

I may try to come up with some cursed custom alternative, and I'll keep
searching for other options. I may also end up removing 1-2 of my requirements
(like allowing contradictions/cycles).

Thought it would be fun to put these thoughts out there.
