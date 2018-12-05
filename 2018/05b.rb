require_relative "../aoc"

def react(polymer)
  i = 0

  loop do
    break unless a = polymer[i]
    break unless b = polymer[i + 1]

    if (("a".."z").include?(a) and a.upcase == b) or
       (("A".."Z").include?(a) and a.downcase == b) then
      polymer.delete_at i
      polymer.delete_at i

      i -= 1

      next
    end

    i += 1
  end

  polymer.size
end

test "dabAcCaCBAcCcaDA", 4

##
# The second part involved repeating the reaction once for each unit removal.
# I extracted the reaction to a method so I could call it repeatedly on the
# new inputs.
#
# Because the #react method mutates the input I remembered that I would need
# to duplicate the polymer to avoid trying to react the product of a removal.
#
# The double-uniq looks clumsy but is maybe faster than calling downcase on
# every unit in the polymer.  I didn't check because the runtime is reasonable
# (under eight seconds).
#
# Array#delete is very handy here.
#
# I wonder how a writing a version using a String as the source object
# would perform in comparison as you could perform some regular-expression
# work instead for removing the units.

input 2018, 5 do |input|
  polymer = input.strip.chars

  units = polymer.uniq.map(&:downcase).uniq

  units.map { |unit|
    attempt = polymer.dup

    attempt.delete unit
    attempt.delete unit.upcase

    react attempt
  }.min
end
