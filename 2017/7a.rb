require_relative "../aoc"

test <<-INPUT, "tknk"
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
INPUT

input 2017, 7 do |input|
  lines = input.lines.map { |line|
    /(\w+) (\S+)( -> (.*))?/ =~ line

    holding = $4.split ", " if $4

    [$1, holding]
  }

  tree = {}
  parents = {}

  lines.each do |name, children|
    tree[name] = children
    next unless children
    children.each do |child|
      parents[child] = name
    end
  end

  tree.keys.find { |key|
    not parents[key]
  }
end
