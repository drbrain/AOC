require_relative "../aoc"
require "set"

test <<-RECORD, 6
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
RECORD

input 2017, 12 do |input|
  pipes = Hash.new { |h, id| h[id] = Set.new }

  input.lines.each do |list|
    /(\S+) <-> (.*)/ =~ list
    id = Integer $1
    endpoints = $2.split(",").map { |endpoint| Integer endpoint }

    endpoints.each do |endpoint|
      pipes[id] << endpoint
      pipes[endpoint] << id
    end
  end

  current = 0
  seen    = {}
  todo    = pipes[current].to_a

  until todo.empty? do
    seen[current] = true

    candidates = pipes[current]
    valid = candidates.to_a - seen.keys

    todo.concat valid

    current = todo.shift
  end

  seen.size
end
