require_relative "../aoc"
require "scanf"

test <<-TEST, 4
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
TEST

input 2018, 3 do |claims|
  fabric = Hash.new 0

  claims.lines.each do |line|
    _, x_offset, y_offset, h, w = line.scanf "#%d @ %d,%d: %dx%d"

    (x_offset...x_offset + h).each do |x|
      (y_offset...y_offset + w).each do |y|
        fabric[[x, y]] += 1
      end
    end
  end

  fabric.values.count { |claimed| claimed > 1 }
end
