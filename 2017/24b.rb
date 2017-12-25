require_relative "../aoc"

test <<-COMPONENTS, 19
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
COMPONENTS

def build port, components
  return [[]] if components.empty?

  choices = $map[port]

  bridges = choices.flat_map { |choice|
    component = [port, choice].sort

    next [[]] unless components.include? component

    remaining = components - [component]

    bridges = build choice, remaining

    bridges.map { |bridge|
     bridge << component
    }
  }.sort_by { |bridge|
    -bridge.length
  }

  max_length = bridges.first.length

  bridges.select { |bridge|
    bridge.length == max_length
  }
end

input 2017, 24 do |input|
  components = input.lines.map { |component|
    a, b = component.split "/"

    a = Integer a
    b = Integer b

    [a, b].sort
  }

  $map = Hash.new { |h, k| h[k] = [] }

  components.each do |a, b|
    $map[a] << b
    $map[b] << a
  end

  bridges = build 0, components

  bridges.map { |bridge|
    bridge.flatten.sum
  }.max
end
