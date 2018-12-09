require_relative "../aoc"
require "scanf"

test "9 players; last marble is worth 25 points", 22563

class Circle
  attr_accessor :fwd
  attr_accessor :rev
  attr_reader :value

  def self.initialize
    node = new 0, nil, nil
    node.fwd = node
    node.rev = node

    node
  end

  def initialize value, rev, fwd
    @value = value
    @rev   = rev
    @fwd   = fwd
  end

  def ==(other)
    other.class === self and
      other.value == @value
  end

  def forward nodes
    node = self

    nodes.times do
      node = node.fwd
    end

    node
  end

  def insert value
    node = self.class.new value, self, @fwd

    @rev = node if @fwd == @rev # self-linked
    @fwd.rev = node
    @fwd     = node

    node
  end

  def inspect
    "[#{@rev.value} -> #{@value} <- #{@fwd.value}]"
  end

  def to_s
    to_s = "(#{@value}) "
    node = @fwd

    until node == self do
      to_s << "#{node.value} "

      node = node.fwd
    end

    to_s.rstrip
  end

  def reverse nodes
    node = self

    nodes.times do
      node = node.rev
    end

    node
  end

  def take
    node = @fwd

    @rev.fwd, @fwd.rev = @fwd, @rev

    @fwd = nil
    @rev = nil

    return node, @value
  end
end

##
# After starting up the part A solution with the <code>last_marble *=
# 100</code> line I realized it was going to take a long time to complete.
#
# I left that running and started exploring if there was some pattern I could
# use to directly compute the score given the input, but this was beyond my
# math skills.
#
# After reading the Advent of Code reddit I noticed people suggest a
# linked-list.
#
# Since I needed to rotate both forward and backward I implemented a
# doubly-linked list and had a bunch of problems getting insert and take
# implemented correctly.
#
# Ruby's multiple-assignment swap feature came in handy in Circle#take,
# though.
#
# Initially some of my struggle was due to forgetting to re-assign the circle
# variable as I rotated it around.  I also didn't catch that I only needed to
# go "forward" one place, not two, to insert the new marble in the correct
# place.
#
# When I need to use two iterators next time I should iterate on the bounded
# one (marbles) and use Enumerator#next for the other.  It will let me avoid
# code like the <code>break if marbles.empty?</code>

input 2018, 9 do |input|
  players, last_marble =
    input.scanf "%d players; last marble is worth %d points"

  last_marble *= 100

  marbles = (1..last_marble).to_a

  scores = Hash.new 0
  circle = Circle.initialize

  (1..players).cycle do |player|
    marble = marbles.shift

    puts marble if (marble % 1000).zero?

    if (marble % 23).zero? then
      circle = circle.reverse 7

      circle, taken = circle.take

      score = marble + taken

      scores[player] += score
    else
      circle = circle.forward 1

      circle = circle.insert marble
    end

    break if marbles.empty?
  end

  scores.values.max
end
