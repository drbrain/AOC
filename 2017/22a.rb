require_relative "../aoc"

test <<-GRID, 5587
..#
#..
...
GRID

def turn direction, offset
  directions = [:up, :left, :down, :right]
  index = directions.index direction

  directions.cycle(2).to_a[index + offset]
end

def left direction
  turn direction, 1
end

def right direction
  turn direction, -1
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
  grid = {}

  input.each_line.with_index do |line, row|
    line.chomp.each_char.with_index do |char, col|
      next if char == "."
      grid[[row, col]] = true
    end
  end

  middle = input.lines.size / 2

  carrier_pos = [middle, middle]
  carrier_dir = :up

  infections = 0

  10_000.times do
    infected = grid[carrier_pos]

    infections += 1 unless infected

    grid[carrier_pos] = !infected

    carrier_dir =
      if infected then
        right carrier_dir
      else
        left carrier_dir
      end

    carrier_pos = forward carrier_pos, carrier_dir
  end

  infections
end
