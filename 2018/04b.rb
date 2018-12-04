require_relative "../aoc"
require "scanf"
require "time"

test <<-TEST, 4455
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
TEST

##
# This solution doesn't need +minutes_asleep+ at all, so I removed it after
# I calculated the solution.
#
# For some reason I forgot the previous solution and calculated the
# most-asleep-minute with the clumsy:
#
#   schedule.each.with_index.max_by {...}.last
#
# Instead of using what I did in the previous solution to calculate the
# max-sleep-moment:
#
#   schedule.index schedule.max
#
# And I didn't notice until I was writing up these notes.  It must be all the
# uses of #max and #max_by for calculating strategy number 2.  I'm leaving it
# as a monument to overthinking things.

input 2018, 4 do |input|
  ordered = input.lines.map { |line|
    /\[(?<timestamp>.*?)\] (?<action>.*)/ =~ line

    timestamp = Time.parse timestamp

    [timestamp, action]
  }.sort_by { |timestamp,| timestamp }

  # guard_id => times asleep in that minute of the hour
  sleep_schedule = Hash.new { |h, guard_id| h[guard_id] = Array.new 60, 0 }

  guard_id    = nil
  sleep_start = nil

  ordered.each do |timestamp, action|
    case action
    when /Guard #(\d+)/ then
      guard_id = Integer $1
    when "falls asleep" then
      sleep_start = timestamp
    when "wakes up" then
      (sleep_start.min...timestamp.min).each do |minute|
        sleep_schedule[guard_id][minute] += 1
      end
    else
      raise "unknown action #{action}"
    end
  end

  most_often_asleep_id, schedule =
    sleep_schedule.max_by { |guard_id, schedule|
      schedule.max
    }

  most_asleep_minute =
    schedule.each.with_index.max_by { |minute,| minute }.last

  most_often_asleep_id * most_asleep_minute
end
