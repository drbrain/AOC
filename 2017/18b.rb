require_relative "../aoc"
require_relative "../aoc"

test <<-DUET, 3
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
DUET

class Program
  attr_reader :recv_queue

  def initialize text
    @registers  = Hash.new 0
    @pc         = 0
    @recv_queue = Queue.new
    @send_queue = nil

    @program = parse text
  end

  def parse text
    text.lines.map { |line|
      instruction, x, y = line.split

      case instruction
      when "snd" then
        -> { @send_queue.enq @registers[x]; @pc += 1 }
      when "set" then
        if /\d/ =~ y then
          y = Integer y
          -> { @registers[x] = y; @pc += 1 }
        else
          -> { @registers[x] = @registers[y]; @pc += 1 }
        end
      when "add" then
        if /\d/ =~ y then
          y = Integer y
          -> { @registers[x] += y; @pc += 1 }
        else
          -> { @registers[x] += @registers[y]; @pc += 1 }
        end
      when "mul" then
        if /\d/ =~ y then
          y = Integer y
          -> { @registers[x] *= y; @pc += 1 }
        else
          -> { @registers[x] *= @registers[y]; @pc += 1 }
        end
      when "mod" then
        if /\d/ =~ y then
          y = Integer y
          -> { @registers[x] %= y; @pc += 1 }
        else
          -> { @registers[x] %= @registers[y]; @pc += 1 }
        end
      when "rcv" then
        -> { @registers[x] = @recv_queue.deq; @pc += 1 }
      when "jgz" then
        if /\d/ =~ y then
          y = Integer y
          -> {
            if @registers[x] > 0 then
              @pc += y
            else
              @pc += 1
            end
          }
        else
          -> {
            if @registers[x] > 0 then
              @pc += @registers[y]
            else
              @pc += 1
            end
          }
        end
      end
    }
  end

  def run id, send_queue
    @registers["p"] = id

    loop do
    end
end

input 2017, 18 do |input|

end
