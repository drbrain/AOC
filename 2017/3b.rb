require_relative "../aoc"

def coordinates location
  return [0, 0] if location == 1

  ring = Integer Math.sqrt location
  ring -= 1 if ring.even?

  edge_length = ring + 1
  total_offset = location - ring ** 2

  _, edge_offset = total_offset.divmod edge_length

  center = edge_length / 2

  offcenter = edge_offset - center
  outward = ring / 2 + 1

  [offcenter, outward]
end

$values = {}
$values[1] = 1

def value_at location
  if cached = $values[location] then
    return cached
  end

  offcenter, outward = coordinates location
end

test "1", 2
test "2", 4
test "4", 5
test "5", 10

input 2017, 3 do |max|
  max = Integer max

  (1...Infinity).each { |location|
    value = value_at location

    break value if value > max
  }
end
