require_relative "../aoc"

test "1,2,3",    "3efbe78a8d82f29979031a4aa0b16a9d"
test "",         "a2582a3a0e66e6e86e3812dcb672a272"
test "AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"
test "1,2,4",    "63960835bcdc130f0b66d7ff4f6a5a8e"

input 2017, 10 do |input|
  lengths = input.chomp.unpack "c*"
  lengths.concat [17, 31, 73, 47, 23]

  list_size = 256

  list = (0...list_size).to_a
  skip_size = 0
  start = 0

  64.times do
    lengths.each do |length|
      reversed = list.take(length).reverse
      rest = list[length, list_size - length]
      list = reversed.concat rest

      total_skip = skip_size + length
      start += total_skip
      skip_size += 1

      list = list.rotate total_skip
    end
  end

  fix = list_size - (start % list_size)

  sparse_hash = list.rotate fix

  dense_hash = sparse_hash.each_slice(16).map do |block|
    block.reduce :^
  end

  "%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x" % dense_hash
end
