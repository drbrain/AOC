require_relative "../aoc"
require "tsort"

# two workers, N seconds
test <<-TEST, 15
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
TEST

class Graph < Hash
  include TSort

  def initialize
    super { |h, step| h[step] = [] }
  end

  def tsort_each_node
    keys.sort.each do |node|
      yield node
    end
  end

  def tsort_each_child(node, &block)
    fetch(node).sort.each(&block)
  end
end

def pad_work workers, until_time
  workers.each do |worker|
    next if worker[until_time - 1]

    worker.fill '.', worker.size, until_time - worker.size
  end
end

input 2018, 7 do |input|
  # step => prerequisite steps
  dependencies = Graph.new
  inverse      = Graph.new

  input.lines.map do |line|
    /Step (\S+) .*? step (\S+) can begin/ =~ line

    dependencies[$1]
    dependencies[$2] << $1

    inverse[$2]
    inverse[$1] << $2
  end

  pp dep: dependencies
  pp inv: inverse

  workers = Array.new(2) { [] }

  timestamp = 0

  order = {}
  work_started = {}

  edges = dependencies.keys.select { |prerequisite|
    dependencies[prerequisite].empty?
  }.uniq.sort

  until edges.empty? do
    p edges: edges

    startable = edges.select { |candidate|
      dependencies[candidate].all? { |dep|
        order.include? dep
      }
    }

    p startable: startable

    startable.each do |node|
      order[node] = true
      edges.concat inverse[node]

      work_started[node] = true
      work = node.ord - 64

      available_worker = workers.min_by { |w| w.size }
      available_worker.concat [node] * work
    end

    timestamp = workers.max_by { |w| w.size }.size

    pad_work workers, timestamp

    puts "workers:"
    puts workers.map(&:join)
    puts

    startable.each do |node|
      edges.delete node
    end

    edges = edges.uniq.sort
  end

  workers.max_by { |w| w.size }.size
end

