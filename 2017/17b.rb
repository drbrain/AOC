require_relative "../aoc"

test "356", nil

class Spinlock
  def initialize steps
    @steps = steps

    @buffer = [0]
  end

  def next year
    @buffer.rotate! @steps

    @buffer[1, 0] = year

    @buffer.rotate! 1
  end

  def value_after_zero
    zero_index = @buffer.index 0

    after = zero_index + 1
    after = 0 if after >= @buffer.size

    @buffer[after]
  end
end

input 2017, 17 do |input|
  steps = Integer input.chomp

  spinlock = Spinlock.new steps

  1.upto 50_000_000 do |year|
    spinlock.next year

    puts "%-8d %d" % [year, spinlock.value_after_zero]
    #buffer.each do |value|
    #  print "%3d " % value
    #end
    #puts
  end

  spinlock.value_after_zero
end
