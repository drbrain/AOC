require_relative "../aoc"
require "scanf"

test <<-INPUT, 17
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
INPUT

##
# The primary frustration was the bugs in my data set:
#
# "Hey #AdventOfCode day 6 solvers! There was a bug in the answers for part 2
# for 33% of users.  It's fixed now; please re-submit your answer.  I'm super
# sorry about anyone that was affected! We're changing the betatesting
# procedures to catch this sort of thing in future puzzles."
# — https://twitter.com/ericwastl/status/1070563771339411457
#
# "Okay, part 1 issues fixed as well.  (20% of users had a faulty part 1
# answer.)  So sorry again!"
# — https://twitter.com/ericwastl/status/1070568609188143104
#
# After some thinking I decided that only calculating within the bounds of
# coordinates in my data set should be enough to determine if a region was
# infinite or not.  (Due to the bug in my data set I later tried with larger
# regions and still got the same sizes for my finite regions.)
#
# Before I was sure there was a bug my two stumbling blocks were getting the
# distances calculated correctly and excluding points that were equidistant to
# two coordinates.
#
# For the first stumbling block I started with a single chained method to
# calculate both the distance and carry the index, but wasn't able to
# understand keeping the values in the right place.  I broke it apart into the
# <code>.map.with_index</code> and the separate +sort_by+.
#
# For the second stumbling block I have the clunky line that calculates
# +closest_distance+ and +second_closest_distance+.
#
# Having a library with Coordinate objects would clean up a lot of the
# messiness.
#
# After verifying my areas matched what the test data set from the text should
# produce I spent a bunch of time on Reddit and Twitter, including running a
# python solution that produced an identical result to my solution.
# Eventually I submitted my second-largest reason which was accepted.

input 2018, 6 do |input|
  coordinates = input.lines.map { |line|
    line.scanf "%d, %d"
  }

  # coordinate_index => size in squares
  region_sizes = Hash.new 0
  # coordinate_index => borders infinity
  infinite = {}

  min_x, max_x = coordinates.minmax_by { |x,| x }.map(&:first)
  min_y, max_y = coordinates.minmax_by { |_, y| y }.map(&:last)

  (min_x..max_x).each do |x|
    (min_y..max_y).each do |y|
      distances = coordinates.map.with_index { |(cx, cy), i|
        [(x - cx).abs + (y - cy).abs, i]
      }

      closest = distances.sort_by { |distance,|
        distance
      }

      closest_distance, second_closest_distance =
        closest.first(2).map(&:first)

      next if closest_distance == second_closest_distance

      index = closest.first.last

      next if infinite[index]

      region_sizes[index] += 1

      if x == min_x or x == max_x or
         y == min_y or y == max_y then
        infinite[index] = true
      end
    end
  end

  region_sizes.reject { |index,|
    infinite[index]
  }.max_by { |_, size|
    size
  }.last
end
