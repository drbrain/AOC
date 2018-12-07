require_relative "../aoc"

test <<-TEST, "CABDFE"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
TEST

##
# I tried using TSort at first but the alphabetical order requirement got in
# the way of having that work as all edges have an implicit dependency on the
# alphabetically early one running.
#
# This solution exploits the property of Hash where keys are stored in
# insertion order.  That allows the <code>order.include?</code> check to run
# faster.
#
# When reviewing this later the destruction of the inverse dependency graph is
# not necessary, and only makes the implementation more confusing.
#
# The important steps are:
#
# 1. Find the first root and add it to the completed order
# 2. Add any other roots to a list of edges to explore
# 3. Make the list of edges to explore unique and sorted
# 4. Find the first edge from the edges list where all its dependencies are
#    completed (exists in order)
# 5. Add that edge's dependencies to the list of edges to explore
# 6. Go to 3 until you run out of edges
# 7. Print out order

input 2018, 7 do |input|
  # step => prerequisite steps
  dependencies = Hash.new { |h, step| h[step] = [] }
  inverse      = Hash.new { |h, step| h[step] = [] }

  input.lines.map do |line|
    /Step (\S+) .*? step (\S+) can begin/ =~ line

    dependencies[$1]
    dependencies[$2] << $1

    inverse[$2]
    inverse[$1] << $2
  end

  order = {}

  roots = dependencies.keys.select { |prerequisite|
    dependencies[prerequisite].empty?
  }.sort

  root = roots.shift

  prerequisites = roots

  until inverse.empty? do
    order[root] = true

    new_prerequisites = inverse.delete root

    prerequisites.concat new_prerequisites
    prerequisites = prerequisites.uniq

    prerequisites = prerequisites.select { |step|
      dependencies[step].all? { |step_prerequisite|
        order.include? step_prerequisite
      }
    }.sort

    root = prerequisites.shift
  end

  order.keys.join
end
