require_relative "../aoc"

test "0\n3\n0\n1\n-3", 5

input 2017, 5 do |input|
  jumps = input.split "\n"
  jumps = jumps.map { |jump| Integer jump }

  pc = 0
  steps = 0

  while pc < jumps.size do
    movement = jumps[pc]
    jumps[pc] += 1
    pc += movement
    steps += 1
  end

  steps
end
