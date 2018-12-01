require_relative "../aoc"

test "+1 +1 +1", 3
test "+1 +1 -2", 0
test "-1 -2 -3", -6

input 2018, 1 do |input|
  ruby = input.tr "\n", " "
  eval "0 #{ruby}"
end
