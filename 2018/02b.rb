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
