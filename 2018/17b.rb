require_relative "../aoc"
require "scanf"

test <<INPUT, 29
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
INPUT

class Profile
  attr_reader :max_depth
  attr_reader :min_depth

  attr_reader :x_max
  attr_reader :x_min
  attr_reader :x_range

  def initialize scans
    @scans = scans.sort_by { |x, y_range| [x, y_range.begin, y_range.end] }

    # [x, y] => type-char
    @profile = Hash.new "."
    @water   = {}

    scans.each do |x, y_range|
      y_range.each do |y|
        @profile[[x, y]] = "#"
      end
    end

    @spring = Spring.new self, [500, 0]

    scans_min, scans_max = @scans.map(&:first).minmax
    @x_min, @x_max = scans_min - 5, scans_max + 5
    @x_range = Range.new @x_min, @x_max

    @min_depth = @scans.map { |_, y_range| y_range.begin }.min
    @max_depth = @scans.map { |_, y_range| y_range.end }.max
  end

  def [] coordinate
    @water.fetch coordinate do
      @profile[coordinate]
    end
  end

  def flow
    current = nil
    to_do   = [@spring]

    until to_do.empty? do
      puts "#{current} using #{water_used}" if (water_used % 100).zero?
      #break if water_used >= 500

      current = to_do.shift

      next if current.out_of_bounds?

      @water[current.coordinate] = current.symbol

      next_flow = current.flow

      to_do.concat next_flow

      to_do.uniq!
    end

  rescue Exception
    puts self
    puts
    puts current
    puts
    pp to_do
    raise
  end

  def still_water_used
    @water.count { |_, type| type == "~" }
  end

  def to_s
    last_x = nil

    lines = 1 + @max_depth
    out = Array.new lines do Hash.new "\e[90m.\e[0m" end

    @profile.each do |(x, y), type|
      next if type == "."

      out[y][x] = type
    end

    @water.each do |(x, y), type|
      out[y][x] = type
    rescue
      next
    end

    header = x_range.to_a.map { |n| n.to_s.chars }.transpose

    header = header.map { |row|
      row.join
    }.join "\n"

    out = out.map { |row|
      @x_range.map { |col|
        row[col]
      }.join
    }.join "\n"

    "#{header}\n#{out}"
  end

  def water_used
    @water.count - @min_depth
  end
end

class OutOfBounds < RuntimeError
  attr_reader :coordinate

  def initialize coordinate
    @coordinate = coordinate

    super "out of bounds at #{@coordinate}"
  end
end

class Water
  attr_reader :coordinate

  def initialize profile, coordinate
    @profile    = profile
    @coordinate = coordinate
    @x, @y      = coordinate
  end

  def above
    @profile[above_coordinate]
  end

  def above_coordinate
    [@x, @y - 1]
  end

  def below
    @profile[below_coordinate]
  end

  def below_coordinate
    [@x, @y + 1]
  end

  def eql? other
    self.class == other.class and
      @coordinate == other.coordinate
  end

  def hash
    [@coordinate, symbol].hash
  end

  def here
    @profile[@coordinate]
  end

  def inspect
    "[#{symbol} @ #{@coordinate.join ","}]"
  end

  alias to_s inspect

  def left
    @profile[left_coordinate]
  end

  def left_coordinate
    left = @x - 1

    raise OutOfBounds, [left, @y] unless @profile.x_range.cover? left

    [left, @y]
  end

  def level_bounds
    left_wall  = @x - 1
    right_wall = @x + 1

    until left_wall <= @profile.x_min or "#" == @profile[[left_wall, @y]] do
      left_wall -= 1
    end

    return false unless @profile[left_wall]

    until right_wall >= @profile.x_max or "#" == @profile[[right_wall, @y]] do
      right_wall += 1
    end

    return false unless @profile[[right_wall, @y]]

    (left_wall + 1..right_wall - 1)
  end

  def out_of_bounds?
    return false unless @profile.x_range.cover? @x

    @y > @profile.max_depth
  end

  def right
    @profile[right_coordinate]
  end

  def right_coordinate
    right = @x + 1

    raise OutOfBounds, [right, @y] unless @profile.x_range.cover? right

    [right, @y]
  end
end

class Spring < Water
  def flow
    case below
    when "#" then
      raise "[BUG] unspecified spring behavior"
    when "." then
      [Flow.new(@profile, below_coordinate)]
    else
      raise "[BUG] unknown type at #{below} for spring"
    end
  end

  def symbol
    "+"
  end
end

class Flow < Water
  def flow
    case below
    when "." then
      [Flow.new(@profile, below_coordinate)]

    when "#", "~" then
      if bounds = level_bounds then
        contained =
          level_bounds.all? { |x|
            ["#", "~"].include? @profile[[x, @y + 1]]
          }

        if contained then
          settles = level_bounds.map { |x|
            Settle.new @profile, [x, @y]
          }

          fill_up = Flow.new @profile, above_coordinate

          settles << fill_up
          settles
        else
          flow_left_right
        end
      else
        flow_left_right
      end

    when "|" then
      [] # ignore
    else
      raise "[BUG] unknown type #{below} at #{@coordinate}"
    end
  end

  def flow_left_right
    flow = []

    left_flow  = @x - 1
    right_flow = @x + 1

    while left_flow > @profile.x_min do
      break if @profile[[left_flow, @y]] == "#"
      break if @profile[[left_flow, @y]] == "|" and
                 @profile[[left_flow, @y - 1]] != "|"


      flow.push Flow.new @profile, [left_flow, @y]

      break if @profile[[left_flow, @y + 1]] == "."

      left_flow -= 1
    end

    while right_flow < @profile.x_max do
      break if @profile[[right_flow, @y]] == "#"
      break if @profile[[right_flow, @y]] == "|" and
                 @profile[[right_flow, @y - 1]] != "|"

      flow.push Flow.new @profile, [right_flow, @y]

      break if @profile[[right_flow, @y + 1]] == "."

      right_flow -= 1
    end

    flow
  end

  def symbol
    "|"
  end
end

class Settle < Water
  def flow
    []
  end

  def symbol
    "~"
  end
end

##
# Since I always ended up with still water recorded as a "~" in the water Hash
# I only needed to count these characters from the Hash.  When flow water
# enters a basin a second time it doesn't overwrite the characters there when
# the basin fills back up.

input 2018, 17 do |input|
  scans = []

  input.lines.each do |line|
    axis_a, a, axis_b, b1, b2 = line.scanf "%c=%d, %c=%d..%d"

    case axis_a
    when "x" then
      scans << [a, b1..b2]
    when "y" then
      samples = (b1..b2).map { |b|
        [b, a..a]
      }

      scans.concat samples
    else
      raise "[BUG] unknown axis in #{line.dump}"
    end
  end

  profile = Profile.new scans

  profile.flow

  puts profile

  profile.still_water_used
end
