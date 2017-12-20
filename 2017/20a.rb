require_relative "../aoc"

test <<-INPUT, 0
p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>
INPUT

input 2017, 20 do |input|
  particles = input.lines.map { |line|
    parts = line.split ", "

    p, v, a = parts.map { |part|
      /< *(-?\d+), *(-?\d+) *,(-?\d+)>/ =~ part

      [Integer($1), Integer($2), Integer($3)]
    }
  }

  min = nil

  10_000.times do
    particles.map! do |p, v, a|
      v = v.zip(a).map { |a, b|
        a + b
      }

      p = p.zip(v).map { |a, b|
        a + b
      }

      [p, v, a]
    end

    distances = particles.map { |(x, y, z),|
      x.abs + y.abs + z.abs
    }

    min = distances.index distances.min
  end

  min
end
