require_relative "../aoc"

input 2017, 23 do |input|
  registers = Hash.new 0
  registers["a"] = 1
  pc = 0

  program = input.lines.map.with_index { |line, i|
    puts "%2d %s" % [i, line]
    name, x, y = line.split

    case name
    when "set" then
      if /[a-z]/ =~ y
        -> { registers[x] = registers[y]; pc += 1 }
      else
        y = Integer y
        -> { registers[x] = y; pc += 1 }
      end
    when "sub" then
      if /[a-z]/ =~ y
        -> { registers[x] -= registers[y]; pc += 1 }
      else
        y = Integer y
        -> { registers[x] -= y; pc += 1 }
      end
    when "mul" then
      if /[a-z]/ =~ y
        -> { registers[x] *= registers[y]; pc += 1 }
      else
        y = Integer y
        -> { registers[x] *= y; pc += 1 }
      end
    when "jnz" then
      y = Integer y

      if "1" == x then
        -> { pc += y }
      else
        -> {
          unless registers[x].zero? then
            pc += y
          else
            pc += 1
          end
        }
      end
    end
  }

  loop do
    puts registers if pc == 23

    break unless instruction = program[pc]

    instruction.call
  end

  registers[h]
end
