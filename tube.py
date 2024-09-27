import os
import yt_dlp

# Get the Downloads directory of the laptop
downloads_path = os.path.join(os.path.expanduser("~"), "Downloads")

def download_youtube_playlist(playlist_url):
    # Fetch playlist information to get the title
    with yt_dlp.YoutubeDL() as ydl:
        playlist_info = ydl.extract_info(playlist_url, download=False)
        playlist_title = playlist_info.get('title', 'Playlist')
    
    # Create a folder named after the playlist inside the Downloads directory
    playlist_folder = os.path.join(downloads_path, playlist_title)
    os.makedirs(playlist_folder, exist_ok=True)

    # yt-dlp options for download
    ydl_opts = {
        'format': 'best[height<=1080]',  # Downloads the best format available up to 1080p
        'outtmpl': os.path.join(playlist_folder, '%(playlist_index)s - %(title)s.%(ext)s'),
        'noplaylist': False  # Set to True if you want to download a single video
    }

    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        try:
            print(f"Downloading playlist: {playlist_title}...")
            ydl.download([playlist_url])
            print(f"Playlist download completed. Videos saved in {playlist_folder}")
        except Exception as e:
            print(f"Failed to download playlist. Error: {e}")

# Replace with your playlist URL
playlist_url = "https://www.youtube.com/playlist?list=PLFr_jkwUp0higsPRxHQ9v9xysuRnvwkPs"
download_youtube_playlist(playlist_url)
