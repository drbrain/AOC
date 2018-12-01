require_relative "../aoc"

test "3", 638

input 2017, 17 do |input|
  steps = Integer input.chomp

  buffer = [0]

  1.upto 2017 do |year|
    buffer.rotate! steps

    buffer[1, 0] = year

    buffer.rotate! 1
  end

  buffer[1]
end
