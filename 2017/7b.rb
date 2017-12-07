require_relative "../aoc"

test <<-INPUT, 60
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

def weigh name
  weight = $weights[name]
  children = $tree[name]

  children.map { |child|
    weigh child
  }.sum + weight
end

input 2017, 7 do |input|
  lines = input.lines.map { |line|
    /(\w+) \((\S+)\)( -> (.*))?/ =~ line

    weight = Integer $2
    holding = $4.split ", " if $4
    holding ||= []

    [$1, holding, weight]
  }

  $tree = {}
  $parents = {}
  $weights = {}

  lines.each do |name, children, weight|
    $weights[name] = weight
    $tree[name]    = children

    children.each do |child|
      $parents[child] = name
    end
  end

  root = $tree.keys.find { |key|
    not $parents[key]
  }

  off = loop do
    weight_counts = Hash.new 0

    children = $tree[root]

    child_weights = children.map { |name|
      weight = weigh name
      weight_counts[weight] += 1

      [name, weight]
    }

    p child_weights

    break root if weight_counts.size == 1

    weight, = weight_counts.find { |name, count|
      count == 1
    }

    root, = child_weights.rassoc weight
  end

  off

  parent = $parents[off]

  children = $tree[parent].map { |child|
    weigh(child)
  }

  min, max = children.minmax

  $weights[off] + min - max
end
