require_relative "../aoc"
require "scanf"
require "time"

test <<-TEST, 240
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
# I didn't read the input to notice it was not in-order despite the
# instructions hinting that it was unordered ("which have already been
# organized into chronological order") and lost a few minutes to that.  It
# manifested first as +guard_id+ being nil then as +sleep_start+ being nil.  I
# finally noticed the timestamps after noting my input has guards repeatedly
# waking up.
#
# The <code>sort_by { |timestamp,| timestamp }</code> isn't necessary, but I
# was worried about actions getting out-of-order if there were two timestamps
# for the same minute.  Reading the actions now, it is unnecessary because
# "G" from "Guard #" > "f" from "falls asleep" > "w" from "wakes up" thus order
# will be preserved for identical timestamps.
#
# I tried a few strategies for accounting by minute like the schedule uses
# instead of second (which Time uses internally).  First I created minute
# timestamps at parse-time but switched to calculating minutes timestamps
# inside the "wakes up" case after adding the +sleep_schedule+ Hash.  This
# allowed me to use Time#min to mark how many times a guard was asleep that
# minute in +sleep_schedule+.
#
# Initially I thought I needed to account for "guards count as asleep on the
# minute they fall asleep […] awake on the minute they wake up" by subtracting
# one minute, but this wasn't necessary.
#
# I think, because the timestamps are before the +time_t+ epoch Time uses,
# that I had to reverse the start and end timestamps for calculating the
# minutes asleep.  My math + logic is currently spent.
#
# I misspelled "guard" as "gaurd" so, so, so many times.

input 2018, 4 do |input|
  ordered = input.lines.map { |line|
    /\[(?<timestamp>.*?)\] (?<action>.*)/ =~ line

    timestamp = Time.parse timestamp

    [timestamp, action]
  }.sort_by { |timestamp,| timestamp }

  # guard_id => minutes sleeping
  minutes_asleep = Hash.new 0
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
      a = sleep_start.to_i / 60
      b = timestamp.to_i / 60
      minutes_asleep[guard_id] += (b - a)

      (sleep_start.min...timestamp.min).each do |minute|
        sleep_schedule[guard_id][minute] += 1
      end
    else
      raise "unknown action #{action}"
    end
  end

  most_sleeping_id =
    minutes_asleep.max_by { |_, minutes_asleep| minutes_asleep }.first

  guard_schedule = sleep_schedule[most_sleeping_id]
  max_sleeps = guard_schedule.max
  max_sleep_moment = guard_schedule.index max_sleeps

  max_sleep_moment * most_sleeping_id
end
