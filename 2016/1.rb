require_relative "../aoc"

directions = %i[north east south west]
x, y = 0, 0
direction = 0

input 2016, 1, split: "," do |input|
  /(?<turn>.)(?<blocks>.+)/ =~ input

  blocks = Integer blocks

  direction +=
    if turn == "L" then
      -1
    else
      1
    end

  direction %= 4

  case direction
  when 0 then
    x += blocks
  when 1 then
    y += blocks
  when 2 then
    x -= blocks
  else
    y -= blocks
  end
end

puts x.abs + y.abs
