require_relative "../aoc"

#test "18", "90,269,16"
#test "42", "232,251,12"

def power_level serial_number, x, y
  rack_id = x + 10
  ((((rack_id * y) + serial_number) * rack_id) % 1000 / 100) - 5
end

def each_square_of_size grid_dimension, square_dimension
  return enum_for __method__, grid_dimension, square_dimension unless
    block_given?

  # origin points for each sub-square
  (0...grid_dimension).each do |x|
    (0...grid_dimension).each do |y|

      coordinates = (x + 1..x + square_dimension).flat_map { |x_offset|
        (y + 1..y + square_dimension).map { |y_offset|
          next if x_offset > grid_dimension
          next if y_offset > grid_dimension
          [x_offset, y_offset]
        }
      }.compact

      yield coordinates
    end
  end
end

#pp each_sub_square(3).to_a

##
# I implemented #each_square better with in #each_square_of_size so it
# automatically trims the square to the correct dimensions.
#
# Originally #each_square_of_size was an O(N³) implementation that would yield
# the dimension of the square and the coordinates of the entries in it.  I
# switched the implementation to one that better-resembles part A after
# reading a hint that implied I should print the state of my calculation by
# dimension.
#
# The maximum size trends up then back down so I was able to use a local
# maximum as a (correct) guess at the answer.
#
# Before doing that I tried several optimizations to reduce the calculating
# required including calculating row sums for a dimension to cache work in
# advance.  This was promising, but printing work-in-progress was better.
#
# Reading other hints, taking time to analyze intermediate products should
# help me come up with better solutions.  I likely would have seen a large
# field of negative values and would have guessed that larger dimensions would
# be unlikely to be maximums.

input 2018, 11 do |input|
  serial_number = Integer input

  grid = {}

  (1..300).each do |x|
    (1..300).each do |y|
      grid[[x, y]] = power_level serial_number, x, y
    end
  end

  (1..300).each do |dimension|
    result = each_square_of_size(300, dimension).map { |coordinates|
      sum = coordinates.map { |coordinate|
        grid[coordinate]
      }.sum

      [sum, coordinates]
    }

    max, (c,) = result.max_by(&:first)

    p max => [c, dimension].flatten.join(',')
  end
end
