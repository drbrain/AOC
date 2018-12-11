require_relative "../aoc"

test "18", "33,45"
test "42", "21,61"

def power_level serial_number, x, y
  rack_id = x + 10
  power_level = rack_id * y
  power_level += serial_number
  power_level *= rack_id
  power_level = power_level % 1000 / 100

  power_level - 5
end

#p power_level(8, 3, 5)
#p power_level(57, 122, 79)
#p power_level(39, 217, 196)
#p power_level(71, 101, 153)

def each_square grid
  return enum_for __method__, grid unless block_given?

  height = grid.size
  width = grid.first.size

  (0...width).each do |x|
    (0...height).each do |y|
      row_1 = grid[x]
      square = row_1[y, 3]

      row_2 = grid[x + 1]
      square.concat row_2[y, 3] if row_2

      row_3 = grid[x + 2]
      square.concat row_3[y, 3] if row_3

      yield [x + 1, y + 1], square.compact
    end
  end
end

##
# I spent a bunch of time on #each_square so I could enumerate the squares and
# use Enumerable#max_by which would make calculating the solution easier to
# reason about by separating where square generating and square sum and
# maximum calculation occur.
#
# I'm not very happy that #each square doesn't calculate which rows to use
# from the dimension and instead has the +nil+ (<code>+f row_N</code>) checks.

input 2018, 11 do |input|
  serial_number = Integer input

  grid =
    (1..300).map { |x|
      (1..300).map { |y|
        power_level serial_number, x, y
      }
    }

  coordinates, = each_square(grid).max_by { |(x, y), square|
    square.sum
  }

  coordinates.join ","
end
