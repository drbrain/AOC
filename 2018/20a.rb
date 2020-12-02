require_relative "../aoc"
require "strscan"

test "^WNE$", 3
test "^ENWWW(NEEE|SSE(EE|N))$", 10
test "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18
test "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23
test "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$", 31

class Branch
  attr_reader :path

  def initialize
    @paths = []
  end

  def << path
    @paths << path
  end

  def min_doors
    @paths.map { |path|
      path.min_doors
    }.min_by { |doors|
      doors.length
    }
  end

  def pretty_print q
    separator = -> {
      q.breakable
      q.text "|"
      q.breakable
    }

    q.group 2, "(", ")" do
      q.seplist @paths, separator do |path|
        q.pp path
      end
    end
  end
end

class Door
  attr_reader :direction

  def initialize direction
    @direction = direction
  end

  def min_doors
    @direction
  end

  def pretty_print q
    q.text @direction
  end
end

class EndAnchor
  def pretty_print q
    q.text "$"
  end

  def min_doors
    ""
  end
end

class Path
  attr_reader :parts

  def initialize
    @parts = []
  end

  def << object
    @parts << object
  end

  def min_doors
    doors = ""

    @parts.each do |part|
      doors << part.min_doors
    end

    doors
  end

  def pretty_print q
    q.group 2 do
      q.seplist @parts do |step|
        q.pp step
      end
    end
  end
end

class StartAnchor
  def pretty_print q
    q.text "^"
  end

  def min_doors
    ""
  end
end

class MapexpParser
  def self.parse input
    parser = new input
    parser.parse
  end

  def initialize input
    @s = StringScanner.new input
  end

  def parse
    path = Path.new

    until @s.eos? do
      case @s.peek 1
      when "^" then
        @s.getch

        path << StartAnchor.new
      when "$" then
        @s.getch

        path << EndAnchor.new

        return path
      when "N", "S", "E", "W" then
        path << parse_door
      when "(" then
        @s.getch

        path << parse_branch
      else
        raise "[BUG] unknown character #{@s.peek}"
      end
    end

    raise "[BUG] input string missing \"$\"?"
  end

  def parse_door
    Door.new @s.getch
  end

  def parse_branch
    branch = Branch.new
    path = Path.new

    until @s.eos? do
      case @s.peek 1
      when "(" then
        @s.getch

        path << parse_branch
      when ")" then
        @s.getch

        branch << path

        return branch
      when "|" then
        @s.getch

        branch << path

        path = Path.new
      when "N", "S", "E", "W" then
        path << parse_door
      else
        raise "[BUG] unknown character #{@s.peek}"
      end

    end

    raise "[BUG] branches not terminated by ) or $"
  end
end

input 2018, 20 do |input|
  path = MapexpParser.parse input

  pp path

  min_doors = path.min_doors

  puts min_doors

  min_doors.length
end

