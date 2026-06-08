

- downloader has a download pool containing all browsed videos or video ids that need to be downloaded 
- a playlist puts all songs into this pool 
- a channel resolves all singles to the ids and treats the albums and eps as playlists

- now we have a pool of ids that all can have an id to be assigned to their playlists 
- as soon as the user calls .download(), all songs get downloaded and mapped to their lists/channels at the end 
- using this allows us to limit the concurrently downloading media streams cleanly

example of downloading a playlist: 
- downloader.from_url()?.as_list()? 
=> now we have the playlist builder with the id 
=> .download() gets called
=> playlist gets browsed 
=> we have media object containging video ids and playlist ids 
=> now we have to start the downlaod of the browsed songs 
=> we can call this by putting the browsing of the playlist and the downloading of the queue in one fn 

the pool gets "sorted" after the download is finished which is possible because we have the ids.


browsing and downloading are two different tasks: browsing is per channel/list when it gets called (channel browses all playlists etc before putting them in the downloading pool)
