require "../aoc"

test <<INPUT, 27730
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
INPUT

test <<INPUT, 27755
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
INPUT

test <<INPUT, 28944
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
INPUT

test <<INPUT, 18740
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
INPUT

test <<INPUT, 39514
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
INPUT

test <<INPUT, 36334
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
INPUT

class Battle
  attr_reader :map
  attr_reader :units

  def initialize input
    @units = {} # [row, col] => Unit
    @map   = {} # [row, col] => Tile
    @turns = 0

    parse input
  end

  def [] coordinate
    @units.fetch coordinate do
      @map.fetch coordinate
    end
  end

  def parse input
    input.each_line.with_index do |line, row|
      line.strip.each_char.with_index do |object_char, col|
        object_class =
          case object_char
          when "E" then Elf
          when "G" then Goblin
          when "." then Floor
          when "#" then Wall
          else          raise "[BUG] Unknown tile #{object_char}"
          end

        next unless object_class

        coordinate = [row, col]

        object = object_class.new self, coordinate

        case object
        when Unit then
          @units[coordinate] = object
          @map[coordinate] = Floor.new self, coordinate
        else
          @map[coordinate] = object
        end
      end
    end
  end

  def run
    catch :no_enemies do
      until won? do
        turn

        #puts "Turn #{@turns}"
        #puts self
        #puts
      end
    end
  end

  def score
    hp = turn_order.map { |unit| unit.hp }.sum

    p hp: hp, turn: @turns

    @turns * hp
  end

  def turn
    turn_order.each do |unit|
      unit.turn
    end

    @turns += 1
  end

  def turn_order
    @units.values.sort
  end

  def to_s
    rows = @map.keys.max_by { |row,| row }.first
    cols = @map.keys.max_by { |_, col| col }.last

    (0..rows).map { |row|
      units = []

      (0..cols).map { |col|
        tile = self[[row, col]]
        units << tile if Unit === tile

        tile&.symbol || " "
      }.join + "\t#{units.join " "}"
    }.join "\n"
  end

  def won?
    @units.values.uniq { |unit| unit.class }.size == 1
  end
end

class Costs
  def initialize
    @costs = Hash.new { |h, k| h[k] = [] }
    @min_cost = Infinity
  end

  def empty?
    @costs.empty?
  end

  def push cost, path
    @costs[cost] << path
    @costs[cost].sort!
    #@costs.delete_if do |stored_cost|
    #  stored_cost > cost
    #end
  end

  def pop
    costs, points = @costs.min
    point = points.shift
    @costs.delete costs if points.empty?
    point
  end
end

class Path
  def self.a_star battle, unit, target
    path = new battle, unit, target
    path.a_star
  end

  def initialize battle, unit, target
    @battle = battle
    @target = target
    @dest   = target.coordinate
    @d_row, @d_col = target.coordinate

    @closed = {}
    @open   = Costs.new

    start_estimate = estimate unit.coordinate

    @open.push start_estimate, [unit.coordinate]
  end

  def a_star
    find { |path|
      return path if path.last == @dest
    }
  end

  def estimate coordinate
    row, col = coordinate

    (row - @d_row).abs + (col - @d_col).abs
  end

  def expand path
    current = @battle[path.last]

    surrounding = current.adjacent.select { |tile|
      not @closed.include?(tile.coordinate) and
        (tile.floor? or
         tile == @target)
    }

    surrounding.each do |tile|
      new = tile.coordinate

      new_path = path + [new]

      new_cost = estimate new
      estimate = new_path.length + new_cost

      @open.push estimate, new_path
    end
  end

  def find
    until @open.empty?
      path = @open.pop

      yield path

      @closed[path.last] = true

      expand path
    end

    nil
  end
end

class Tile
  include Comparable

  attr_reader :battle
  attr_reader :coordinate

  def initialize battle, coordinate
    @battle     = battle
    @coordinate = coordinate
  end

  def <=> other
    return nil unless Tile === other

    @coordinate <=> other.coordinate
  end

  def adjacent
    return enum_for __method__ unless block_given?

    row, col = @coordinate

    yield @battle[[row - 1, col]]
    yield @battle[[row,     col - 1]]
    yield @battle[[row,     col + 1]]
    yield @battle[[row + 1, col]]
  end

  def distance_to tile
    row,   col   = tile.coordinate
    s_row, s_col = @coordinate

    (row - s_row).abs + (col - s_col).abs
  end

  def floor?
    false
  end

  def inspect
    "[#{symbol} @ #{@coordinate.join ", "}]"
  end

  def in_range
    adjacent.select { |tile| tile.floor? }
  end

  alias to_s inspect

  def unit?
    false
  end
end

class Unit < Tile
  attr_accessor :hp

  def initialize battle, coordinate
    super

    @hp = 200
    @ap = 3
  end

  def attack
    target = attackable.sort_by { |enemy|
      [enemy.hp, enemy]
    }.first

    target.hp -= @ap

    return if target.hp.positive?

    @battle.units.delete target.coordinate
  end

  def can_attack?
    attackable.any?
  end

  def can_move_to_target?
    targets.any? { |target|
      target.in_range.any? { |tile|
        tile.floor?
      }
    }
  end

  def can_take_action?
    can_move_to_target? or
      can_attack?
  end

  def attackable
    adjacent.select { |tile|
      enemy_on? tile
    }
  end

  def dead?
    @hp <= 0
  end

  def enemy_on? tile
    tile.unit? and not tile.class === self
  end

  def inspect
    "[#{symbol} (#{@hp}) @ #{@coordinate.join ", "}]"
  end

  alias to_s inspect

  def move
    paths = paths_to_closest_target

    return if paths.empty?

    path = paths.first
    step = path[1]

    @battle.units.delete @coordinate
    @coordinate = step
    @battle.units[@coordinate] = self
  end

  ##
  # Use A* from 2017 problem 11

  def paths_to_closest_target
    in_range_of_enemy =
      targets.flat_map { |target|
        target.in_range
      }.sort

    in_range_of_enemy.map { |tile|
      Path.a_star @battle, self, tile
    }.compact.sort_by { |path|
      [path.length, path.last]
    }
  end

  def targets
    @battle.units.values.reject { |unit| self.class === unit }
  end

  def turn
    throw :no_enemies if targets.empty?

    return if dead?

    move unless can_attack?

    attack if can_attack?
  end

  def unit?
    true
  end
end

class Elf < Unit
  def symbol
    "\e[92mE\e[0m"
  end
end

class Goblin < Unit
  def symbol
    "\e[96mG\e[0m"
  end
end

class Floor < Tile
  def floor?
    true
  end

  def symbol
    "\e[94m.\e[0m"
  end
end

class Wall < Tile
  def symbol
    "\e[90m#\e[0m"
  end
end

input 2018, 15 do |input|
  battle = Battle.new input

  battle.run

  puts battle

  battle.score
end
