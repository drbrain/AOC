require_relative "../aoc"

test <<INSTRUCTIONS, 1
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
INSTRUCTIONS

input 2017, 8 do |input|
  registers = Hash.new 0

  input.lines.each do |line|
    /(\S+) (inc|dec) (\S+) if (\S+) (\S+) (\S+)/ =~ line

    register, operation, amount, condition_register, condition, value =
      $1, $2, Integer($3), $4, $5, Integer($6)

    operation =
      if "inc" == operation then
        :+
      else
        :-
      end

    result = registers[condition_register].send condition, value

    registers[register] = registers[register].send operation, amount if result
  end

  registers.values.max
end
