# ytuwu

A youtube downloader written in rust

example usage for downloading video, audio and thumbnail and printing basic metadata like the author: 
```rs 
    let media_url = "https://music.youtube.com/watch?v=lndG8BiZCmM";

    let id_collection = IdCollection::from_url(media_url);

    let downloader = Downloader::new();
    if let Some(ids) = id_collection {
        let media = downloader.download_dual_media_stream(
            ids.video_id.ok_or(anyhow!("no video id found"))?, 
            VideoItag::MP4240p,
            AudioItag::highest(), 
            ThumbnailResolution::Low,
        ).await?;
        let path = Path::new("teehee");
        media.save(&path)?;
        println!("{}", media.metadata.author);
    } else {
         println!("no ids found");
    }
```


## features: 
- bypass the "captcha"
- download audio and video streams (in chunks)
- download thumbnails
- download full playlists
- most of the metadata

## todos: 
- rate limiter
- channel/artist browse and downlaod
- more metadata
- save chunks when crashing while downloading and continue the download afterwards
- cache 
