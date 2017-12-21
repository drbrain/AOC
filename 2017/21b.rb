require_relative "../aoc"

#test <<-RULES, 12
#../.# => ##./#../...
#.#./..#/### => #..#/..../..../#..#
#RULES

def rotations pattern, replacement
  return enum_for __method__, pattern, replacement unless block_given?

  yield pattern,         replacement
  yield pattern.reverse, replacement

  pattern_flipped = pattern.map { |row| row.reverse }

  yield pattern_flipped,         replacement
  yield pattern_flipped.reverse, replacement

  pattern_rotate = pattern_flipped.map { |row|
    row.reverse.chars
  }.transpose.map { |row|
    row.reverse.join
  }

  yield pattern_rotate, replacement
  yield pattern_rotate.reverse, replacement
  yield pattern_rotate.map { |row| row.reverse }, replacement

  pattern_rotate_2 = pattern.map { |row|
    row.chars
  }.transpose.map { |row|
    row.join
  }

  yield pattern_rotate_2.reverse, replacement
end

def grid program, size
  new_grid = []

  (0...program.size).step size do |row|
    new_rows = []

    (0...program.size).step size do |col|
      chunk = size.times.map { |offset|
        program[row + offset][col, size]
      }

      new_chunk = yield chunk

      new_rows << new_chunk
    end

    final_rows = Array.new(size + 1) { "" }

    new_rows.each do |chunk|
      chunk.each_with_index do |row_part, index|
        final_rows[index] << row_part
      end
    end

    new_grid.concat final_rows
  end

  new_grid
end

input 2017, 21 do |input|
  rules = input.lines.map { |line|
    pattern, replacement = line.chomp.split " => "

    pattern     = pattern.split "/"
    replacement = replacement.split "/"

    [pattern, replacement]
  }

  patterns = {}

  rules.each do |pattern, replacement|
    rotations pattern, replacement do |pattern|
      patterns[pattern] = replacement
    end
  end

  program = [
    ".#.",
    "..#",
    "###",
  ]

  18.times do |i|
    puts "iteration: #{i} size: #{program.size}"
    size =
      if 0 == program.size % 2
        2
      else
        3
      end

    program = grid(program, size) { |chunk|
      patterns[chunk]
    }
  end

  program.map { |row|
    row.count "#"
  }.sum
end
