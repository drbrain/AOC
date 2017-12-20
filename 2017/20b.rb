require_relative "../aoc"

test <<-INPUT, 1
p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
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

  2_000.times do
    particles.map! do |p, v, a|
      next unless p

      v = v.zip(a).map { |a, b|
        a + b
      }

      p = p.zip(v).map { |a, b|
        a + b
      }

      [p, v, a]
    end

    exists = Hash.new 0
    positions = Hash.new { |h,p| h[p] = [] }

    particles.each_with_index do |(p, _), index|
      positions[p] << index
      exists[p] += 1
    end

    exists.select { |p, count|
      count > 1
    }.each do |p,|
      positions[p].each do |index|
        particles[index] = nil
      end
    end
  end

  particles.compact.count
end
