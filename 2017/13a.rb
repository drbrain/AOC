require_relative "../aoc"

test <<-INPUT, 24
0: 3
1: 2
4: 4
6: 4
INPUT

input 2017, 13 do |input|
  scanner_ranges = {}

  input.lines.each do |line|
    depth, range = line.split ": "

    scanner_ranges[Integer(depth)] = Integer range
  end

  scanner_positions = {}
  scanner_directions = {}

  scanner_ranges.each_key do |depth|
    scanner_positions[depth] = 0
    scanner_directions[depth] = :+
  end

  severity = 0

  layers = scanner_ranges.keys.max + 1

  layers.times do |layer|
    p layer: layer
    scanner_position = scanner_positions[layer]

    if scanner_position&.zero?
      p scanned: [layer, scanner_ranges[layer]]
      severity += layer * scanner_ranges[layer]
    end

    scanner_positions.each do |depth, position|
      max_position = scanner_ranges[depth] - 1

      direction = scanner_directions[depth]
      new_position = position.send direction, 1
      scanner_positions[depth] = new_position

      if new_position == max_position or new_position.zero?
        reverse =
          if direction == :+
            :-
          else
            :+
          end

        scanner_directions[depth] = reverse
      end
    end

    p severity:  severity
    p ranges___: scanner_ranges
    p positions: scanner_positions
    p direction: scanner_directions
    puts
  end

  severity
end
