require_relative "../aoc"

test "aA", 0
test "abBA", 0
test "aabAAB", 6
test "dabAcCaCBAcCcaDA", 10

##
# I spent a bunch of time thinking about using Enumerable#each_cons but due to
# the need for backtracking I went with this weird loop.
#
# The loop looks at a pair of units from the polymer and stops if it runs out
# of items to make a pair.  Running out of units occurs when the loop reaches
# the end of the polymer.
#
# If there is no reaction the loop continues to the next pair.
#
# If there is a reaction we remove the current and next unit, since they
# reacted, and step back one place because we need to compare the previous
# unit with the unit after the one we deleted.
#
# Using +next+ is key to avoid ending up right where we left from the
# index-increment at the bottom of the loop.
#
# Since the loop mutates the polymer we can ask its size after reacting all
# units.
#
# Determining if two units reacted took some revision.  At first I wrote:
#
#   a == b.upcase || a == b.downcase
#
# which doesn't work because it evaluates to true for two units of the same
# case.
#
# Unfortunately ruby doesn't have any methods to test the case of a character
# so the test I ended up with feels unsatisfactory.  Adding methods like
# +uppercase?+ and +lowercase?+ would help for real use.

input 2018, 5 do |input|
  polymer = input.strip.chars

  i = 0

  loop do
    break unless a = polymer[i]
    break unless b = polymer[i + 1]

    if (("a".."z").include?(a) and a.upcase == b) or
       (("A".."Z").include?(a) and a.downcase == b) then
      polymer.delete_at i
      polymer.delete_at i

      i -= 1

      next
    end

    i += 1
  end

  polymer.size
end
