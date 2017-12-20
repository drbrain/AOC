require_relative "../aoc"

test "flqrgnkx", 1242

def knot input
  lengths = input.chomp.unpack "c*"
  lengths.concat [17, 31, 73, 47, 23]

  list_size = 256

  list = (0...list_size).to_a
  skip_size = 0
  start = 0

  64.times do
    lengths.each do |length|
      reversed = list.take(length).reverse
      rest = list[length, list_size - length]
      list = reversed.concat rest

      total_skip = skip_size + length
      start += total_skip
      skip_size += 1

      list = list.rotate total_skip
    end
  end

  fix = list_size - (start % list_size)

  sparse_hash = list.rotate fix

  dense_hash = sparse_hash.each_slice(16).map do |block|
    block.reduce :^
  end

  dense_hash
end

def make_grid key
  grid = 128.times.map do |row|
    row_key = "#{key}-#{row}"

    knot_hash = knot row_key

    knot_hash.map { |byte| "%08b" % byte }.join.split ""
  end
end

def adjacent coordinate
  x, y = coordinate

  [
                    [x    , y + 1],
    [x - 1, y    ],                 [x + 1, y    ],
                    [x    , y - 1],
  ].select { |x1, y1|
    (0...128).include? x1 and
      (0...128).include? y1 and
      $grid[x1][y1] == "1" and
      not $seen[[x1, y1]]
  }
end

def search coordinate
  todo = [coordinate]

  until todo.empty? do
    $seen[coordinate] = true

    coordinate = todo.shift

    x, y = coordinate

    if $grid[x][y] == "1" then
      $regions[coordinate] = $region_number
      $region_grid[x][y] = ($region_number % 36).to_s 36
    end

    todo.concat adjacent coordinate
    todo.uniq!
  end
end

input 2017, 14 do |input|
  key = input.chomp

  $grid = make_grid key
  $region_grid = Array.new 128 do
    Array.new 128, '.'
  end

  # coordinate => region number
  $seen = {}
  $regions = {}
  $region_number = 0

  128.times do |x|
    128.times do |y|
      next if $regions[[x, y]]
      next if $grid[x][y] == "0"

      search [x, y]

      $region_number += 1
    end
  end

  $region_grid.each do |row|
    puts row.join
  end

  $region_number
end
