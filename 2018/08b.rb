require_relative "../aoc"

test "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", 66

class Node
  attr_reader :children
  attr_reader :metadata

  def self.create list
    node = new

    child_count    = list.shift
    metadata_count = list.shift

    child_count.times do
      child = create list

      node.children << child
    end

    metadata_count.times do
      metadata = list.shift

      node.metadata << metadata
    end

    node
  end

  def initialize
    @children = []
    @metadata = []
  end

  def childless?
    @children.empty?
  end

  def value
    return @metadata.sum if childless?

    @metadata.map { |index|
      @children[index - 1]&.value
    }.compact.sum
  end
end

##
# Here I added the #childless? method for some readability points.
#
# I thought finding the value of a node might require memoization as the
# structure may require re-traversal of a large portion of the tree, but this
# turned out to be unnecessary.

input 2018, 8 do |input|
  list = input.strip.split.map { |n| Integer n }

  tree = Node.create list

  tree.value
end
