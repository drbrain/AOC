require_relative "../aoc"

test "0\t2\t7\t0", 4

input 2017, 6 do |input|
  count = 0
  seen  = Hash.new { |h, layout| h[layout] = 0 }

  banks = input.split "\t"
  banks = banks.map { |blocks| Integer blocks }
  max_banks = banks.size

  until seen.include? banks do
    seen[banks] = count

    max_bank = banks.index banks.max
    redistribute, banks[max_bank] = banks[max_bank], 0

    start_bank = max_bank + 1

    redistribute.times do |index|
      offset = start_bank + index
      offset %= max_banks

      banks[offset] += 1
    end

    count += 1
  end

  count - seen[banks]
end
