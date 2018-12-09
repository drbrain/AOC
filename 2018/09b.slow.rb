require_relative "../aoc"
require "scanf"

test "9 players; last marble is worth 25 points", 22563

##
# This is not an efficient implementation because the Array is continually
# reallocated due to all the Array#rotate calls.
#
# The -9 rotation undoes the previous rotation, and that can probably be
# eliminated by moving the +2 rotation but I was too lazy.
#
# A better implementation might use an Array and track the index, but you
# would still pay the reallocation penalty on Array#insert and
# Array#delete_at.

input 2018, 9 do |input|
  players, last_marble =
    input.scanf "%d players; last marble is worth %d points"

  last_marble *= 100

  marbles = (1..last_marble).to_a

  scores = Hash.new 0
  circle = [0]

  (1..players).cycle do |player|
    marble = marbles.shift

    puts marble if (marble % 1000).zero?

    if (marble % 23).zero? then
      circle = circle.rotate -7

      taken = circle.shift
      score = marble + taken

      scores[player] += score
    else
      circle = circle.rotate 2

      circle.unshift marble
    end

    break if marbles.empty?
  end

  scores.values.max
end
