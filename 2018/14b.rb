require_relative "../aoc"

test "01245", 5
test "51589", 9
test "92510", 18
test "59414", 2018

def count_recipes_before recipe_section
  match_size = recipe_section.length
  last_size = match_size + 1

  recipes = {
    0 => 3,
    1 => 7,
  }

  recipes_size = 2

  elf_a = 0
  elf_b = 1

  while true do
    recipe_a = recipes[elf_a]
    recipe_b = recipes[elf_b]

    sum = recipe_a + recipe_b

    if sum > 9 then
      recipes[recipes_size] = 1
      recipes[recipes.size] = sum - 10
    else
      recipes[recipes_size] = sum
    end

    recipes_size = recipes.size

    elf_a = (1 + elf_a + recipe_a) % recipes_size
    elf_b = (1 + elf_b + recipe_b) % recipes_size

    section_a = (recipes_size - match_size - 1..recipes_size - 2).map { |i|
      recipes[i]
    }

    break recipes.size - match_size - 1 if recipe_section == section_a

    section_b = (recipes_size - match_size..recipes_size - 1).map { |i|
      recipes[i]
    }

    break recipes.size - match_size if section_b == recipe_section
  end
end

##
# This one required a lot of optimization attempts.
#
# I memoized digits_of before replacing it with the sum check.
#
# I replaced the elves array with variables to reduce method calls.
#
# I calculated more things only once to reduce duplicate work.
#
# The real speedup came from
# https://old.reddit.com/r/adventofcode/comments/a61ojp/2018_day_14_solutions/ebr3em5/
# which used a Hash to store the recipes instead of an Array to avoid the
# reallocation cost.  This easily offsets the cost of constructing an Array to
# compare the recipe section.

input 2018, 14 do |input|
  section = input.strip.chars.map { |c| Integer c }

  count_recipes_before section
end
