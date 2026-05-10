curl -s -X POST \
  'https://music.youtube.com/youtubei/v1/browse' \
  -H 'Content-Type: application/json' \
  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36' \
  -H 'Origin: https://music.youtube.com' \
  -H 'Referer: https://music.youtube.com/' \
  -d '{
    "browseId": "MPADUC6Tg7GWjZw48EiZ8m5bRtWg",
    "context": {
      "client": {
        "clientName": "WEB_REMIX",
        "clientVersion": "1.20240101.01.00",
        "hl": "en"
      }
    }
  }' > channel.json
