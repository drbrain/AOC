require_relative "../aoc"

test <<-INPUT, "fgij"
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
INPUT

##
# This implementation uses +catch+ and +throw+ to break out of nested blocks.
# Using +return+ requires a method.  Extracting a method seemed like more work
# than indenting the code one level and wrapping it in +catch+.
#
# Originally the <code>id.select.with_index</code> used Array#zip, but I
# didn't like that I had to either record the matching characters as a
# side-effect in the comparison block, or repeat the work to get the matching
# letters of the ID.

input 2018, 2 do |ids|
  ids = ids.lines.map { |id|
    id.strip.chars
  }

  catch :done do
    ids.each_with_index do |id, i|
      ((i + 1)...ids.length).each do |j|
        other = ids[j]

        common = id.select.with_index { |a, k|
          a == other[k]
        }

        throw :done, common.join if common.length == id.length - 1
      end
    end
  end
end
