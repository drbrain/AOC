require_relative "../aoc"
require "scanf"

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
end

class Opcode
  class << self
    attr_reader :opcodes
  end

  @opcodes = []

  def self.inherited child
    @opcodes << child
  end

  def initialize registers
    @registers = registers
  end

  def === arguments_after
    (*arguments, c), after = arguments_after

    expected = execute(*arguments)

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class ADDR < Opcode
  def execute a, b
    @registers[a] + @registers[b]
  end
end

class ADDI < Opcode
  def execute a, b
    @registers[a] + b
  end
end

class MULR < Opcode
  def execute a, b
    @registers[a] * @registers[b]
  end
end

class MULI < Opcode
  def execute a, b
    @registers[a] * b
  end
end

class BANR < Opcode
  def execute a, b
    @registers[a] & @registers[b]
  end
end

class BANI < Opcode
  def execute a, b
    @registers[a] & b
  end
end

class BORR < Opcode
  def execute a, b
    @registers[a] | @registers[b]
  end
end

class BORI < Opcode
  def execute a, b
    @registers[a] | b
  end
end

class SETR < Opcode
  def execute a, b
    @registers[a]
  end
end

class SETI < Opcode
  def execute a, b
    a
  end
end

class GTIR < Opcode
  def execute a, b
    if a > @registers[b] then
      1
    else
      0
    end
  end
end

class GTRI < Opcode
  def execute a, b
    if @registers[a] > b then
      1
    else
      0
    end
  end
end

class GTRR < Opcode
  def execute a, b
    if @registers[a] > @registers[b] then
      1
    else
      0
    end
  end
end

class EQIR < Opcode
  def execute a, b
    if a == @registers[b] then
      1
    else
      0
    end
  end
end

class EQRI < Opcode
  def execute a, b
    if @registers[a] == b then
      1
    else
      0
    end
  end
end

class EQRR < Opcode
  def execute a, b
    if @registers[a] == @registers[b] then
      1
    else
      0
    end
  end
end

class Machine
  attr_reader :registers

  def initialize opcode_table, instructions
    @registers = Registers.new [0, 0, 0, 0]

    @opcode_table = opcode_table.map { |opcode|
      opcode.new @registers
    }

    @instructions = instructions
  end

  def run
    @instructions.each do |instruction, a, b, c|
      opcode = @opcode_table[instruction]

      @registers[c] = opcode.execute a, b
    end
  end
end

def parse input
  return enum_for __method__, input unless block_given?

  stream = input.lines.lazy.map(&:strip)

  loop do
    case stream.peek
    when "" then
      stream.next
    when /^Before/ then
      before = stream.next.scanf "Before: [%d, %d, %d, %d]"
      opcode = stream.next.scanf "%d %d %d %d"
      after  = stream.next.scanf  "After: [%d, %d, %d, %d]"

      before = Registers.new before
      after  = Registers.new after

      yield :sample, [before, opcode, after]
    when /^\d/
      opcode = stream.next.scanf "%d %d %d %d"

      yield :opcode, opcode
    else
      raise "[BUG] unknown line #{stream.peek.inspect}"
    end
  end
rescue StopIteration
end

def opcode_candidates samples
  # number => opcodes
  opcode_map = Hash.new { |h, instruction| h[instruction] = Opcode.opcodes.dup }

  samples.each do |before, (instruction, *arguments), after|
    Opcode.opcodes.reject { |opcode_class|
      opcode = opcode_class.new before

      opcode === [arguments, after]
    }.each do |opcode|
      opcode_map[instruction].delete opcode
    end
  end

  opcode_map
end

def resolve_opcodes samples
  opcode_candidates = opcode_candidates samples

  known = []

  until known.length == 16 and known.all? do
    opcode_candidates.select { |instruction, opcodes|
      opcodes.length == 1
    }.each do |instruction, (opcode, _)|
      known[instruction] = opcode
    end

    opcode_candidates.each do |instruction, opcodes|
      known.compact.each do |known_opcode|
        opcodes.delete known_opcode
      end
    end
  end

  known
end

##
# This one took some extra time to finish because I designed part A poorly.
# After reading the input while writing part A I was sure I'd have to execute
# the opcodes but I didn't bother making an #execute method from the
# beginning.  This would have saved a bunch of time.  I may even have realized
# I could put #=== on Opcode like I have here.
#
# It took some thought to figure out how to resolve the opcodes to a single
# address.  I don't like how that gets done because of the repeated loops.
#
# One piece I missed was that I didn't have to pass in C from the instruction,
# so that took a bit of time to straighten out as well.

input 2018, 16 do |input|
  samples, instructions = parse(input).partition { |type, object|
    type == :sample
  }

  samples.map!(&:last)
  instructions.map!(&:last)

  opcode_table = resolve_opcodes samples

  machine = Machine.new opcode_table, instructions

  machine.run

  machine.registers[0]
end
