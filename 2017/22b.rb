require_relative "../aoc"

test <<-GRID, 2511944
..#
#..
...
GRID

DIRECTIONS = [:up, :left, :down, :right].cycle(2).to_a
def turn direction, offset
  index = DIRECTIONS.index direction

  DIRECTIONS[index + offset]
end

def left direction
  turn direction, 1
end

def right direction
  turn direction, -1
end

def reverse direction
  turn direction, 2
end

def forward position, direction
  row, col = position

  case direction
  when :down  then row += 1
  when :left  then col -= 1
  when :right then col += 1
  when :up    then row -= 1
  end

  [row, col]
end

input 2017, 22 do |input|
  grid = Hash.new 0

  input.each_line.with_index do |line, row|
    line.chomp.each_char.with_index do |char, col|
      next if char == "."

      grid[[row, col]] = 2
    end
  end

  middle = input.lines.size / 2

  carrier_pos = [middle, middle]
  carrier_dir = :up

  10_000_000.times.count do
    state = grid[carrier_pos]

    carrier_dir =
      case state
      when 0 then left    carrier_dir
      when 1 then         carrier_dir
      when 2 then right   carrier_dir
      when 3 then reverse carrier_dir
      end

    grid[carrier_pos] = (state + 1) % 4

    carrier_pos = forward carrier_pos, carrier_dir

    state == 1
  end
end
