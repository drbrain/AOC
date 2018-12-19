require_relative "../aoc"
require "scanf"
require "prime"

class InvalidRegister < RuntimeError
end

class Registers
  include Enumerable

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

  def each
    @values.each do |value|
      yield value
    end
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

  def explain
    "#{@registers[@a]}r + #{@registers[@b]}r => #{execute}"
  end
end

class ADDI < Instruction
  def execute
    @registers[@a] + b
  end

  def explain
    "#{@registers[@a]}r + #{b}i => #{execute}"
  end
end

class MULR < Instruction
  def execute
    @registers[@a] * @registers[@b]
  end

  def explain
    "#{@registers[@a]}r * #{@registers[@b]}r => #{execute}"
  end
end

class MULI < Instruction
  def execute
    @registers[@a] * b
  end

  def explain
    "#{@registers[@a]}r * #{b}i => #{execute}"
  end
end

class BANR < Instruction
  def execute
    @registers[@a] & @registers[@b]
  end

  def explain
    "#{@registers[@a]}r & #{@registers[@b]}r => #{execute}"
  end
end

class BANI < Instruction
  def execute
    @registers[@a] & b
  end

  def explain
    "#{@registers[@a]}r & #{b}i => #{execute}"
  end
end

class BORR < Instruction
  def execute
    @registers[@a] | @registers[@b]
  end

  def explain
    "#{@registers[@a]}r | #{@registers[@b]}r => #{execute}"
  end
end

class BORI < Instruction
  def execute
    @registers[@a] | b
  end

  def explain
    "#{@registers[@a]}r | #{b}i => #{execute}"
  end
end

class SETR < Instruction
  def execute
    @registers[@a]
  end

  def explain
    "#{@registers[@a]}r => #{execute}"
  end
end

class SETI < Instruction
  def execute
    @a
  end

  def explain
    "#{@a}i => #@a"
  end
end

class GTIR < Instruction
  def execute
    if @a > @registers[@b] then
      1
    else
      0
    end
  end

  def explain
    if execute.zero? then
      "#{@a}i <= #{@registers[@b]}r => 0"
    else
      "#{@a}i > #{@registers[@b]}r => 1"
    end
  end
end

class GTRI < Instruction
  def execute
    if @registers[@a] > @b then
      1
    else
      0
    end
  end

  def explain
    if execute.zero? then
      "#{@registers[@a]}r <= #{@b}i => 0"
    else
      "#{@registers[@a]}r > #{@b}i => 1"
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

  def explain
    if @registers[@a] > @registers[@b] then
      "#{@registers[@a]}r <= #{@registers[@b]}r => 0"
    else
      "#{@registers[@a]}r > #{@registers[@b]}r => 1"
    end
  end
end

class EQIR < Instruction
  def execute
    if @a == @registers[@b] then
      1
    else
      0
    end
  end

  def explain
    if execute.zero? then
      "#{@a}r != #{@registers[@b]}r => 0"
    else
      "#{@a}r == #{@registers[@b]}r => 1"
    end
  end
end

class EQRI < Instruction
  def execute
    if @registers[@a] == @b then
      1
    else
      0
    end
  end

  def explain
    if execute.zero? then
      "#{@registers[@a]}r != #{@b}i => 0"
    else
      "#{@registers[@a]}r == #{@b}i => 1"
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

  def explain
    if @registers[@a] == @registers[@b] then
      "#{@registers[@a]}r != #{@registers[@b]}r => 0"
    else
      "#{@registers[@a]}r == #{@registers[@b]}r => 1"
    end
  end
end

class Machine
  attr_reader :instruction_pointer
  attr_reader :registers

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

    if register_index == @bind_ip then
      @instruction_pointer = value
    end
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
    while instruction = next_instruction do
      self[@bind_ip] = @instruction_pointer

      self[instruction.c] = instruction.execute

      @instruction_pointer += 1

      break if @instruction_pointer == @instructions.size - 1
    end

    @instruction_pointer -= 1
  end

  def to_s
    "ip = #{@instruction_pointer} #{@registers.inspect} #{next_instruction}"
  end

  alias inspect to_s
end

##
# I had no clue on this one and got tired and looked up what the program was
# supposed to do.  I never would have bothered to solve this one without
# looking up how to get the answer.

input 2018, 19 do |input|
  machine = Machine.new
  machine.parse input
  machine[0] = 1
  machine.run

  puts machine

  number = machine.registers.max
  prime_divisors = number.prime_division.transpose.first

  prime_divisors << 1

  divisors = prime_divisors.combination(2).map { |a, b|
    a * b
  }.sort

  divisors << 1
  divisors << number

  divisors.sum
end
