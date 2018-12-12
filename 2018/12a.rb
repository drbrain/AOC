require_relative "../aoc"

test <<-TEST, 325
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
TEST

class EmptyLeftPot
  include Enumerable

  attr_reader :left
  attr_accessor :right

  def initialize right
    @left  = self
    @right = right
  end

  def apply rules
    new_pots = leftmost_new_pot = EmptyLeftPot.new nil

    each do |pot|
      planted = rules[pot.pattern]

      new_pots = Pot.new planted, pot.index, new_pots, nil
      new_pots.left.right = new_pots
    end

    new_pots.right = EmptyRightPot.new new_pots

    leftmost_new_pot.trim_empty
  end

  def each
    return enum_for __method__ unless block_given?

    pot = self

    until pot.rightmost? do
      yield pot

      pot = pot.right
    end

    yield pot

    nil
  end

  def index
    @right.index - 1
  end

  def inspect
    "[. (#{index})"
  end

  def leftmost?
    true
  end

  def leftmost_pot
    self
  end

  def pattern
    ["...", @right.symbol, @right.right.symbol].join
  end

  def planted?
    false
  end

  def rightmost?
    false
  end

  def rightmost_pot
    @right.rightmost_pot
  end

  def trim_empty
    pot = @right

    until pot.planted? do
      pot = pot.right
    end

    pot.left = self
    @right = pot

    rightmost_pot.trim_empty

    self
  end

  def symbol
    "."
  end

  def to_s
    leftmost_pot.each.map(&:symbol).join
  end

  def value
    0
  end
end

class EmptyRightPot
  attr_accessor :left
  attr_reader :right

  def initialize left
    @left  = left
    @right = self
  end

  def index
    @left.index + 1
  end

  def inspect
    ". (#{index})]"
  end

  def leftmost?
    false
  end

  def pattern
    [@left.left.symbol, @left.symbol, "..."].join
  end

  def planted?
    false
  end

  def rightmost?
    true
  end

  def rightmost_pot
    self
  end

  def symbol
    "."
  end

  def to_s
    leftmost_pot.to_s
  end

  def trim_empty
    pot = @left

    until pot.planted? do
      pot = pot.left
    end

    pot.right = self
    @left = pot

    self
  end

  def value
    0
  end
end

class Pot
  attr_accessor :left
  attr_accessor :right

  def self.from_pots pot_string
    left_pot = pots = EmptyLeftPot.new nil

    pot_string.each_char.with_index do |pot, index|
      planted = pot == "#"

      pots = Pot.new planted, index, pots, nil
    end

    pots.right = EmptyRightPot.new pots

    left_pot
  end

  def initialize planted, index, left, right
    @planted = planted
    @index   = index
    @left    = left
    @right   = right

    @left.right = self
  end

  def index
    return @index if @index

    @index =
      if EmptyLeftPot === @left then
        @right.index - 1
      else
        @left.index + 1
      end
  end

  def inspect
    "#{symbol} (#{index})"
  end

  def leftmost?
    false
  end

  def leftmost_pot
    pot = self

    until pot.leftmost? do
      pot = pot.left
    end

    pot
  end

  def pattern
    [
      @left.left.symbol,
      @left.symbol,
      symbol,
      @right.symbol,
      @right.right.symbol
    ].join
  end

  def planted?
    @planted
  end

  def rightmost?
    false
  end

  def rightmost_pot
    pot = self

    until pot.rightmost? do
      pot = pot.right
    end

    pot
  end

  def symbol
    if planted? then
      "#"
    else
      "."
    end
  end

  def to_s
    leftmost_pot.to_s
  end

  def value
    return 0 unless planted?

    index
  end
end

##
# I tried to implement this using an Array of characters to track the pots
# along with an offset of where the 0 pot was, but that was too unworkable to
# track where 0 was supposed to be, and I had a bunch of errors using
# #each_cons so I gave up reasonably quickly.
#
# I then implemented this linked-list solution (guessing from the marble
# problem from day 9 that an outrageous number of iterations would be used for
# part B).
#
# I added a bunch of ruby object protocols to make the calculation look easy
# including Enumerable, #to_s and #inspect.
#
# I don't think this needs to be a doubly-linked list because I always return
# the empty left pot when applying the rules.
#
# The empty left and right pots allow me to avoid a bunch of nil checking
# because they do-the-right-thing when operations are applied to them.  I
# don't think this solution will work for rules like:
#
#   "....#" => "#"
#   "#...." => "#"
#
# because it doesn't synthesize those locations.  Fortunately my input does
# not have them.

input 2018, 12 do |input|
  /initial state: (?<initial_state>[#.]+)/ =~ input

  pots = Pot.from_pots initial_state

  rules = input.lines.select { |line| line.start_with? "#", "." }
  rules = rules.map { |rule|
    pattern, planted = rule.chomp.split " => "

    planted = planted == "#"

    [pattern, planted]
  }.sort.flatten

  rules = Hash[*rules]
  rules.default = false

  20.times do
    pots = pots.apply rules
  end

  pots.map(&:value).sum
end
