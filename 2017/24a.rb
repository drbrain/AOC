require_relative "../aoc"

test <<-COMPONENTS, 31
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
  return 0 if components.empty?

  choices = $map[port]

  choices.map { |choice|
    component = [port, choice].sort

    next 0 unless components.include? component

    remaining = components - [component]

    score = build choice, remaining

    score + port + choice
  }.max
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

  build 0, components
end
