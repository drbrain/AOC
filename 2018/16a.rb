require_relative "../aoc"
require "scanf"

test <<INPUT, 1
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
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
end

class ADDR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = @registers[a] + @registers[b]

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class ADDI < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = @registers[a] + b

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class MULR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = @registers[a] * @registers[b]

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class MULI < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = @registers[a] * b

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class BANR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = @registers[a] & @registers[b]

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class BANI < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    after[c] == @registers[a] & b
  rescue InvalidRegister
    false
  end
end

class BORR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    after[c] == @registers[a] | @registers[b]
  rescue InvalidRegister
    false
  end
end

class BORI < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = @registers[a] | b

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class SETR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = @registers[a]

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class SETI < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected = a

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class GTIR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected =
      if a > @registers[b] then
        1
      else
        0
      end

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class GTRI < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected =
      if @registers[a] > b then
        1
      else
        0
      end

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class GTRR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected =
      if @registers[a] > @registers[b] then
        1
      else
        0
      end

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class EQIR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected =
      if a == @registers[b] then
        1
      else
        0
      end

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class EQRI < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected =
      if @registers[a] == b then
        1
      else
        0
      end

    after[c] == expected
  rescue InvalidRegister
    false
  end
end

class EQRR < Opcode
  def === arguments_after
    (a, b, c), after = arguments_after

    expected =
      if @registers[a] == @registers[b] then
        1
      else
        0
      end

    after[c] == expected
  rescue InvalidRegister
    false
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

##
# This problem shares the under-specification and uncertainty of yesterday's
# problem, but had fewer levels of that so I was able to track what needs to
# be done and when better.
#
# I made several revisions to the match (#===) methods as my understanding of
# the relationship between the two registers and the instruction in the sample
# improved.  It took me a few readings to understand that numbers in the
# register and the numbers in the instruction were different things.
#
# The Registers class was helpful because I was getting TypeErrors when
# walking off the end of the Registers when trying to match.  The exception
# made sure I could separate program errors opcodes that didn't match a
# sample.
#
# Using Class#inherited was nice because it gathered all my opcodes up without
# having to write each name again.

input 2018, 16 do |input|
  samples = parse(input).select { |type, object|
    type == :sample
  }.map(&:last)

  samples.map { |before, (instruction, *arguments), after|
    Opcode.opcodes.select { |opcode_class|
      opcode = opcode_class.new before

      opcode === [arguments, after]
    }
  }.count { |matches|
    matches.size >= 3
  }
end
