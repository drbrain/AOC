require "../aoc"

class Lehmer
  def initialize seed, factor, multiple
    @seed     = seed
    @factor   = factor
    @multiple = multiple
  end

  def next
    begin
      @seed = (@seed * @factor) % 0x7fffffff
    end until 0 == @seed % @multiple

    @seed & 0xffff
  end
end

input 2017, 15 do |input|
  seeds = input.lines.map { |line|
    /\d+/ =~ line

    Integer $&
  }

  a = Lehmer.new seeds.first, 16807, 4
  b = Lehmer.new seeds.last,  48271, 8

  5_000_000.times.count do
    a.next == b.next
  end
end
