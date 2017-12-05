module AOC
end

require "net/http"

class AOC::Input
  def fetch uri
    req = Net::HTTP::Get.new uri
    req["cookie"] = ENV.fetch "AOC_COOKIE"

    http.request req do |res|
      raise "error: #{res.code}" unless Net::HTTPOK === res

      yield res
    end
  end

  def http
    @http =
      begin
        http = Net::HTTP.new "adventofcode.com", 443
        http.use_ssl = true
        http
      end
  end

  def input year, day, split: nil
    url = URI "https://adventofcode.com/#{year}/day/#{day}/input"

    fetch url do |res|
      input = res.body

      result =
        case split
        when "," then
          fields = input.split ","

          fields.each do |field|
            yield field.strip
          end
        when nil then
          yield input
        else
          raise ArgumentError, "unknown split type #{split}"
        end

      return result
    end
  end
end

module AOC
  INPUT = AOC::Input.new

  def input year, day, **options, &block
    run_tests &block

    result = INPUT.input year, day, **options, &block

    puts result
  end

  def run_tests
    @tests.each do |input, expected|
      result = yield input

      abort <<-FAILED unless result == expected
For input part 1:

#{input}

Expected:

#{expected}

Actual:

#{result}
      FAILED

      puts "✅ #{input} → #{result}"
    end
  end

  def test input, expected
    @tests ||= []
    @tests << [input, expected]
  end
end

Infinity = 1.0 / 0

include AOC

