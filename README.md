# renameClips

rename twitch clips automacally!

This program has been released for Windows, Linux and MacOS.
 
  |       Supported OS(CPU)      |   Test(v0.1.1)  | 
  |:---------------------------: | :-------------: |
  |   MacOS(ventura x64)         |        O        |
  |  MacOS(ventura AppleSilicon) |        X        |
  |      Windows(10 x64)         |        O        |
  |   Linux(Ubuntu x64)          |        O        |
  
### Here are the sample video files and JSON file for testing.
  - [sampleVideos.zip](https://github.com/ppugend/renameClips/files/10037747/sampleVideos.zip)


### How to use
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


### Screenshot of ANZ Streamer Directory
  <img width="726" alt="image" src="https://user-images.githubusercontent.com/13452294/203360685-584503dc-3a29-4e68-8e00-4e26224df900.png">


### How to build it(for Developer using Intel MacOS)
 * v0.1.1 was built with 'rustc 1.65.0(897e37553 2022-11-02)', and will always be built using the latest version of rustc.

 * Install Rust
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
 <del>
 * Add target to '$[project]/.cargo/config/' file  
   [target.x86_64-unknown-linux-musl]
   linker = "x86_64-linux-musl-gcc"
   [target.x86_64-pc-windows-gnu]
   linker = 'x86_64-w64-mingw32-gcc'
 </del>
 
 * Build for MacOS(Intel)
   * cargo build --release --target=x86_64-apple-darwin

 * Build for MacOS(AppleSilicon)
   * rustup target add  aarch64-apple-darwin
   * cargo build --release --target=aarch64-apple-darwin

 * Build for Windows x64
   * brew install mingw-w64    
   * rustup target add x86_64-pc-windows-gnu
   * cargo build --release --target=x86_64-pc-windows-gnu
   
 * Build for Linux x64
   * rustup toolchain install stable-x86_64-unknown-linux-musl 
   * rustup target add x86_64-unknown-linux-musl
   * brew install FiloSottile/musl-cross/musl-cross
   * cargo build --release --target=x86_64-unknown-linux-musl
   