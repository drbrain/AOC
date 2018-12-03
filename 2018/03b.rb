require_relative "../aoc"
require "scanf"

test <<-TEST, 3
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
TEST

input 2018, 3 do |claims|
  ids       = nil
  reclaimed = {}
  fabric    = Hash.new { |h, location| h[location] = [] }

  claims.lines.each do |line|
    id, x_offset, y_offset, h, w = line.scanf "#%d @ %d,%d: %dx%d"

    ids = id

    (x_offset...x_offset + h).each do |x|
      (y_offset...y_offset + w).each do |y|
        location = [x, y]

        existing_claim_ids = fabric[location]
        unless existing_claim_ids.empty?
          reclaimed[id] = true

          existing_claim_ids.each do |other_id|
            reclaimed[other_id] = true
          end
        end

        fabric[location] << id
      end
    end
  end

  1.upto(ids).find { |id|
    not reclaimed.include? id
  }
end
