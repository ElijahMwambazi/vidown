# Simple Video Downloader - Development Plan

A minimal, focused video downloader with clean terminal UI using **Rust**, **yt-dlp**, and **aria2**.

## Project Goals

Build a simple, functional downloader that:

- Takes a URL input
- Shows available video/audio formats
- Lets user select format
- Downloads with progress indication
- Clean terminal interface

## Core Features

- **URL Input**: Paste any video URL
- **Format Selection**: Interactive menu of available formats
- **Audio Download**: MP3/M4A extraction options
- **Progress Display**: Real-time download progress
- **Clean TUI**: Simple, intuitive terminal interface

## Development Plan

### Phase 1: Basic Structure (Week 1)

**Goal: Get the foundation working**

#### Tasks:

- [ ] Set up Rust project with basic dependencies
- [ ] Create simple CLI that accepts URL input
- [ ] Integrate yt-dlp to extract video information
- [ ] Display basic video info (title, duration, available formats)
- [ ] Test with popular sites (YouTube, etc.)

#### Key Dependencies:

```toml
[dependencies]
clap = "4.0"
tokio = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

#### Deliverable:

CLI that shows "Here's what I can download from this URL"

---

### Phase 2: Format Selection (Week 2)

**Goal: Interactive format chooser**

#### Tasks:

- [ ] Parse yt-dlp format output into structured data
- [ ] Create interactive format selection menu
- [ ] Group formats by type (video+audio, video-only, audio-only)
- [ ] Show format details (resolution, file size, codec)
- [ ] Handle user selection input
- [ ] Add audio-only extraction options (MP3, M4A)

#### Additional Dependencies:

```toml
dialoguer = "0.10"  # For interactive prompts
console = "0.15"    # For terminal styling
```

#### Deliverable:

Interactive menu: "Select format: [1] 1080p MP4 [2] 720p MP4 [3] Audio MP3"

---

### Phase 3: Download Implementation (Week 2-3)

**Goal: Actually download the selected content**

#### Tasks:

- [ ] Integrate aria2 for downloading
- [ ] Implement download progress tracking
- [ ] Handle download errors and retries
- [ ] Show download speed and ETA
- [ ] Clean up temporary files
- [ ] Post-process audio files if needed (FFmpeg integration)

#### Additional Dependencies:

```toml
indicatif = "0.17"  # Progress bars
```

#### Deliverable:

Working downloader with progress: "Downloading... ████████░░ 80% 5.2MB/s ETA 0:30"

---

### Phase 4: Polish & Error Handling (Week 3-4)

**Goal: Make it reliable and user-friendly**

#### Tasks:

- [ ] Better error messages and handling
- [ ] Input validation (URL checking)
- [ ] Configuration file for default preferences
- [ ] Output filename handling and conflicts
- [ ] Clean, colored terminal output
- [ ] Help text and usage examples

#### Deliverable:

Production-ready tool with good UX

---

## Project Structure

```
simple-video-downloader/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs           # CLI entry point
│   ├── extractor.rs      # yt-dlp integration
│   ├── downloader.rs     # aria2 integration
│   ├── formats.rs        # Format parsing and selection
│   ├── progress.rs       # Progress display
│   └── config.rs         # Configuration handling
```

## Minimal Cargo.toml

```toml
[package]
name = "simple-video-downloader"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
tokio = { version = "1.0", features = ["rt-multi-thread", "process", "fs"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
dialoguer = "0.10"
console = "0.15"
indicatif = "0.17"
```

## User Experience Flow

```
$ video-dl

Enter video URL: https://youtube.com/watch?v=example
Analyzing... ✓

📺 "Cool Video Title" (5:23)
Available formats:

Video + Audio:
  [1] 1080p MP4 (125MB) - Best quality
  [2] 720p MP4 (78MB)   - Good quality
  [3] 480p MP4 (45MB)   - Lower quality

Audio Only:
  [4] MP3 128kbps (5MB)
  [5] M4A 256kbps (8MB) - Best audio

Select format [1-5]: 1

Downloading to: Cool_Video_Title.mp4
████████████████████████████████ 100% 5.2MB/s

✅ Download complete!
```

## Prerequisites

- Rust toolchain installed
- yt-dlp in PATH: `pip install yt-dlp`
- aria2 in PATH: Download from https://aria2.github.io/
- FFmpeg in PATH (for audio conversion): https://ffmpeg.org/

## Quick Start

```bash
# Install prerequisites
pip install yt-dlp
# Install aria2 (varies by OS)
# Install FFmpeg (varies by OS)

# Create project
cargo new simple-video-downloader
cd simple-video-downloader

# Add dependencies to Cargo.toml (see above)

# Start with Phase 1 implementation
cargo run
```

## Success Criteria

- Works with major sites (YouTube, Vimeo, etc.)
- Downloads complete without corruption
- Progress indication works correctly
- Audio extraction produces playable files
- Handles common errors gracefully
- Clean, intuitive interface

## Total Timeline: 3-4 weeks

This focused approach gets you a working, useful tool quickly without feature creep. You can always add more advanced features later once the core functionality is solid.

---
