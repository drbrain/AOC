require_relative "../aoc"

test "9", "5158916779"
test "5", "0124515891"
test "18", "9251071085"
test "2018", "5941429882"

$recipes = [3, 7]
$elves   = [0, 1]

def digits_of number
  num = number
  digits = []

  until num < 10 do
    num, den = num.divmod 10
    digits << den
  end

  digits << num

  digits.reverse
end

def recipes_after recipe_number
  until $recipes.length >= recipe_number + 11
    new_recipes_sum = $elves.map { |elf|
      $recipes[elf]
    }.sum

    $recipes.concat digits_of new_recipes_sum

    $elves = $elves.map { |elf|
      offset = 1 + elf + $recipes[elf]
      offset % $recipes.length
    }
  end

  $recipes[recipe_number, 10]
end

##
# It took some effort to figure out what the description of this problem
# wanted.
#
# I thought caching might be helpful in this one so I use cached the recipes
# so I wouldn't have to regenerate them (possibly in part B) but this didn't
# happen.
#
# The #digits_of method is unnecessary because 9 + 9 == 18 and checking if the
# sum is greater than 0 is easier.
#
# I also thought the number of elves could expand in part B so I used an elves
# array instead of two variables.

input 2018, 14 do |input|
  after = Integer input, 10 # some inputs are 0-padded

  recipes = recipes_after after

  recipes.join
end
