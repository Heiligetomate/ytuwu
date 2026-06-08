#!/bin/bash

curl -s -X POST "https://www.youtube.com/youtubei/v1/player" \
  -H "Content-Type: application/json" \
  -H "User-Agent: Mozilla/5.0 (Linux; Android 10; Quest 2) AppleWebKit/537.36 (KHTML, like Gecko) OculusBrowser/32.0.0.3.65 SamsungBrowser/4.3 Chrome/137.0.7151.61 Mobile VR Safari/537.36" \
  -H "X-YouTube-Client-Name: 28" \
  -H "X-YouTube-Client-Version: 1.60.19" \
  -H "Origin: https://www.youtube.com" \
  -d '{
    "context": {
      "client": {
        "clientName": "ANDROID_VR",
        "clientVersion": "1.60.19",
        "deviceMake": "Oculus",
        "deviceModel": "Quest 2",
        "androidSdkVersion": 29,
        "hl": "en",
        "gl": "US",
        "timeZone": "UTC",
        "utcOffsetMinutes": 0,

    "visitorData": "CgtLSnEyM0NyWXRWUSj0oprRBjIKCgJERRIEEgAgWWLfAgrcAjE5LllUPVJ3NlI4RnVURHl5dEx1bFppS3FGWl9PTnpOTlJJMFF0T2tXMnJtUTgxRDY5bXlDLU9LUGgzTDhCVXJ0UmVLaVVWWmlYaUhWNjdkc1M3WDNnNFZnZWxYWHo3M25DTlU5QU1PLVdNUjZmcjU1NGFqUWd4aXg1MFEtRkNQQTIydzZOaGxxeDM1WGl5cWh0WlhwQUFibE12LVBUc1k0TzVSc3ROeks1M0ctMjcwdzlpNGF4MGVlcFJMN1VxbDhLZ19YUE9fcFV6QjNOQzNiN29tR2NFcXkwWFlkN2hVWmpwUHhmb3BZWmxLZUJWcXFnM3U0WnVkMVFiVUtDWDRxMG5HcVBHaENMbzFyY0d6TGdBbFRmZHVtY3BnbE0yclNfS2JXX0FaakpQaHRpLXo4enRlSktuUFJKb1ZtZTFMclpIUHZqZWZyMExCNmN4S3FIUzdTb25yTEZOdw%3D%3D",
      }
    },
    
    "videoId": "feQ9sBdZd4c",
    "contentCheckOk": true,
    "racyCheckOk": true
  }' > res.json

