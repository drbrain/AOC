require_relative "../aoc"

test <<-TEST, 18
5 1 9 5
7 5 3
2 4 6 8
TEST

input_part_1 2017, 2 do |input|
  input.lines.map { |line|
    row = line.strip.split(" ").map { |entry| Integer entry }
    min, max = row.minmax
    max - min
  }.sum
end

