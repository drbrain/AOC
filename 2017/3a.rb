require_relative "../aoc"

test "1",     0
test "12",    3
test "13",    4
test "14",    3
test "15",    2
test "16",    3
test "17",    4
test "23",    2
test "26",    5
test "1024", 31

input 2017, 3 do |location|
  location = Integer location

  next 0 if location == 1

  ring = Integer Math.sqrt location
  ring -= 1 if ring.even?

  edge_length = ring + 1
  total_offset = location - ring ** 2

  _, edge_offset = total_offset.divmod edge_length

  center = edge_length / 2

  offcenter = edge_offset - center
  inward = ring / 2 + 1

  offcenter.abs + inward
end
