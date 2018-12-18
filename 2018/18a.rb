require_relative "../aoc"

test <<-INPUT, 1147
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
INPUT

class Map
  def initialize
    # [row, col] => Tile
    @map = {}
  end

  def [] coordinate
    @map.fetch coordinate
  end

  def []= coordinate, tile
    @map[coordinate] = tile
  end

  def include? coordinate
    @map.include? coordinate
  end

  def lumberyards
    @map.count { |_, tile| Lumberyard === tile }
  end

  def parse map
    map.each_line.with_index do |line, row|
      line.each_char.with_index do |tile, col|
        coordinate = [row, col]

        tile_class =
          case tile
          when "." then Open
          when "|" then Trees
          when "#" then Lumberyard
          when "\n" then next
          else
            raise "[BUG] unknown tile type #{tile.inspect}"
          end

        tile = tile_class.new self, coordinate

        @map[coordinate] = tile
      end
    end
  end

  def step
    new_map = Map.new

    @map.each do |coordinate, tile|
      new_map[coordinate] = tile.step new_map
    end

    new_map
  end

  def trees
    @map.count { |_, tile| Trees === tile }
  end

  def to_s
    rows, cols = @map.keys.last

    rows += 1
    cols += 1

    rows.times.map { |row|
      cols.times.map { |col|
        @map[[row, col]].symbol
      }.join
    }.join "\n"
  end
end

class Tile
  def initialize map, coordinate
    @map        = map
    @coordinate = coordinate
  end

  def adjacent
    return enum_for __method__ unless block_given?
    row, col = @coordinate

    adjacent_coordinates =
      [
        [row - 1, col],
        [row - 1, col + 1],
        [row,     col + 1],
        [row + 1, col + 1],
        [row + 1, col],
        [row + 1, col - 1],
        [row,     col - 1],
        [row - 1, col - 1],
      ]

    adjacent_coordinates.each do |coordinate|
      yield @map[coordinate] if @map.include? coordinate
    end

    self
  end

  def adjacent_to tile_class, min = 1
    adjacent.count { |tile| tile_class === tile } >= min
  end
end

class Lumberyard < Tile
  def step map
    if adjacent_to Lumberyard and adjacent_to Trees then
      Lumberyard.new map, @coordinate
    else
      Open.new map, @coordinate
    end
  end

  def symbol
    "#"
  end
end

class Open < Tile
  def step map
    if adjacent_to Trees, 3 then
      Trees.new map, @coordinate
    else
      Open.new map, @coordinate
    end
  end

  def symbol
    "."
  end
end

class Trees < Tile
  def step map
    if adjacent_to Lumberyard, 3 then
      Lumberyard.new map, @coordinate
    else
      Trees.new map, @coordinate
    end
  end

  def symbol
    "|"
  end
end

##
# This implementation turned out to be fancy but I find it easy to read as
# each part of the problem is isolated in a class.
#
# I don't like that #adjacent_to uses Enumerable#count because it checks more
# tiles than it needs to.  It turns out the slowdown there is largely
# irrelevant as this implementation does so much other work elsewhere.

input 2018, 18 do |input|
  map = Map.new
  map.parse input

  10.times do
    map = map.step
  end

  map.lumberyards * map.trees
end
