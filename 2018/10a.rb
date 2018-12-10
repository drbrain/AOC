require_relative "../aoc"
require "scanf"

test <<-INPUT, 3
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
INPUT

class Canvas

  def initialize points
    @points = points
  end

  def area
    min_col, max_col, min_row, max_row = dimension

    (max_col - min_col) * (max_row - min_row)
  end

  def dimension
    row, col = @points.first.first
    min_row = max_row = row
    min_col = max_col = col

    @points.each do |(row, col),|
      min_row, max_row = [min_row, max_row, row].minmax
      min_col, max_col = [min_col, max_col, col].minmax
    end

    [min_col, max_col, min_row, max_row]
  end

  def tick
    self.class.new @points.map { |(row, col), (vrow, vcol)|
      [[row + vrow, col + vcol], [vrow, vcol]]
    }
  end

  def to_s
    canvas = Hash.new { |h, col| h[col] = Hash.new " " }

    @points.each do |(row, col),|
      canvas[col][row] = "#"
    end

    min_col, max_col, min_row, max_row = dimension

    (min_col..max_col).map { |col|
      row_hash = canvas[col]

      (min_row..max_row).map { |row|
        row_hash[row]
      }.join
    }.join "\n"
  end

end

##
# I decided to make the Canvas class to make picturing the result easier.  I
# started trying to print out the whole canvas every step using an Array, but
# missed the negative coordinates.
#
# To account for negative coordinates I switched to the Hash and the first run
# against the real dataset took several seconds displaying nothing before I
# hit ⌃C.
#
# I next added Canvas#dimension to see just how big the arrays to print were
# getting, and they were very, very big.  Instead of printing them I guessed I
# needed to run it until it got small enough to be printable so I added
# Canvas#area and printed the seconds taken and the size.
#
# After finding the second with the smallest size I printed the canvas and got
# the message.  Some extra work was done to clean this up to make it automatic
# after submitting the answers.
#
# Since there was no real difference between the implementation for parts A
# and B they are the same file.

input 2018, 10 do |input|
  points = input.strip.lines.map { |line|
    /position=<\s* (?<row>-?\d+) ,\s* (?<col>-?\d+) >
     \s+
     velocity=<\s* (?<vrow>-?\d+) ,\s* (?<vcol>-?\d+) >
    /x =~ line

    row = Integer row
    col = Integer col
    vrow = Integer vrow
    vcol = Integer vcol

    [[row, col], [vrow, vcol]]
  }

  canvas = Canvas.new points

  area = canvas.area

  seconds_taken = 0.upto(Infinity) { |second|
    new_canvas = canvas.tick

    new_area = new_canvas.area

    break second if new_area > area

    area   = new_area
    canvas = new_canvas
  }

  puts canvas

  seconds_taken
end
