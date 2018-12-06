require_relative "../aoc"
require "scanf"

#test <<-INPUT, 16
#1, 1
#1, 6
#8, 3
#3, 4
#5, 5
#8, 9
#INPUT

##
# The test and implementation use different constants (32 vs 10,000) for the
# region distance boundary, so the test above is commented out.
#
# This solution would benefit from a method that enumerated all the
# coordinates within a bounding box as then it could use Enumerable#count to
# get all the points are in the region, something like:
#
#   coordinates_within(min_x: min_x, min_y: min_y,
#                      max_x: max_x, max_y: max_y).count { |x, y|
#     coordinates.map { |cx, cy|
#       (x - cx).abs + (y - cy).abs
#     }.sum < 10_000
#   }

input 2018, 6 do |input|
  coordinates = input.lines.map { |line|
    line.scanf "%d, %d"
  }

  region_size = 0

  min_x, max_x = coordinates.minmax_by { |x,| x }.map(&:first)
  min_y, max_y = coordinates.minmax_by { |_, y| y }.map(&:last)

  (min_x..max_x).each do |x|
    (min_y..max_y).each do |y|
      distances = coordinates.map { |cx, cy|
        (x - cx).abs + (y - cy).abs
      }

      region_size += 1 if distances.sum < 10000
    end
  end

  region_size
end
