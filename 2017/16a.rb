require_relative "../aoc"

#test "s1,x3/4,pe/b", "baedc"

input 2017, 16 do |input|
  moves = input.chomp.split ","

  dancers = ("a".."p").to_a

  moves.each do |move|
    case move
    when /^s(\d+)/ then
      size = Integer $1

      dancers.rotate! -size
    when /^p([a-z])\/([a-z])/ then
      a = dancers.index $1
      b = dancers.index $2

      dancers[a], dancers[b] = dancers[b], dancers[a]
    when /^x(\d+)\/(\d+)/ then
      a = Integer $1
      b = Integer $2

      dancers[a], dancers[b] = dancers[b], dancers[a]
    else
      raise "no: #{move}"
    end
  end

  dancers.join
end
