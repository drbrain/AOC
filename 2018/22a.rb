require_relative "../aoc"
require "scanf"

test <<-INPUT, 114
depth: 510
target: 10,10
INPUT

class ModeMaze
  def initialize depth, target_x, target_y
    @depth = depth
    @target_x = target_x
    @target_y = target_y

    @erosion_level  = {}
    @geologic_index = {}
  end

  def erosion_level x, y
    @erosion_level.fetch [x, y] do |coordinate|
      index = geologic_index x, y

      @erosion_level[coordinate] = (index + @depth) % 20183
    end
  end

  def geologic_index x, y
    if x.zero? and y.zero? then
      0
    elsif x == @target_x and y == @target_y then
      0
    elsif y.zero? then
      x * 16807
    elsif x.zero? then
      y * 48271
    else
      erosion_level(x - 1, y) * erosion_level(x, y - 1)
    end
  end

  def risk_level x, y
    erosion_level = erosion_level x, y

    erosion_level % 3
  end

  def total_risk_level
    (0..@target_x).map { |x|
      (0..@target_y).map { |y|
        risk_level x, y
      }.sum
    }.sum
  end
end

input 2018, 22 do |input|
  depth, target_x, target_y = input.scanf "depth: %d\ntarget: %d,%d"

  map = ModeMaze.new depth, target_x, target_y

  map.total_risk_level
end
