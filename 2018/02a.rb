require_relative "../aoc"

def count id, recurrences
  seen = Hash.new 0

  id.chars.each do |char|
    seen[char] += 1
  end

  seen.count { |id, times| times == recurrences }.nonzero?
end

test <<-INPUT, 12
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
INPUT

input 2018, 2 do |ids|
  twos   = 0
  threes = 0

  ids.lines.each do |id|
    id.strip!

    twos   += 1 if count id, 2
    threes += 1 if count id, 3
  end

  twos * threes
end
