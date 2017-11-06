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

  def input day
    url = URI "https://adventofcode.com/2016/day/#{day}/input"

    fetch url do |res|
      yield res.body
    end
  end
end

module AOC
  INPUT = AOC::Input.new

  def input day, &block
    INPUT.input day, &block
  end
end

include AOC

