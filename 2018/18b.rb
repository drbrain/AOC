require_relative "../aoc"

#test <<-INPUT, 1147
#.#.#...|#.
#.....#|##|
#.|..|...#.
#..|#.....#
##.#|||#|#|
#...#.||...
#.|....|...
#||...#|.#|
#|.||||..|.
#...#.|..|.
#INPUT

class Map
  attr_accessor :map

  def initialize
    @map       = nil
    @max_index = nil
  end

  def adjacent row, col
    adjacent = []

    unless row.zero? then
      adjacent << @map[row - 1][col - 1] unless col.zero?
      adjacent << @map[row - 1][col]
      adjacent << @map[row - 1][col + 1]
    end

    adjacent << @map[row][col - 1] unless col.zero?
    adjacent << @map[row][col + 1]

    if row < @max_index then
      adjacent << @map[row + 1][col - 1] unless col.zero?
      adjacent << @map[row + 1][col + 1]
      adjacent << @map[row + 1][col]
    end

    adjacent
  end

  def hash
    @map.hash
  end

  def lumberyards
    @map.map { |tiles|
      tiles.count { |tile| 35 == tile }
    }.sum
  end

  def parse input
    @map = input.lines.map { |line|
      line.chomp.chars.map(&:ord)
    }

    @max_index = @map.count - 1
  end

  def step
    @map =
      @map.map.with_index { |tiles, row|
        tiles.map.with_index { |tile, col|
          around = adjacent row, col

          case tile
          when 46 then
            if around.count { |t| t == 124 } >= 3 then
              124
            else
              46
            end
          when 124 then
            if around.count { |t| t == 35 } >= 3 then
              35
            else
              124
            end
          when 35 then
            if around.any? { |t| t == 124 } and
                 around.any? { |t| t == 35 } then
              35
            else
              46
            end
          end
        }
      }
  end

  def score
    lumberyards * trees
  end

  def trees
    @map.map { |tiles|
      tiles.count { |tile| 124 == tile }
    }.sum
  end

  def to_s
    @map.map { |tiles|
      tiles.map(&:chr).join
    }.join "\n"
  end
end

##
# I spent a bunch of time prematurely optimizing this one before adding period
# detection like problem 12 has.
#
# Had I gone immediately to period detection I would have had an answer much
# more quickly.  The part A implementation is about 10 times slower than this
# one because of all the extra method calls and object creation and garbage
# collection it does, but the 30 seconds or so to wait should have been
# enough.
#
# Next time one of these automata problems shows up I'll remember this lesson.
#
# Converting characters to numbers is likely a useless optimization given only
# about 500 maps are generated.
#
# I really like how I reused Array#hash to get loop detection, it saved
# storing a whole map but there is a low possibility of a false match.

input 2018, 18 do |input|
  map = Map.new
  map.parse input

  # map hash => [sum, count]
  seen = {}

  index = 1_000_000_000.times do |i|
    break i if seen.include? map.hash

    seen[map.hash] = [i, map.score]

    map.step
  end

  loop_start, = seen[map.hash]

  period =
    (loop_start...index).map { |i|
      seen.values.find { |pattern_index,| pattern_index == i }.last
    }

  remaining = 1_000_000_000 - index

  offset = remaining % (index - loop_start)

  period[offset]
end
