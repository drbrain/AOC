module AOC
end

require "net/http"
require "pp"

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

  def input year, day
    if File.exist? "#{day}.input" then
      input = File.read "#{day}.input"

      return yield input
    end

    url = URI "https://adventofcode.com/#{year}/day/#{day}/input"

    fetch url do |res|
      input = res.body

      File.write "#{day}.input", input

      return yield input
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
    return unless @tests

    @test = true

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

    @test = false
  end

  def test input, expected
    @tests ||= []
    @tests << [input, expected]
  end

  def test_run?
    @test
  end
end

Infinity = 1.0 / 0

include AOC

