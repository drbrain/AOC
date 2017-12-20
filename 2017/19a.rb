require_relative "../aoc"

test <<-ROUTE, "ABCDEF"
     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 

ROUTE

input 2017, 19 do |input|
  grid = input.lines.map { |line| line.chomp }

  row = 0
  col = grid.first.index "|"
  dir = :down

  path = ""

  loop do
    current = grid[row][col]

    p current => [row, col, dir]

    case current
    when "|" then
      case dir
      when :down  then row += 1
      when :left  then col -= 1
      when :right then col += 1
      when :up    then row -= 1
      end
    when "-" then
      case dir
      when :down  then row += 1
      when :left  then col -= 1
      when :right then col += 1
      when :up    then row -= 1
      end
    when /[A-Z]/ then
      path << current

      case dir
      when :down  then row += 1
      when :left  then col -= 1
      when :right then col += 1
      when :up    then row -= 1
      end
    when "+" then
      case dir
      when :down, :up then
        case grid[row][col + 1]
        when "-", /[A-Z]/ then
          col += 1
          dir = :right
        else
          col -= 1
          dir = :left
        end
      when :left, :right then
        case grid[row + 1][col]
        when "|", /[A-Z]/ then
          row += 1
          dir = :down
        else
          row -= 1
          dir = :up
        end
      end
    when " " then
      break path
    else
      raise "unknown char #{current}"
    end
  end
end
