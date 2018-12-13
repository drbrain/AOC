require_relative "../aoc"

test <<INPUT, "6,4"
/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/
INPUT

class Car
  TURNS = [:lt, :st, :rt]

  attr_reader :coordinate
  attr_accessor :network

  def initialize coordinate, direction
    @coordinate = coordinate
    @direction  = direction

    @crash     = false
    @network   = nil
    @next_turn = TURNS.cycle
  end

  def crash
    @crash = true
  end

  def crashed?
    @crash
  end

  def forward_coordinate
    col, row = @coordinate

    #p car_at: @coordinate, dir: @direction
    case @direction
    when :up then
      [col, row - 1]
    when :dn then
      [col, row + 1]
    when :lt then
      [col - 1, row]
    when :rt then
      [col + 1, row]
    end
  end

  def symbol
    return "\e[31mⓧ\e[39m" if crashed?

    symbol =
      case @direction
      when :up then "⇑"
      when :dn then "⇓"
      when :lt then "⇐"
      when :rt then "⇒"
      else
        raise "[BUG] Unknown direction #{@direction} for car at #{@coordinate}"
      end

    index = @network.cars.index self
    index += 1 # avoid color black
    color = 16 + (index * 13) % 216

    "\e[38;5;#{color}m#{symbol}\e[39m"
  end

  def tick
    return if crashed?

    next_coordinate = forward_coordinate
    next_track = @network[next_coordinate]

    @direction =
      case next_track

      # no direction change
      when UD, LR then @direction

      # turn regular corners
      when DL then
        case @direction
        when :dn then :rt
        when :lt then :up
        end
      when DR then
        case @direction
        when :dn then :lt
        when :rt then :up
        end
      when UL then
        case @direction
        when :up then :rt
        when :lt then :dn
        end
      when UR then
        case @direction
        when :up then :lt
        when :rt then :dn
        end

      # follow intersection turn protocol
      when Int then
        turn_at_intersection

      else
        raise "[BUG] car moving off track at #{next_coordinate}"
      end

    #p car_at: @coordinate,
    #  moving_to: next_coordinate,
    #  track: "#{next_track.class} #{next_track.symbol}",
    #  new_direction: @direction

    collisions = @network.cars.select { |car| car.coordinate == next_coordinate }

    if collisions.any? then
      @crash = true

      collisions.each do |car|
        car.crash
      end
    end

    @coordinate = next_coordinate
  end

  def to_s
    "#{symbol} at #{coordinate.join ", "}"
  end

  def turn_at_intersection
    case @direction
    when :up then
      case @next_turn.next
      when :lt then :lt
      when :st then :up
      when :rt then :rt
      end
    when :dn then
      case @next_turn.next
      when :lt then :rt
      when :st then :dn
      when :rt then :lt
      end
    when :lt then
      case @next_turn.next
      when :lt then :dn
      when :st then :lt
      when :rt then :up
      end
    when :rt then
      case @next_turn.next
      when :lt then :up
      when :st then :rt
      when :rt then :dn
      end
    end
  end
end

class Network
  UP_DOWN    = %w[| v ^ +]
  LEFT_RIGHT = %w[- > < +]

  attr_reader :cars

  def initialize input
    @cars    = []
    @layout  = {}
    @network = {}

    input.each_line.with_index.each do |line, row|
      line.each_char.with_index.each do |char, col|
        coordinate = [col, row]

        @layout[coordinate] = char
      end
    end

    @layout.each do |coordinate, char|
      parse char, coordinate
    end

    @cars.each do |car|
      car.network = self
    end
  end

  def [] coordinate
    @network[coordinate]
  end

  def cols
    1 + @network.keys.max_by { |col,| col }.first
  end

  def parse char, coordinate
    col, row = coordinate

    car_direction =
      case char
      when "^" then :up
      when "v" then :dn
      when "<" then :lt
      when ">" then :rt
      end

    @cars.push Car.new coordinate, car_direction if car_direction

    track_class =
      case char
      when /\s/ then return

      when "+"         then Int
      when *UP_DOWN    then UD
      when *LEFT_RIGHT then LR

      when "/"  then
        if UP_DOWN.include? @layout[[col, row - 1]]
          DR
        elsif UP_DOWN.include? @layout[[col, row + 1]]
          UL
        else
          raise "[BUG] unable to determine track type #{char.inspect} at #{coordinate}"
        end
      when "\\" then
        if UP_DOWN.include? @layout[[col, row - 1]]
          DL
        elsif UP_DOWN.include? @layout[[col, row + 1]]
          UR
        else
          raise "[BUG] unable to determine track type #{char.inspect} at #{coordinate}"
        end
      end

    @network[coordinate] = track_class.new coordinate
  end

  def rows
    1 + @network.keys.max_by { |_, row| row }.last
  end

  def tick
    @cars.sort_by { |car| car.coordinate.reverse }.each do |car|
      car.tick
    end

    @cars.delete_if { |car| car.crashed? }

    return true if @cars.size > 1
  end

  def to_s
    grid = Array.new(rows) { Array.new cols, " " }

    @network.each do |(col, row), track|
      grid[row][col] = track.symbol
    end

    @cars.each do |car|
      col, row = car.coordinate
      grid[row][col] = car.symbol
    end

    grid.map { |row|
      row.join
    }.join "\n"
  end
end

class Track
  def initialize coordinate
    @coordinate = coordinate
  end
end

# straight
class LR < Track
  def symbol
    "═"
  end
end

class UD < Track
  def symbol
    "║"
  end
end

# curves named by inbound car direction
class DL < Track
  def symbol
    "╚"
  end
end

class DR < Track
  def symbol
    "╝"
  end
end

class UL < Track
  def symbol
    "╔"
  end
end

class UR < Track
  def symbol
    "╗"
  end
end

class Int < Track # Intersection
  def symbol
    "╬"
  end
end

##
# A small change here to continue when a car crashes until there is only one
# left.

input 2018, 13 do |input|
  network = Network.new input

  #puts network.cars.sort_by { |car| car.coordinate.reverse }.join "\n"
  #puts network
  #puts

  while network.tick do
    #puts network.cars.sort_by { |car| car.coordinate.reverse }.join "\n"
    #puts network
    #puts
  end

  #puts network.cars.sort_by { |car| car.coordinate.reverse }.join "\n"
  #puts network
  #puts

  car = network.cars.first

  car.coordinate.join ","
end
