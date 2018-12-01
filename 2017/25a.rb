require_relative "../aoc"

test <<-BLUEPRINT, 3
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
BLUEPRINT

def parse text
  header, *states = text.split "\n\n"

  /Begin in state (.)/ =~ header

  begin_state = $1

  /after (\d+) steps/ =~ header

  steps = Integer $1

  blueprint = {}

  states.each do |blueprint_part|
    state_header, *ifs = blueprint_part.split "If"

    /In state (.)/ =~ state_header

    state_name = $1

    state_blueprint = ifs.map { |iff|
      /the current value is (.)/ =~ iff

      value = Integer $1

      _, *things = iff.strip.split /\n\s*-\s*/

      [value, things]
    }

    blueprint[state_name] = state_blueprint
  end

  return begin_state, steps, blueprint
end

input 2017, 25 do |input|
  begin_state, steps, blueprint = parse input

  program = []
  program << %Q{state  = "#{begin_state}"}
  program << %Q{tape   = Hash.new 0}
  program << %Q{cursor = 0}
  program << nil
  program << "#{steps}.times do"

  program << "  case state"

  blueprint.each do |state, blueprint|
    program << %Q{  when "#{state}" then}

    blueprint.each do |value, instructions|
      program << %Q{    if tape[cursor] == #{value} then} if value == 0
      program << %Q{    else} if value == 1

      instructions.each do |instruction|
        case instruction
        when /Write the value (.)/ then
          program << %Q{      tape[cursor] = #{$1}}
        when /Move one slot to the left/ then
          program << %Q{      cursor -= 1}
        when /Move one slot to the right/ then
          program << %Q{      cursor += 1}
        when /Continue with state (.)/ then
          program << %Q{      state = "#{$1}"}
        end
      end
    end

    program << %Q{    end}
  end

  program << %Q{  end}
  program << %Q{end}
  program << nil
  program << %Q{tape}

  program = program.join "\n"

  tape = eval program

  tape.values.count 1
end
