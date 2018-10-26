# Cardchoose algorithm

Algorithm | Python | C++ | Order guarantee | Data structures | Time
----|----|----|----|----|----
"[cardchoose](cardchoose.md)" | [Python](python/algorithms/cardchoose.py) | [C++](cpp/cardchoose.cpp) | Sorted | none | _k_ log _k_

Cardchoose is my new algorithm for sampling without replacement efficiently.

With this approach, you don't need any extra data structures or
storage beyond the list that you'll return, and the steps that fill the list
are simple and fast with no loops; everything except the sort is O(k).

Intuition: supposing you have (n=7, k=3). So you have three red balls out of
seven balls total; let's say the other four balls are white. Start with the four
white balls in a row

     O O O O
    d = []

We want to add a red ball; there are five places it can go

     O O O O
    0 1 2 3 4
    d = []

Supposing we pick 1, then we get

     O x O O O
    d = [1]

Now there are six places the next ball can go. However, instead of numbering
them sequentially, we number them like this:

     O x O O O
    0 1 5 2 3 4
    d = [1]

Instead of writing numbers on the slots, we write numbers on the balls. We write
1, 2, 3, 4 on the white balls, and 5, 6, 7 on the red balls. Each slot gets the
number of the ball to its left, while the leftmost slot gets 0.

We use this numbering to choose where to put the next ball. But we don't care
about the order among the red balls, so in `d` we just record the number of
white balls to the left of each new red ball. Now add another red ball, this
time in slot 3:

     O x O O x O
    0 1 5 2 3 6 4
    d = [1, 3]

And another, this time in slot 5, so we look up `d[0]` to see how many white
balls are to its left:

     O x x O O x O
    0 1 5 7 2 3 6 4
    d = [1, 3, 1]

It helps me to imagine that instead of red balls we have thin red cards.
Initially there's just one place to drop a card between two white balls, but
once we drop a card in there, there are now two places to drop a second card,
and so on.

Once we've dropped in all the cards, we pour water on them, and they swell into
balls, pushing along everything to their right to make space for themselves. So
each red ball moves along by as many positions as it has red balls to its left;
that's what the sort and the last line does.

    d = [1, 3, 1]
    d = [1, 1, 3] # d.sort()
    d = [1, 2, 5] # for i in range(k): d[i] += i

     O x x O O x O
     0 1 2 3 4 5 6
