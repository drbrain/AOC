require "../aoc"

class Lehmer
  def initialize seed, factor
    @seed   = seed
    @factor = factor
  end

  def next
    @seed = (@seed * @factor) % 0x7fffffff

    @seed & 0xffff
  end
end

input 2017, 15 do |input|
  seeds = input.lines.map { |line|
    /\d+/ =~ line

    Integer $&
  }

  a = Lehmer.new seeds.first, 16807
  b = Lehmer.new seeds.last,  48271

  40_000_000.times.count do
    a.next == b.next
  end
end
