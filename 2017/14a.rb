require_relative "../aoc"

test "flqrgnkx", 8108

def knot input
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

  dense_hash
end

input 2017, 14 do |input|
  key = input.chomp

  grid = 128.times.map do |row|
    row_key = "#{key}-#{row}"

    knot_hash = knot row_key

    knot_hash.map { |byte| byte.to_s 2 }.join
  end.join

  grid.count '1'
end
