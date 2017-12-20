require_relative "../aoc"

test <<-INPUT, 10
0: 3
1: 2
4: 4
6: 4
INPUT

def reset
  $scanner_ranges.each_key do |depth|
    $scanner_positions[depth] = 0
    $scanner_directions[depth] = :+
  end
end

def tick
  $scanner_positions.each do |depth, position|
    max_position = $scanner_ranges[depth] - 1

    direction = $scanner_directions[depth]
    new_position = position.send direction, 1
    $scanner_positions[depth] = new_position

    if new_position == max_position or new_position.zero?
      reverse =
        if direction == :+
          :-
        else
          :+
        end

      $scanner_directions[depth] = reverse
    end
  end
end

def tick_to delay
  $scanner_ranges.each do |depth, range|

  end
end

input 2017, 13 do |input|
  $scanner_ranges = {}

  input.lines.each do |line|
    depth, range = line.split ": "

    $scanner_ranges[Integer(depth)] = Integer range
  end

  $scanner_positions = {}
  $scanner_directions = {}

  severity = 0

  layers = $scanner_ranges.keys.max + 1

  (1...Infinity).find do |delay|
    reset

    delay.times do
      tick
    end

    layers.times do |layer|
      scanner_position = $scanner_positions[layer]

      if scanner_position&.zero? then
        p delay: delay, scanned: layer
        break false
      end

      tick

      true
    end
  end
end
