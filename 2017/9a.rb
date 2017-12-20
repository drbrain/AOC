require_relative "../aoc"

test "{}", 1
test "{{{}}}", 6
test "{{},{}}", 5
test "{{{},{},{{}}}}", 16
test "{<a>,<a>,<a>,<a>}", 1
test "{{<ab>},{<ab>},{<ab>},{<ab>}}", 9
test "{{<!!>},{<!!>},{<!!>},{<!!>}}", 9
test "{{<a!>},{<a!>},{<a!>},{<ab>}}", 3

input 2017, 9 do |input|
  tokens = input.chars

  groups = 0
  depth = 0

  until tokens.empty? do
    case tokens.shift
    when "{" then
      depth += 1
    when "}" then
      depth -= 1
      groups += depth + 1
    when "<" then
      loop do
        case tokens.shift
        when "!" then
          tokens.shift
        when ">"
          break
        end
      end
    when "!" then
      tokens.shift
    end
  end

  groups
end
