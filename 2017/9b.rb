require_relative "../aoc"

test "<>", 0
test "<random characters>", 17
test "<<<<>", 3
test "<{!>}>", 2
test "<!!>", 0
test "<!!!>>", 0
test "<{o\"i!a,<{i<a>", 10


input 2017, 9 do |input|
  tokens = input.chars

  garbage = 0

  until tokens.empty? do
    case tokens.shift
    when "{" then
    when "}" then
    when "<" then
      loop do
        case tokens.shift
        when "!" then
          tokens.shift
        when ">"
          break
        else
          garbage += 1
        end
      end
    when "!" then
      tokens.shift
    end
  end

  garbage
end
