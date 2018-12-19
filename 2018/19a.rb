require_relative "../aoc"
require "scanf"

test <<-INPUT, 6
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
INPUT

class InvalidRegister < RuntimeError
end

class Registers
  def initialize values
    @values = values
  end

  def [] index
    value = @values[index]

    raise InvalidRegister, index unless value

    value
  end

  def []= index, value
    raise InvalidRegister, index unless @values[index]

    @values[index] = value
  end

  def to_s
    @values.to_s
  end

  alias inspect to_s
end

class Instruction
  attr_reader :a
  attr_reader :b
  attr_reader :c

  def initialize a, b, c, registers
    @a = a
    @b = b
    @c = c

    @registers = registers
  end

  def to_s
    "#{self.class.name.downcase} #@a #@b #@c"
  end

  def inspect
    "[#{to_s}]"
  end
end

class ADDR < Instruction
  def execute
    @registers[@a] + @registers[@b]
  end
end

class ADDI < Instruction
  def execute
    @registers[@a] + b
  end
end

class MULR < Instruction
  def execute
    @registers[@a] * @registers[@b]
  end
end

class MULI < Instruction
  def execute
    @registers[@a] * b
  end
end

class BANR < Instruction
  def execute
    @registers[@a] & @registers[@b]
  end
end

class BANI < Instruction
  def execute
    @registers[@a] & b
  end
end

class BORR < Instruction
  def execute
    @registers[@a] | @registers[@b]
  end
end

class BORI < Instruction
  def execute
    @registers[@a] | b
  end
end

class SETR < Instruction
  def execute
    @registers[@a]
  end
end

class SETI < Instruction
  def execute
    a
  end
end

class GTIR < Instruction
  def execute
    if a > @registers[@b] then
      1
    else
      0
    end
  end
end

class GTRI < Instruction
  def execute
    if @registers[@a] > b then
      1
    else
      0
    end
  end
end

class GTRR < Instruction
  def execute
    if @registers[@a] > @registers[@b] then
      1
    else
      0
    end
  end
end

class EQIR < Instruction
  def execute
    if a == @registers[@b] then
      1
    else
      0
    end
  end
end

class EQRI < Instruction
  def execute
    if @registers[@a] == b then
      1
    else
      0
    end
  end
end

class EQRR < Instruction
  def execute
    if @registers[@a] == @registers[@b] then
      1
    else
      0
    end
  end
end

class Machine
  attr_reader :instruction_pointer

  def initialize
    @instruction         = []
    @bind_ip             = nil
    @registers           = Registers.new [0, 0, 0, 0, 0, 0]
    @instruction_pointer = 0
  end

  def [] register_index
    @registers[register_index]
  end

  def []= register_index, value
    @registers[register_index] = value

    @instruction_pointer = value if register_index == @bind_ip
  end

  def next_instruction
    @instructions[@instruction_pointer]
  end

  def parse input
    @instructions =
      input.lines.map { |line|
        case line
        when /^#ip/ then
          @bind_ip, = line.scanf "#ip %d"

          next
        else
          name, a, b, c = line.scanf "%s %d %d %d"

          instruction_class = Object.const_get name.upcase

          instruction_class.new a, b, c, @registers
        end
      }.compact
  end

  def run
    instruction = next_instruction

    while instruction do
      self[@bind_ip] = @instruction_pointer

      self[instruction.c] = instruction.execute

      @instruction_pointer += 1

      instruction = next_instruction
    end

    @instruction_pointer -= 1
  end

  def to_s
    "ip = #{@instruction_pointer} #{@registers.inspect} #{next_instruction}"
  end

  alias inspect to_s
end

##
# I borrowed heavily from 16b.rb, but rewrote parsing and changed Opcode to
# Instruction which carries its arguments with it.
#
# The first part explaining instruction pointer binding, "When the instruction
# pointer is bound to a register, its value is written to that register just
# before each instruction is executed", was confusing and I updated the bound
# register when setting the register, but that didn't work.

input 2018, 19 do |input|
  machine = Machine.new
  machine.parse input
  machine.run

  puts machine

  machine[0]
end
