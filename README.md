# ytuwu

A youtube downloader written in rust

example usage for downloading video, audio and thumbnail and printing basic metadata like the author: 
```rs 
let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";
let id_collection = IdCollection::from_url(media_url)?;

let downloader = Downloader::new();
let media = downloader
    .download_dual_media_stream(
        id_collection.get_id()?, 
        ShortVideoItag::highest(), 
        AudioItag::highest(), 
        ThumbnailResolution::Low
        )
    .await?;
let path = Path::new("teehee");
println!("title: {}", media.metadata.title);
media.save(&path)?;

```


download a short: 
```rs 
let short_url = "https://youtube.com/shorts/any_short";
let id_collection = IdCollection::from_url(short_url)?;

let downloader = Downloader::new();
let media = downloader
    .download_short(
        id_collection.get_id()?, 
        ShortVideoItag::highest(), 
        AudioItag::highest(), 
        ThumbnailResolution::Low
        )
    .await?;
let path = Path::new("teehee");
media.save(&path)?;

```

download a playlist: 
```rs 
let playlist_url = "https://music.youtube.com/playlist?list=OLAK5uy_nVY7Ekmu-3gJilFDUz8xrjkzmVmVnQSMQ";
let id_collection = IdCollection::from_url(playlist_url)?;

let downloader = Downloader::new();
let media = downloader
    .download_full_playlist(
        id_collection.get_id()?, 
        AudioItag::highest(), 
        ThumbnailResolution::Low
        )
    .await?;
let path = Path::new("teehee");
println!("title: {}", media.metadata.title);
media.save(&path)?;
```

download a channel: 
```rs
let channel_url = "https://music.youtube.com/browse/MPADUC6Tg7GWjZw48EiZ8m5bRtWg";

let id_collection = IdCollection::from_url(channel_url)?;

let downloader = Downloader::new();
let downloaded_channel = downloader.download_channel(id_collection.get_id()?, AudioItag::AacLow).await?;
let path = Path::new("teehee");
downloaded_channel.save(path)?;

```

## features: 
- bypass the "captcha"
- download audio and video streams (in chunks)
- download thumbnails
- download full playlists
- download short
- most of the metadata
- download full channels (just one stream rn)

## todos: 
- better file save handling 
- rate limiter
- more metadata
- save chunks when crashing while downloading and continue the download afterwards
- cache 
- age verification 
- continuation
