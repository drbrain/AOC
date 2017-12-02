require_relative "../aoc"

test "1122", 3
test "1111", 4
test "1234", 0
test "91212129", 9

input_part_1 2017, 1 do |input|
  values = input.strip.each_char.map { |char|
    Integer char
  }

  result = values.each_cons(2).select { |a, b|
    a == b
  }.map(&:first).sum

  result += values.first if values.first == values.last

  result
end

test "1212",      6, part: 2
test "1221",      0, part: 2
test "123425",    4, part: 2
test "123123",   12, part: 2
test "12131415",  4, part: 2

input_part_2 2017, 1 do |input|
  values = input.strip.each_char.map { |char|
    Integer char
  }

  offset = values.length / 2
  extended = values.cycle(2).to_a

  result = values.select.with_index { |v, i|
    v == extended[i + offset]
  }.sum

  result
end

