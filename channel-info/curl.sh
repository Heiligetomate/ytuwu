curl -s -X POST "https://music.youtube.com/youtubei/v1/browse?prettyPrint=false" \
  -H "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36" \
  -H "Content-Type: application/json" \
  -H "X-YouTube-Client-Name: 67" \
  -H 'X-Youtube-Client-Version: 1.20260428.11.00' \
  -H "Origin: https://music.youtube.com" \
  -d '{
    "context": {
      "client": {
        "hl": "en",
        "gl": "US",
        "userAgent":"Mozilla/5.0 (X11; Linux x86_64; rv:149.0) Gecko/20100101 Firefox/149.0,gzip(gfe)",
        "clientName":"WEB_REMIX",
        "clientVersion":"1.20260428.11.00",
        "utcOffsetMinutes": 0,
      }
    },

    "browseId": "UCGbyt2Lm2WlWULsMuhMeZvg",
    }' > channel.json

