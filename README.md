# ytuwu

A rust lib for using the internal youtube api to downlaod media/playlists/channels

## features: 

- download channels/artists 
- download playlists/albums 
- download videos/songs
- get the most important metadata 
- bypass the captcha
- trait that can be implemented to track the progress (for cli tools) (just for media rn)

## todos: 
- better rate limit handler
- more metadata (maybe)
- save chunks when crashing while downloading and continue the download afterwards
- cache (^ should be included there)
- continuation
- better api would be good i think
- better chunk size (maybe adjust it while running for different internet speed)
- maybe some more features like searching (not sure if i want that here tho)
- logging 
- publish on crates io 
- documentation

## Example usages: 

### download the highest audio stream of a video/song, ignore the thumbnail and save the result: 
```rs 
let ids = IdCollection::from_url("my_awesome_url")?;

let downloader = Downloader::default();

let downloaded = downloader.download_media(ids.get_id()?, AudioItag::Highest, None).await?;

downloaded.save_media_stream(Path::new("my_awesome_path"))?;
```


### download the highest audio and video stream and the hightest thumbnail of a video/song and save the result in an own folder:
```rs 
let ids = IdCollection::from_url("my_awesome_url")?;

let downlader = Downloader::default();

let downloaded = downloader
    .download_media_bundle(
        ids.get_id()?, 
        vec![
            AnyItag::Audio(AudioItag::Highest), 
            AnyItag::LongVideo(VideoItag::Highest)
        ], 
        None
    )
    .await?;

downloaded.save_full(Path::new("my_awesome_path"))?;
```

*The id part and the creation of the downlaoder object will be ignored for the next examples.*

### download the highest audio stream for every song in a full album, ignore the thumbnail but print some metadata: 
```rs 
let downloaded = downloader.download_album(ids.get_id()?, AudioItag::Highest, None).await?;

println!("downlaoded album {} ({} songs)", downloaded.metadata.title, downloaded.metadata.song_count);

downloaded.save_with_dir(Path::new("my_silly_path"))?;
```

### download a full channel and save it: 
```rs
let downloaded = downloader.download_channel(ids.get_id()?, AudioItag::Highest).await?;

downloaded.save(Path::new("my_extra_silly_path"))?;
```


