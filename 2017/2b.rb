require_relative "../aoc"

test <<-TEST, 9, part: 2
5 9 2 8
9 4 7 3
3 8 6 5
TEST

input_part_2 2017, 2 do |input|
  input.lines.map { |line|
    row = line.strip.split(" ").map { |entry| Integer entry }

    row.flat_map.with_index { |a, i|
      row.select { |b|
        next if a == b
        result, remainder = a.divmod b
        next unless remainder.zero?
        result
      }.compact.first
    }.compact.first
  }.sum
end

