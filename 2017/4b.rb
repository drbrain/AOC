require_relative "../aoc"

test "abcde fghij", 1
test "abcde xyz ecdab", 0
test "a ab abc abd abf abj", 1
test "iii oiii ooii oooi ooo", 1
test "oiii ioii iioi iiio", 0

input 2017, 4 do |list|
  list.lines.count { |passphrase|
    words = passphrase.split.map { |word|
      word.chars.sort.join
    }

    words.size == words.uniq.size
  }
end

