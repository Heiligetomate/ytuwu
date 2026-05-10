curl -s -X POST "https://music.youtube.com/youtubei/v1/browse" \
  -H "Content-Type: application/json" \
  -H "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36" \
  -H "X-YouTube-Client-Name: 28" \
  -H "X-YouTube-Client-Version: 1.60.19" \
  -H "Origin: https://music.youtube.com" \
  -d '{
    "context": {
      "client": {
        "clientName": "ANDROID_VR",
        "hl": "en",
        "gl": "US",
        "timeZone": "UTC",
        "utcOffsetMinutes": 0,
      }
    },

    "browseId": "MPADUC6Tg7GWjZw48EiZ8m5bRtWg",
    "contentCheckOk": true,
    "racyCheckOk": true
    }' > channel.json

