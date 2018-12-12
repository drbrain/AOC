require_relative "../aoc"

test <<-TEST, 999999999374
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
    [false, false, false, @right.planted?, @right.right.planted?]
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
    leftmost_pot.map(&:symbol).join
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
    [@left.left.planted?, @left.planted?, false, false, false]
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
    from_pots_at_offset pot_string, 0
  end

  def self.from_pots_at_offset pot_string, offset
    left_pot = pots = EmptyLeftPot.new nil

    pot_string.each_char.with_index do |pot, index|
      planted = pot == "#"

      pots = Pot.new planted, offset + index, pots, nil
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
      @left.left.planted?,
      @left.planted?,
      planted?,
      @right.planted?,
      @right.right.planted?
    ]
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
# As I guessed, the number of iterations become ridiculous.  Fortunately I
# learned from day 11's part B and was prepared.
#
# Printing intermediate progress was slow at first so I optimized the matching
# for rules a bit by using arrays of true/false values instead of strings to
# avoid creating the extra garbage strings from joining the pot symbols.
#
# I also noticed I was getting a lot of extra left and right padding with
# empty pots so I implemented trimming down the ends.
#
# Once these two were in place I could see that a regular pattern formed.
# I figured out how to detect the repeating pattern then figured out how to
# calculate what the final arrangement would look like.
#
# I had an off-by-one error and mistakenly assumed it was due to the
# EmptyLeftPot being included in the output and incorrectly corrected that by
# deleting the empty pot prefix, but I could have reduced the index by one as
# well.
#
# Instead I should have subtracted 1 from the remaining count as the
# pot_string starts at the correct index.

input 2018, 12 do |input|
  /initial state: (?<initial_state>[#.]+)/ =~ input

  pots = Pot.from_pots initial_state

  rules = input.lines.select { |line| line.start_with? "#", "." }
  rules = rules.flat_map { |rule|
    pattern, planted = rule.chomp.split " => "

    planted = planted == "#"
    pattern = pattern.chars.map { |plant|
      plant == "#"
    }

    [pattern, planted]
  }

  rules = Hash[*rules]
  rules.default = false

  index = 50_000_000_000.times do |i|
    old_pots = pots
    pots = pots.apply rules

    break i if pots.to_s == old_pots.to_s
  end

  remaining = 50_000_000_000 - index

  offset = pots.index + remaining

  pot_string = pots.to_s.delete_prefix "."

  pots = Pot.from_pots_at_offset pot_string, offset

  pots.map(&:value).sum
end
