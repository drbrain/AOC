require_relative "../aoc"
require_relative "../aoc"

test <<-DUET, 4
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
DUET

input 2017, 18 do |input|
  registers    = Hash.new 0
  last_sound   = 0
  pc           = 0

  program = input.lines.map { |line|
    instruction, x, y = line.split

    case instruction
    when "snd" then
      -> { last_sound = registers[x]; pc += 1 }
    when "set" then
      if /\d/ =~ y then
        y = Integer y
        -> { registers[x] = y; pc += 1 }
      else
        -> { registers[x] = registers[y]; pc += 1 }
      end
    when "add" then
      if /\d/ =~ y then
        y = Integer y
        -> { registers[x] += y; pc += 1 }
      else
        -> { registers[x] += registers[y]; pc += 1 }
      end
    when "mul" then
      if /\d/ =~ y then
        y = Integer y
        -> { registers[x] *= y; pc += 1 }
      else
        -> { registers[x] *= registers[y]; pc += 1 }
      end
    when "mod" then
      if /\d/ =~ y then
        y = Integer y
        -> { registers[x] %= y; pc += 1 }
      else
        -> { registers[x] %= registers[y]; pc += 1 }
      end
    when "rcv" then
      -> {
        throw :recover, last_sound unless registers[x].zero?
        pc += 1
      }
    when "jgz" then
      if /\d/ =~ y then
        y = Integer y
        -> {
          if registers[x] > 0 then
            pc += y
          else
            pc += 1
          end
        }
      else
        -> {
          if registers[x] > 0 then
            pc += registers[y]
          else
            pc += 1
          end
        }
      end
    end
  }

  catch :recover do
    loop do
      program[pc].call
    end
  end
end
