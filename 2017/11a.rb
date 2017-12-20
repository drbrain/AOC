require "../aoc"

test "ne,ne,ne", 3
test "ne,ne,sw,sw", 0
test "ne,ne,s,s", 2
test "se,sw,se,sw,sw", 3

SURROUNDING = [
  [ 0,  2], # N
  [ 1,  1], # NE
  [ 1, -1], # SE
  [ 0, -2], # S
  [-1, -1], # SW
  [-1,  1], # NW
]

class Costs
  def initialize
    @costs = Hash.new { |h, k| h[k] = [] }
    @min_cost = 1/0.0
  end

  def empty?
    @costs.empty?
  end

  def push cost, path
    @costs[cost] << path
    @costs.delete_if do |stored_cost|
      stored_cost > cost
    end
  end

  def pop
    costs, points = @costs.min
    point = points.shift
    @costs.delete costs if points.empty?
    point
  end
end

class Path
  def initialize start
    @closed = {}
    @open   = Costs.new

    start_estimate = estimate start

    @open.push start_estimate, [start]
  end

  def a_star
    find { |path|
      return path if path.last == [0, 0]
    }
  end

  def surrounding coordinate
    x0, y0 = coordinate

    SURROUNDING.each do |x_off, y_off|
      x = x0 + x_off
      y = y0 + y_off

      yield [x, y]
    end
  end

  def estimate point
    x, y = point

    Math.sqrt x**2 + y**2
  end

  def expand path
    x_0, y_0 = path.last

    surrounding path.last do |new|
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

input 2017, 11 do |input|
  path = input.strip.split ","

  x = 0
  y = 0

  path.each do |movement|
    case movement
    when "n" then
      y += 2
    when "ne" then
      x += 1
      y += 1
    when "se" then
      x += 1
      y -= 1
    when "s" then
      y -= 2
    when "sw" then
      x -= 1
      y -= 1
    when "nw" then
      x -= 1
      y += 1
    end
  end

  start = [x, y]

  path = Path.new start

  found = path.a_star

  found.length - 1
end
