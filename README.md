# Automata Survey

This is a small Rust program that will generate GIFs with the results
of running random 1D (2D images) or 2D  (3D images/movies) binary automata
with variable numbers of *neighbors*.

For a 1D automata with 1 neighbor (total: 3 cells), there are 2^3 possible
state transitions (e.g. 000->a; 001->b; etc.; 111->h) for the automata.
So the *family* of 1D automata with 1 neighbor is comprised
of 2^8 (or 2^(2^**3**)) automata (for each possible binary value of
*hgfedcba*).  Each individual automata in the family can be described
by an 8-bit number (e.g. Rule 217).  For a 1D automata with 2 neighbors
(total: 5 cells), the *family* is comprised of 2^32 (or 2^(2^**5**))
automata.  Likewise, each individual automata in the family can be
described by a 32-bit number.

For a 2D automata with 1 neighbor (total: 9 cells; think *Conway's Game
of Life*), there are 2^9 possible state transitions (exercise for the
reader...).  So the *family* has 2^512 (or 2^(2^9)) members (with one of
them being the *Game of Life*).  Each individual automata in the family
can be described by a 512-bit number.

## Output

Goes in *./output/$(dimensions)/${neighbors}/${rule}.....gif

## Notes & ToDos

* Woefully incomplete...
* This has been developed only for amd64 on Debian (so Ubuntu should work).
* Suitable Intel MKL and SSL libraries and dev headers are required.
  Like so: `sudo apt install libmkl-dev libssl-dev`.
* TODO: Implement 2D automata.  Only 1D automata right now...
* TODO: Fix 1D automata logic to replicate Wolfram results.
* TODO: Properly calculate the *Interesting* properties for Rules and Automata.
  These will be necessary to avoid calculating or writing non-Interesting
  Automata.
* TODO: Calculate invariant rules (e.g. rotations, mirrors, etc) so that
  calculation can be avoided (e.g. 1Dx1: if a Rule with 001->a; etc; then a Rule
  with 100->a (mirror); etc; is identical).  This is straightforward for 1D
  automata but might not be for 2D (balancing reducing calculation against
  ignoring possibly missing automata).
