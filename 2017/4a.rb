require_relative "../aoc"

test "aa bb cc dd ee", 1
test "aa bb cc dd aa", 0
test "aa bb cc dd aaa", 1

input 2017, 4 do |list|
  list.lines.count { |passphrase|
    words = passphrase.split
    words.size == words.uniq.size
  }
end
