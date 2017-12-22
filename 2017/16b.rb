require_relative "../aoc"

#test "s1,x3/4,pe/b", "baedc"

input 2017, 16 do |input|
  moves = input.chomp.split ","

  dancers = ("a".."p").to_a

  moves = moves.map { |move|
    case move
    when /^s(\d+)/ then
      size = Integer $1
      size = -size

      -> { dancers.rotate! size }
    when /^p([a-z])\/([a-z])/ then
      name_a = $1
      name_b = $2

      -> {
        a = dancers.index name_a
        b = dancers.index name_b

        dancers[a], dancers[b] = dancers[b], dancers[a]
      }
    when /^x(\d+)\/(\d+)/ then
      a = Integer $1
      b = Integer $2

      -> {
        dancers[a], dancers[b] = dancers[b], dancers[a]
      }
    end
  }

  seen = {}

  16.times do |i|
    moves.each do |move|
      move.call
    end

    (puts i; exit) if seen[dancers]

    seen[dancers] = true
  end

  dancers.join
end
