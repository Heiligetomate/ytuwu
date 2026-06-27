use ytuwu::{
    id_resolver::Id,
    itags::{ThumbRes, VideoItag},
    types::VideoId,
};

#[tokio::main]
async fn main() -> ytuwu::Result<()> {
    let dwnld = ytuwu::Downloader::default();
    dwnld
        .download_media(VideoId::new("asd")?, VideoItag::MP4720p, Some(ThumbRes::Medium))
        .await?;
    Ok(())
}
