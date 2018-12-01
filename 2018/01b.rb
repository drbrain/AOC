require_relative "../aoc"

test "+1 -2 +3 +1", 2
test "+1 -1", 0
test "+3 +3 +4 -2 -4", 10
test "-6 +3 +8 +5 -6", 5
test "+7 +7 -2 -7 -4", 14

input 2018, 1 do |input|
  frequencies = input.split /\s/
  frequencies.map! { |input| Integer input }

  current       = 0
  seen          = {}
  seen[current] = true

  frequencies.cycle do |frequency|
    current += frequency

    break current if seen.include? current

    seen[current] = true
  end
end
