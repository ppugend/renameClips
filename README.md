# renameClips

rename twitch clips automacally!

This program has been released for Windows, Linux and MacOS<del>, but only tested on MacOS. I'll test it out and update the table at the below.</del>
 
  |  Supported OS          |  Test(v0.1.0) | 
  |:---------------------: | :-------------: |
  |   MacOS(ventura x64)   |      O        |
  |      Windows(10)       |      O        |
  |   Linux(Ubuntu x64)    |      O        |
  
Here are the sample video files and JSON file for testing.
  - [sampleVideos.zip](https://github.com/ppugend/renameClips/files/10037747/sampleVideos.zip)


How to use
  1. Disable "Ask where to save each file before downloading" in Internet browser options
  2. Visit [ANZ Streamer Directory](https://www.twitchanz.com/clips)
  3. Click the Login button and log in with your Twitch ID to give twitchanz access to all your video clips
  4. Select the period you want to download the video for
  5. Input the channel ID of the clip you want to download.
  6. Click the Search button and wait for the list of videos to be listed
  7. Click the Batch Download button and wait for all video files to download
  8. Select JSON in Export type
  9. Click the Export Data to download JSON file
  10. Download renameCLips and run.
  11. Select the JSON file you downloaded.
  12. Video file names will be changed along with a guide message.


Screenshot of ANZ Streamer Directory
 ![webscreenshot](https://user-images.githubusercontent.com/13452294/202572154-ec68168b-7efb-4270-9b05-ecdd7fc6c8f1.png)


How to build it(for Developer using Intel MacOs)
 * Add target to '$[project]/.cargo/config/' file
   ```
   [target.x86_64-unknown-linux-musl]
   linker = "x86_64-linux-musl-gcc"

   [target.x86_64-pc-windows-gnu]
   linker = 'x86_64-w64-mingw32-gcc'
   ```

 * Build for Intel MacOs
   * cargo build --release --target=x86_64-apple-darwin

 * Build for Windows x64
   * brew install mingw-w64    
   * rustup target add x86_64-pc-windows-gnu
   * cargo build --release --target=x86_64-pc-windows-gnu
   
 * Build for Linux x64
   * rustup toolchain install stable-x86_64-unknown-linux-musl 
   * rustup target add x86_64-unknown-linux-musl
   * brew install FiloSottile/musl-cross/musl-cross
   * cargo build --release --target=x86_64-unknown-linux-musl
   