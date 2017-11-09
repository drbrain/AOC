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

  def input day, split:
    url = URI "https://adventofcode.com/2016/day/#{day}/input"

    fetch url do |res|
      case split
      when "," then
        fields = res.body.split ","

        fields.each do |field|
          yield field.strip
        end
      else
        raise ArgumentError, "unknown split type #{split}"
      end
    end
  end
end

module AOC
  INPUT = AOC::Input.new

  def input day, **options, &block
    INPUT.input day, **options, &block
  end
end

include AOC

