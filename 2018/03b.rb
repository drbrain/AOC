require_relative "../aoc"
require "scanf"

test <<-TEST, 3
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
TEST

##
# The <code>reclaimed[id] = true</code> was an important piece I missed and
# was a source of great confusion when my first answer didn't work.  It took
# me some time to realize that I needed to mark the claim that was colliding
# with the other claims was also in conflict.
#
# When I was documenting this solution I realized I could ask the +reclaimed+
# Hash directly which claim was not covered by other claims, provided a claim
# started out marked as not-reclaimed.

input 2018, 3 do |claims|
  reclaimed = {}
  # [x, y] coordinates => Array of ids of other claims for this coordinate
  fabric    = Hash.new { |h, location| h[location] = [] }

  claims.lines.each do |line|
    id, x_offset, y_offset, h, w = line.scanf "#%d @ %d,%d: %dx%d"

    reclaimed[id] = false

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

  reclaimed.rassoc(false).first
end
