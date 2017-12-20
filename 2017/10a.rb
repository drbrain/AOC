require_relative "../aoc"

test "3", 2
test "3,4,1,5", 12

input 2017, 10 do |input|
  lengths = input.split ","
  lengths = lengths.map { |length| Integer length }

  list_size = test_run? ? 5 : 256

  list = (0...list_size).to_a
  skip_size = 0
  start = 0

  lengths.each do |length|
    reversed = list.take(length).reverse
    rest = list[length, list_size - length]
    list = reversed.concat rest

    total_skip = skip_size + length
    start += total_skip
    skip_size += 1

    list = list.rotate total_skip
  end

  fix = list_size - (start % list_size)

  list.rotate(fix).first(2).reduce :*
end
