require_relative "../aoc"

test "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", 138

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

  def sum_metadata
    metadata.sum + @children.map(&:sum_metadata).sum
  end
end

##
# I started writing a loop to do the parsing, but noticed the metadata for a
# node occurs after all its children and I thought how hard it would be to
# keep track of the proper index to read from by hand.
#
# So then I made a Node class and recursively create the tree with the
# Node::create method.  (I guess ::create_tree would be a better name.)
#
# Having a real tree made summing the metadata entries very easy.

input 2018, 8 do |input|
  list = input.strip.split.map { |n| Integer n }

  tree = Node.create list

  tree.sum_metadata
end
