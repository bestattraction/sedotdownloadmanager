# Sedot Download Manager (SDM)

<p align="center">
  <img src="crates/app/assets/icons/app/sdm-128.png" alt="Sedot Download Manager's logo" width="128">
</p>

<h3 align="center">Downloads faster than IDM. Modern, Free, and Open Source.</h3>

<p align="center">🌐 <a href="https://github.com/bestattraction/sedotdownloadmanager"><b>github.com/bestattraction/sedotdownloadmanager</b></a>: the official repository, downloads, and documentation</p>

<p align="center">
  <a href="https://github.com/bestattraction/sedotdownloadmanager/releases/latest"><img src="https://img.shields.io/badge/Download%20for-Windows-2a78d6?style=flat-square&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0id2hpdGUiPjxwYXRoIGQ9Ik0yIDMuNWw4LjUtMS4ydjguMkgyek0xMS41IDIuMkwyMiAuOHY5LjdIMTEuNXpNMiAxMS41aDguNXY4LjJMMiAxOC41ek0xMS41IDExLjVIMjJ2OS43bC0xMC41LTEuNHoiLz48L3N2Zz4%3D" alt="Download for Windows" height="34"></a>
  <a href="https://github.com/bestattraction/sedotdownloadmanager/releases/latest"><img src="https://img.shields.io/badge/Download%20for-Linux-2a78d6?style=flat-square&logo=linux&logoColor=white" alt="Download for Linux" height="34"></a>
</p>

<p align="center">
  🧩 <b>Browser Extension:</b><br>
  Available for Chromium (Chrome, Edge, Brave, Opera, Vivaldi) and Firefox. See <a href="#browser-extension-guide">Browser Extension Guide</a> below.
</p>

<p align="center">
  <a href="https://github.com/bestattraction/sedotdownloadmanager/stargazers"><img src="https://img.shields.io/github/stars/bestattraction/sedotdownloadmanager?style=social&label=Star%20SDM" alt="Star Sedot Download Manager on GitHub" height="28"></a><br>
  <sub>⭐ <b>Like it? Give it a star.</b> SDM has no ads and no marketing budget: stars help others find it.</sub>
</p>

<p align="center">
  Windows 10 and 11 (x64, ARM64, 32-bit) · Linux (Ubuntu, Debian, Mint, Fedora, Arch)<br>
  86 languages · modern readable typography (Inter & Segoe UI) · no ads, no trial, no sign-up
</p>

<table align="center">
  <tr>
    <td align="center" width="210"><img src="docs/images/icons/faster.png" width="76" alt=""><h1>5×</h1><b>Faster</b><br><sub>a 1 GB file: 1 s instead of 5.5 s</sub></td>
    <td align="center" width="210"><img src="docs/images/icons/languages.png" width="76" alt=""><h1>+46</h1><b>More languages</b><br><sub>86 instead of IDM's 40</sub></td>
    <td align="center" width="210"><img src="docs/images/icons/free.png" width="76" alt=""><h1>100%</h1><b>Free & Open Source</b><br><sub>no licence, no trial, ever</sub></td>
    <td align="center" width="210"><img src="docs/images/icons/torrent.png" width="76" alt=""><h1>Torrent</h1><b>Supported fully</b><br><sub>magnet links, .torrent files, tray options</sub></td>
  </tr>
</table>

<p align="center">
  <a href="https://github.com/bestattraction/sedotdownloadmanager/releases/latest"><img src="https://img.shields.io/github/v/release/bestattraction/sedotdownloadmanager?logo=github" alt="Latest release"></a>
  <a href="https://github.com/bestattraction/sedotdownloadmanager/releases"><img src="https://img.shields.io/github/downloads/bestattraction/sedotdownloadmanager/total?logo=github" alt="Downloads"></a>
  <a href="https://github.com/bestattraction/sedotdownloadmanager/stargazers"><img src="https://img.shields.io/github/stars/bestattraction/sedotdownloadmanager?logo=github" alt="GitHub stars"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/bestattraction/sedotdownloadmanager" alt="GPL-3.0 license"></a>
</p>

| 🚀 [Download](#download) | 🧩 [Extension Guide](#browser-extension-guide) | ⚡ [Why switch](#why-switch-from-idm) | 📜 [Help & Docs](docs/README.md) |
| --- | --- | --- | --- |

## Why switch from IDM?

**Same workflow you know. Your downloads finish sooner. And it costs nothing.**

<!-- compare:start -->
| | Sedot Download Manager (SDM) | IDM 6.42 |
| --- | --- | --- |
| Download a big file (1 GB) | **1 second** | 5.5 seconds |
| Download a small file | **instantly** | 3.1 seconds |
| Download over a shaky connection | **1.6 seconds** | 5.4 seconds |
| Wait before a download starts | **instantly** | 3.1 seconds |
| Price | **Free, forever** | Paid, after a 30-day trial |
| Ads, nag screens, sign-up | **None** | Registration needed |
| Open source | **Yes** (GPL-3.0) | No |
<!-- compare:end -->

Nothing new to learn: the toolbar, categories, list of downloads, progress window, and video download hover buttons are exactly where you expect them.

---

## Browser Extension Guide

The **Sedot Download Manager Extension** integrates seamlessly with your web browser to automatically capture downloads, torrent links, and video streams.

### 1. Installation

#### For Chromium Browsers (Google Chrome, Microsoft Edge, Brave, Opera, Vivaldi):
1. Download `sdm-chromium.zip` from [Releases](https://github.com/bestattraction/sedotdownloadmanager/releases) (or find it in your SDM installation folder under `extension/chromium`).
2. Unzip the file to a folder of your choice.
3. Open your browser and navigate to the Extensions management page:
   - Chrome / Brave: `chrome://extensions`
   - Edge: `edge://extensions`
   - Opera: `opera://extensions`
4. Enable **Developer mode** using the toggle in the upper-right corner.
5. Click **Load unpacked** and select the unzipped `chromium` folder.
6. Click the Extensions (puzzle piece) icon in your browser toolbar and pin **Sedot Download Manager**.

#### For Mozilla Firefox:
1. Download `sdm-firefox.xpi` from [Releases](https://github.com/bestattraction/sedotdownloadmanager/releases).
2. Open Firefox and go to `about:debugging#/runtime/this-firefox`.
3. Click **Load Temporary Add-on...** and choose `manifest.json` from the unpacked folder or select `sdm-firefox.xpi`.

---

### 2. Key Extension Features & How to Use

#### 🎥 Video Download Hover Button
- **Hover to Reveal**: When you hover your mouse over a video player on supported platforms (YouTube, Vimeo, TikTok, Instagram, Twitter/X, Facebook, and standard HTML5 web players), a sleek "Download video" badge appears at the top-right corner outside the video frame.
- **Auto-Hide in Fullscreen**: Whenever you enter fullscreen mode, the download button automatically hides so your video viewing is completely uninterrupted.
- **Draggable Position**: Prefer the button in another corner? Simply click and drag the handle of the button to any position on your screen. SDM remembers your preferred position across all websites.
- **Quality & Audio Selector**: Click the dropdown arrow on the button to choose from available video resolutions (e.g., 4K, 1440p, 1080p, 720p, 480p) or download audio-only (MP3 / M4A).
- **Subtitle-Only Download**: Download subtitles alone directly without fetching video streams, or choose to embed them into video downloads.
- **Categorization**: When capturing a video, you can specify the target category (Video, Music, General) directly from the dialog.

#### ⚡ Intercepting Downloads & Alt + Click
- **Automatic Interception**: Clicking file download links (ZIP, RAR, EXE, ISO, PDF, MP4, etc.) automatically routes the transfer to SDM for multi-segmented acceleration.
- **Alt + Click Bypass**: Hold the `Alt` key while clicking a download link to send it directly to SDM.

#### 🖱️ Context Menus
- **Download Link**: Right-click any link, image, or audio/video to select **Download with Sedot Download Manager**.
- **Download All Links**: Right-click anywhere on a webpage and choose **Download all links with SDM** to batch grab multiple files, images, or media items in a single pass.

#### 🧲 BitTorrent & Multi-Indexer Search
- **Built-in Torrent Search**: Search across 6 major public indexers in parallel (**The Pirate Bay, BitSearch, Nyaa.si, Torrents-csv, SolidTorrents, EZTV**) directly inside SDM without opening a browser.
- **Deduplication & Top Seeds**: Results are automatically deduplicated by infohash, keeping the highest seeder/leecher counts across all providers.
- **Auto Public Trackers**: Enhanced with 23 verified high-stability public trackers automatically appended to magnet links and downloads for maximum peer connectivity and download speeds.
- **Direct Magnet Interception**: Clicking `magnet:` links or `.torrent` files sends them directly to SDM's built-in BitTorrent engine without requiring external torrent clients.

---

## Download

On the [Releases page](https://github.com/bestattraction/sedotdownloadmanager/releases/latest), pick the appropriate package for your system:

| File | Description |
| --- | --- |
| **`SedotDownloadManager-...-setup.exe`** | **Recommended for Windows.** Installs SDM, configures browser native messaging host, and sets up shortcuts. |
| `SedotDownloadManager-...-portable.zip` | Portable edition for USB drives or running without installation. All data is kept in a local `data` folder. |
| `SedotDownloadManager-...-x64.zip` | Standalone binaries archive without installer. |
| `sdm-chromium.zip` | Extension package for Chrome, Edge, Brave, and other Chromium browsers. |
| `sdm-firefox.xpi` | Extension package for Mozilla Firefox. |

> [!NOTE]
> If Windows displays *"Windows protected your PC"* on first launch, click **More info**, then **Run anyway** (as community open-source builds are not yet EV code-signed).

---

## Helper Modules (yt-dlp & ffmpeg)

For video extraction and merging of high-resolution video streams:
- SDM automatically checks for missing or updated helper modules every 2 weeks.
- You can manually check for module updates at any time by navigating to **Help → Check for tools updates (yt-dlp, ffmpeg)...**.

---

## Licence & Attribution

Sedot Download Manager is free software licensed under the [GNU General Public License, version 3 (GPL-3.0)](LICENSE) or later.

Built with Rust, egui, librqbit, yt-dlp, and ffmpeg. Bundled icons and artwork retain their respective licenses, detailed in [ATTRIBUTION.md](ATTRIBUTION.md).

<!-- languages:start -->
<p align="center">
<img src="crates/app/assets/flags/za.svg" width="28" height="21" alt="Afrikaans" title="Afrikaans">
<img src="crates/app/assets/flags/al.svg" width="28" height="21" alt="Shqip (Albanian)" title="Shqip (Albanian)">
<img src="crates/app/assets/flags/et.svg" width="28" height="21" alt="አማርኛ (Amharic)" title="አማርኛ (Amharic)">
<img src="crates/app/assets/flags/sa.svg" width="28" height="21" alt="العربية (Arabic)" title="العربية (Arabic)">
<img src="crates/app/assets/flags/am.svg" width="28" height="21" alt="Հայերեն (Armenian)" title="Հայերեն (Armenian)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="অসমীয়া (Assamese)" title="অসমীয়া (Assamese)">
<img src="crates/app/assets/flags/az.svg" width="28" height="21" alt="Azərbaycan (Azerbaijani)" title="Azərbaycan (Azerbaijani)">
<img src="crates/app/assets/flags/bd.svg" width="28" height="21" alt="বাংলা (Bangla)" title="বাংলা (Bangla)">
<img src="crates/app/assets/flags/es-pv.svg" width="28" height="21" alt="Euskara (Basque)" title="Euskara (Basque)">
<img src="crates/app/assets/flags/by.svg" width="28" height="21" alt="Беларуская (Belarusian)" title="Беларуская (Belarusian)">
<img src="crates/app/assets/flags/ba.svg" width="28" height="21" alt="Bosanski (Bosnian)" title="Bosanski (Bosnian)">
<img src="crates/app/assets/flags/bg.svg" width="28" height="21" alt="Български (Bulgarian)" title="Български (Bulgarian)">
<img src="crates/app/assets/flags/es-ct.svg" width="28" height="21" alt="Català (Catalan)" title="Català (Catalan)">
<img src="crates/app/assets/flags/us.svg" width="28" height="21" alt="ᏣᎳᎩ (Cherokee)" title="ᏣᎳᎩ (Cherokee)">
<img src="crates/app/assets/flags/cn.svg" width="28" height="21" alt="简体中文 (Chinese (Simplified))" title="简体中文 (Chinese (Simplified))">
<img src="crates/app/assets/flags/tw.svg" width="28" height="21" alt="繁體中文 (Chinese (Traditional))" title="繁體中文 (Chinese (Traditional))">
<img src="crates/app/assets/flags/hr.svg" width="28" height="21" alt="Hrvatski (Croatian)" title="Hrvatski (Croatian)">
<img src="crates/app/assets/flags/cz.svg" width="28" height="21" alt="Čeština (Czech)" title="Čeština (Czech)">
<img src="crates/app/assets/flags/dk.svg" width="28" height="21" alt="Dansk (Danish)" title="Dansk (Danish)">
<img src="crates/app/assets/flags/nl.svg" width="28" height="21" alt="Nederlands (Dutch)" title="Nederlands (Dutch)">
<img src="crates/app/assets/flags/gb.svg" width="28" height="21" alt="English" title="English">
<img src="crates/app/assets/flags/ee.svg" width="28" height="21" alt="Eesti (Estonian)" title="Eesti (Estonian)">
<img src="crates/app/assets/flags/ph.svg" width="28" height="21" alt="Filipino" title="Filipino">
<img src="crates/app/assets/flags/fi.svg" width="28" height="21" alt="Suomi (Finnish)" title="Suomi (Finnish)">
<img src="crates/app/assets/flags/ca.svg" width="28" height="21" alt="Français (Canada) (French (Canada))" title="Français (Canada) (French (Canada))">
<img src="crates/app/assets/flags/fr.svg" width="28" height="21" alt="Français (France) (French (France))" title="Français (France) (French (France))">
<img src="crates/app/assets/flags/es-ga.svg" width="28" height="21" alt="Galego (Galician)" title="Galego (Galician)">
<img src="crates/app/assets/flags/ge.svg" width="28" height="21" alt="ქართული (Georgian)" title="ქართული (Georgian)">
<img src="crates/app/assets/flags/de.svg" width="28" height="21" alt="Deutsch (German)" title="Deutsch (German)">
<img src="crates/app/assets/flags/gr.svg" width="28" height="21" alt="Ελληνικά (Greek)" title="Ελληνικά (Greek)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="ગુજરાતી (Gujarati)" title="ગુજરાતી (Gujarati)">
<img src="crates/app/assets/flags/il.svg" width="28" height="21" alt="עברית (Hebrew)" title="עברית (Hebrew)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="हिन्दी (Hindi)" title="हिन्दी (Hindi)">
<img src="crates/app/assets/flags/hu.svg" width="28" height="21" alt="Magyar (Hungarian)" title="Magyar (Hungarian)">
<img src="crates/app/assets/flags/is.svg" width="28" height="21" alt="Íslenska (Icelandic)" title="Íslenska (Icelandic)">
<img src="crates/app/assets/flags/id.svg" width="28" height="21" alt="Bahasa Indonesia (Indonesian)" title="Bahasa Indonesia (Indonesian)">
<img src="crates/app/assets/flags/ie.svg" width="28" height="21" alt="Gaeilge (Irish)" title="Gaeilge (Irish)">
<img src="crates/app/assets/flags/it.svg" width="28" height="21" alt="Italiano (Italian)" title="Italiano (Italian)">
<img src="crates/app/assets/flags/jp.svg" width="28" height="21" alt="日本語 (Japanese)" title="日本語 (Japanese)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="ಕನ್ನಡ (Kannada)" title="ಕನ್ನಡ (Kannada)">
<img src="crates/app/assets/flags/kz.svg" width="28" height="21" alt="Қазақ тілі (Kazakh)" title="Қазақ тілі (Kazakh)">
<img src="crates/app/assets/flags/kh.svg" width="28" height="21" alt="ខ្មែរ (Khmer)" title="ខ្មែរ (Khmer)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="कोंकणी (Konkani)" title="कोंकणी (Konkani)">
<img src="crates/app/assets/flags/kr.svg" width="28" height="21" alt="한국어 (Korean)" title="한국어 (Korean)">
<img src="crates/app/assets/flags/la.svg" width="28" height="21" alt="ລາວ (Lao)" title="ລາວ (Lao)">
<img src="crates/app/assets/flags/lv.svg" width="28" height="21" alt="Latviešu (Latvian)" title="Latviešu (Latvian)">
<img src="crates/app/assets/flags/lt.svg" width="28" height="21" alt="Lietuvių (Lithuanian)" title="Lietuvių (Lithuanian)">
<img src="crates/app/assets/flags/lu.svg" width="28" height="21" alt="Lëtzebuergesch (Luxembourgish)" title="Lëtzebuergesch (Luxembourgish)">
<img src="crates/app/assets/flags/mk.svg" width="28" height="21" alt="Македонски (Macedonian)" title="Македонски (Macedonian)">
<img src="crates/app/assets/flags/my.svg" width="28" height="21" alt="Bahasa Melayu (Malay)" title="Bahasa Melayu (Malay)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="മലയാളം (Malayalam)" title="മലയാളം (Malayalam)">
<img src="crates/app/assets/flags/mt.svg" width="28" height="21" alt="Malti (Maltese)" title="Malti (Maltese)">
<img src="crates/app/assets/flags/nz.svg" width="28" height="21" alt="Māori (Maori)" title="Māori (Maori)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="मराठी (Marathi)" title="मराठी (Marathi)">
<img src="crates/app/assets/flags/np.svg" width="28" height="21" alt="नेपाली (Nepali)" title="नेपाली (Nepali)">
<img src="crates/app/assets/flags/no.svg" width="28" height="21" alt="Norsk bokmål (Norwegian Bokmål)" title="Norsk bokmål (Norwegian Bokmål)">
<img src="crates/app/assets/flags/no.svg" width="28" height="21" alt="Norsk nynorsk (Norwegian Nynorsk)" title="Norsk nynorsk (Norwegian Nynorsk)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="ଓଡ଼ିଆ (Odia)" title="ଓଡ଼ିଆ (Odia)">
<img src="crates/app/assets/flags/ir.svg" width="28" height="21" alt="فارسی (Persian)" title="فارسی (Persian)">
<img src="crates/app/assets/flags/pl.svg" width="28" height="21" alt="Polski (Polish)" title="Polski (Polish)">
<img src="crates/app/assets/flags/br.svg" width="28" height="21" alt="Português (Brasil) (Portuguese (Brazil))" title="Português (Brasil) (Portuguese (Brazil))">
<img src="crates/app/assets/flags/pt.svg" width="28" height="21" alt="Português (Portugal) (Portuguese (Portugal))" title="Português (Portugal) (Portuguese (Portugal))">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="ਪੰਜਾਬੀ (Punjabi)" title="ਪੰਜਾਬੀ (Punjabi)">
<img src="crates/app/assets/flags/pe.svg" width="28" height="21" alt="Runasimi (Quechua)" title="Runasimi (Quechua)">
<img src="crates/app/assets/flags/ro.svg" width="28" height="21" alt="Română (Romanian)" title="Română (Romanian)">
<img src="crates/app/assets/flags/ru.svg" width="28" height="21" alt="Русский (Russian)" title="Русский (Russian)">
<img src="crates/app/assets/flags/gb-sct.svg" width="28" height="21" alt="Gàidhlig (Scottish Gaelic)" title="Gàidhlig (Scottish Gaelic)">
<img src="crates/app/assets/flags/rs.svg" width="28" height="21" alt="Српски (ћирилица) (Serbian (Cyrillic))" title="Српски (ћирилица) (Serbian (Cyrillic))">
<img src="crates/app/assets/flags/rs.svg" width="28" height="21" alt="Srpski (latinica) (Serbian (Latin))" title="Srpski (latinica) (Serbian (Latin))">
<img src="crates/app/assets/flags/sk.svg" width="28" height="21" alt="Slovenčina (Slovak)" title="Slovenčina (Slovak)">
<img src="crates/app/assets/flags/si.svg" width="28" height="21" alt="Slovenščina (Slovenian)" title="Slovenščina (Slovenian)">
<img src="crates/app/assets/flags/mx.svg" width="28" height="21" alt="Español (México) (Spanish (Mexico))" title="Español (México) (Spanish (Mexico))">
<img src="crates/app/assets/flags/es.svg" width="28" height="21" alt="Español (España) (Spanish (Spain))" title="Español (España) (Spanish (Spain))">
<img src="crates/app/assets/flags/se.svg" width="28" height="21" alt="Svenska (Swedish)" title="Svenska (Swedish)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="தமிழ் (Tamil)" title="தமிழ் (Tamil)">
<img src="crates/app/assets/flags/ru.svg" width="28" height="21" alt="Татар (Tatar)" title="Татар (Tatar)">
<img src="crates/app/assets/flags/in.svg" width="28" height="21" alt="తెలుగు (Telugu)" title="తెలుగు (Telugu)">
<img src="crates/app/assets/flags/th.svg" width="28" height="21" alt="ไทย (Thai)" title="ไทย (Thai)">
<img src="crates/app/assets/flags/tr.svg" width="28" height="21" alt="Türkçe (Turkish)" title="Türkçe (Turkish)">
<img src="crates/app/assets/flags/ua.svg" width="28" height="21" alt="Українська (Ukrainian)" title="Українська (Ukrainian)">
<img src="crates/app/assets/flags/pk.svg" width="28" height="21" alt="اردو (Urdu)" title="اردو (Urdu)">
<img src="crates/app/assets/flags/cn.svg" width="28" height="21" alt="ئۇيغۇرچە (Uyghur)" title="ئۇيغۇرچە (Uyghur)">
<img src="crates/app/assets/flags/uz.svg" width="28" height="21" alt="Oʻzbek (Uzbek)" title="Oʻzbek (Uzbek)">
<img src="crates/app/assets/flags/es.svg" width="28" height="21" alt="Valencià (Valencian)" title="Valencià (Valencian)">
<img src="crates/app/assets/flags/vn.svg" width="28" height="21" alt="Tiếng Việt (Vietnamese)" title="Tiếng Việt (Vietnamese)">
<img src="crates/app/assets/flags/gb-wls.svg" width="28" height="21" alt="Cymraeg (Welsh)" title="Cymraeg (Welsh)">
</p>
<!-- languages:end -->

## Screenshots

<table>
  <tr>
    <td width="50%"><img src="docs/images/video-button-youtube.webp" alt="The Download video button over a YouTube video in Chrome"><br><sub><b>One click on YouTube</b>: the Download video button appears over the video.</sub></td>
    <td width="50%"><img src="docs/images/video-button-page.webp" alt="The Download video button over a video file playing in Chrome"><br><sub><b>Any video on any page</b>: drag the button where you like, it stays there.</sub></td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/progress.webp" alt="Progress window"><br><sub><b>Watch it fly</b>: every connection's share of the file, live speed and time left.</sub></td>
    <td width="50%"><img src="docs/images/main-dark.webp" alt="Dark mode"><br><sub><b>Dark mode</b> that follows Windows.</sub></td>
  </tr>
  <tr>
    <td><img src="docs/images/add-url.webp" alt="Add URL"><br><sub><b>Add a download</b>: paste a link, or let MDM catch it from your browser.</sub></td>
    <td><img src="docs/images/add-torrent.webp" alt="Add torrent"><br><sub><b>Torrents and magnet links</b>: pick the files you want, skip the rest.</sub></td>
  </tr>
  <tr>
    <td><img src="docs/images/progress-torrent.webp" alt="A torrent's progress window"><br><sub><b>A torrent's progress</b>: its files, peers, trackers and pieces, in tabs.</sub></td>
    <td><img src="docs/images/progress-video.webp" alt="A video's progress window"><br><sub><b>A video's progress</b>: speed and time left while yt-dlp brings it in.</sub></td>
  </tr>
  <tr>
    <td><img src="docs/images/language-picker.webp" alt="Language picker"><br><sub><b>86 languages</b>, chosen by flag on the first start.</sub></td>
    <td><img src="docs/images/arabic-main.webp" alt="Arabic interface"><br><sub><b>Right to left</b> for Arabic, Hebrew, Persian, Urdu and Uyghur.</sub></td>
  </tr>
  <tr>
    <td><img src="docs/images/scheduler.webp" alt="Scheduler"><br><sub><b>Download overnight</b>: queues that start and stop on their own.</sub></td>
    <td><img src="docs/images/options-connection.webp" alt="Options"><br><sub><b>Options</b> where you expect them, every control working.</sub></td>
  </tr>
</table>

## What it does

- **Speeds up every download** by fetching parts of the file at the same time, and keeps
  re-balancing until the last byte.
- **Resumes** after a crash, a restart or a dropped connection, without starting over.
- **Catches downloads from Firefox, Chrome, Edge, Brave, Opera, Vivaldi and Helium** (the
  extension is on Firefox Add-ons and the Chrome Web Store), and gives them back to the browser
  when MDM isn't running.
- **Downloads videos** from web pages (a button appears over the video; playlists, subtitles,
  subtitle-only downloads, embed in video, the sound alone), **image galleries** with gallery-dl,
  and **torrents and magnet links** (built-in multi-indexer search across 6 public providers,
  23 bundled high-stability public trackers with CDN fallback, sequential order, RSS feeds, sharing limits).
- **Many links at once**: the Link grabber sorts them into packages and extracts the archives
  (zip, 7z, rar) when they're in. It takes Click'n'Load from link sites, asking once per site. Debrid accounts (Real-Debrid, AllDebrid, Premiumize, TorBox, Mega-Debrid; experimental) download
  file-host links through your own account.
- **Downloads overnight** with queues and a scheduler that can wake the computer, then shuts it
  down if you like.
- **Through your VPN only**, if you want: hold every download to one network adapter, with a kill
  switch, and choose an encrypted DNS. Each download shows its server's country.
- **From your phone, or on a server**: control MDM from a phone on your home network, or run it
  without a window on a NAS (Docker, Unraid) ([how](docs/server.md)).
- **Works from scripts too**: IDM's command-line switches, a command-line tool, and a local API for other programs ([how](docs/command-line.md)).
- **Works with your AI assistant**: an MCP server lets Claude, Cursor or VS Code list, add, pause and remove your downloads ([set it up](docs/ai-assistants.md)).
- **Leaves you some speed**: limit one download or all of them, only while you're at the computer.
- **Day and dark mode, and themes**: Classic, Native, High contrast, Metro, Adwaita, Breeze,
  Luna, Aero and Aqua, or your own in a text file ([how](docs/themes.md)). Everything larger, up
  to 200 %, with **View → Interface size**. 86 languages, and a portable version for USB sticks.
- **Works from the keyboard and with screen readers** (Narrator, NVDA): every window, dialog and
  menu ([the keys](docs/keyboard-shortcuts.md)).
- **No ads, no trial, no account**. Free and open source.

Step by step: [your first download](docs/guide-first-download.md) ·
[catch every browser download](docs/guide-catch-browser-downloads.md) ·
[download overnight](docs/guide-download-overnight.md) ·
[download a video](docs/guide-video-from-page.md) · [all help pages](docs/README.md)

---

## The benchmark in detail

Both programs downloaded the same files from the same local test server
([labserver](crates/labserver)), with the network trouble each scenario names: seven runs each,
both at their default settings. MDM finished first in **all seven scenarios**; every difference is
statistically significant (Mann-Whitney p < 0.001), and MDM used less CPU in every one.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/bench-time-dark.svg">
    <img src="docs/images/bench-time-light.svg" alt="Time to finish a download in seconds, Mister Download Manager versus IDM, in seven scenarios" width="880">
  </picture>
</p>

On a server that allows only two connections, MDM used 2 and still finished first; IDM opened 480.
The full table, the machine and the raw data: [docs/bench/run5/REPORT.md](docs/bench/run5/REPORT.md).
The harness is in [tools/bench](tools/bench), so anyone can run it again.

## Not a reverse-engineering project

MDM is an independent, clean-room program. It is a new implementation of a familiar way of
working, not a copy of anyone's software.

- **No one else's code or assets.** It contains no code, binaries, artwork, icons, sounds or
  text from Internet Download Manager, nor code from any other download manager. The app icon
  was made for MDM with ChatGPT (OpenAI's image generation); the other icons come from openly
  licensed sets ([ATTRIBUTION.md](ATTRIBUTION.md)).
- **Nothing taken apart.** No IDM file was ever disassembled, decompiled, debugged, patched or
  inspected below its user interface, and its licensing, trial and registration were never
  touched or bypassed.
- **Learned as a user learns.** What MDM does was learned from IDM's public documentation and
  help pages, its public screenshots, and by using a licensed copy as a black box: downloading
  test files from our own test server ([labserver](crates/labserver)) and watching what reached
  the network and the screen.
- **Built on public standards.** The download engine follows the HTTP specifications (RFC 9110,
  RFC 9112) and was designed, tested and measured independently. Its speed comes from its own
  design, which the benchmark shows.
- **Not affiliated.** MDM isn't affiliated with, endorsed by or connected to Tonec Inc.
  "Internet Download Manager" and "IDM" are trademarks of their owner, named here only to
  describe the layout people already know and to compare the two programs.

## For developers

<details>
<summary><b>Build from source</b></summary>

Requires Rust stable 1.98 or later (the MSVC toolchain on Windows), and for the engine's
browser-like connection (BoringSSL, built from source): CMake, LLVM's libclang and, on Windows,
NASM (`winget install Kitware.CMake LLVM.LLVM NASM.NASM`; on Linux `cmake` and
`libclang-dev`; on macOS Xcode's command line tools and `cmake`).
`cargo build -p mdm-app --no-default-features` builds the app without it (a site that refuses MDM's
own connection then stays refused).

```
cargo build --release
cargo test
```

This makes `misterdownloadmanager.exe` (the app), `misterdownloadmanager-cli.exe` (headless
command line), `misterdownloadmanager-mcp.exe` (the MCP server for AI assistants) and
`misterdownloadmanager-nmhost.exe` (the browser host) in `target\release\`.
`tools\installer\build.ps1` builds the installer and both zips into `target\dist\` (needs
[Inno Setup 6.3](https://jrsoftware.org/isinfo.php) or later), for x64; `-Arch arm64` or
`-Arch x86` for Windows on ARM or 32-bit Windows (after `rustup target add
aarch64-pc-windows-msvc` or `i686-pc-windows-msvc`). `tools/installer/build-linux.sh` builds the
Linux files for the machine's own processor (x86-64 or ARM64). A `vX.Y.Z` tag does all of them
on GitHub Actions and publishes the release.

Browser integration from a build: register the native messaging host with
`target\release\misterdownloadmanager-nmhost.exe --register` (undo with `--unregister`), then in
`chrome://extensions` or `edge://extensions` turn on Developer mode, **Load unpacked** and pick
the `extension/` folder (its ID is always `daadellipncgkggmmgaimhcndgblechg`).

</details>

<details>
<summary><b>Layout and tools</b></summary>

A Cargo workspace (edition 2024); [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) explains how the
pieces fit.

| Crate | What it is |
|---|---|
| [`mdm-engine`](crates/engine) | The download engine: segmentation, connections, resume, limits |
| [`mdm-app`](crates/app) | The desktop app (egui), and `--headless` for servers |
| [`mdm-cli`](crates/cli) | Headless downloads from the command line |
| [`mdm-mcp`](crates/mcp) | The MCP server, for AI assistants |
| [`mdm-nmhost`](crates/nmhost) | The browser's native messaging host |
| [`mdm-ipc`](crates/ipc) | Talking to a running app; shared capture rules; the extension's IDs |
| [`mdm-remote`](crates/remote) | The remote control: the HTTPS server and page a paired phone uses |
| [`mdm-crash`](crates/crash) | Crash and freeze reports: the monitor, minidumps, stacks |
| [`mdm-debrid`](crates/debrid) | Debrid accounts through the services' official APIs |
| [`mdm-extract`](crates/extract) | Archive extraction (zip, tar, gz, xz; 7z and RAR through 7-Zip; in place through HatchZip) |
| [`mdm-geo`](crates/geo) | A server's country, from an IP table on this computer |
| [`mdm-tools`](crates/tools) | yt-dlp and ffmpeg for video pages, gallery-dl for galleries |
| [`mdm-torrent`](crates/torrent) | BitTorrent through librqbit |
| [`mdm-labserver`](crates/labserver) | An HTTP test server with fault injection, for tests and benchmarks |

| Tool | What it does |
|---|---|
| `tools\dev\check-cross.ps1` | Checks the code for Linux and macOS from Windows (with [Zig](https://ziglang.org) as the C compiler) |
| `node tools/extension/e2e.mjs` | The browser extension in real Chrome, Brave, Edge and Firefox, headless: a link's download must reach MDM (or another MDM, a server); on Windows and Linux |
| `python tools/i18n/extract.py` | Lists every interface text in `i18n/en.tsv` (CI checks it matches the code) |
| `python tools/i18n/translate.py` | Translates what the catalogs lack (checks every row in code) |
| `python tools/installer/languages.py` | The installer's messages in every language |
| `node tools/readme/build.mjs` | This page's pictures, comparison table and flags, from the benchmark data |
| `tools\lab\docs-shots.ps1` | The screenshots, from a separate test instance (`/show <window>` opens each window), off the screen |
| `python tools/shots/make.py` | The help pages' pictures, from one list (`tools/shots/shots.toml`): shot, cropped, annotated, lossless WebP; `make.py check` keeps them in step |
| `tools/lab/wsl/` | A WSL2 machine that builds MDM for Linux in minutes and shoots its windows on a virtual screen ([README](tools/lab/wsl/README.md)) |
| `tools\lab\progress-shots.ps1` | The progress window of a video (yt-dlp) and of a torrent, from a swarm on this computer (`cargo run -p mdm-torrent --example swarm`) |
| `python tools/website/build.py --serve` | misterdownloadmanager.com into `target/site` (help pages from `docs/`, news from the CHANGELOG, FAQ and legal pages from `website/content/`), served on http://127.0.0.1:8765; GitHub Pages publishes it (`pages.yml`) |
| `python tools/readme/social.py` | The repository's social preview card (`docs/images/social-preview.png`) |
| `MDM_FIT_AUDIT=<file>` | Lists every text that doesn't fit its window or runs over another, for checking a translation |
| `MDM_OFFSCREEN=1` | Opens every window off the screen, without focus, tray icon or sound (screenshots and checks) |
| `MDM_TEST_TRASH_DIR=<folder>` | Tests only: archives deleted after extracting go into this folder, never into the real Recycle Bin or Trash |
| `MDM_TEST_7ZIP=<program>\|none` | Tests only: the 7-Zip program to use for 7z and RAR archives, or `none` to act as if 7-Zip weren't installed |
| `MDM_TEST_HATCHZIP=<program>\|none` | Tests only: the HatchZip program to use (extraction in place), or `none` to act as if HatchZip weren't installed |

</details>

## Status and helping out

MDM is young: the engine, the main window and the browser integration are solid on Windows and
Linux, and some corners are still "good enough for now".

- **Translations are machine-made.** Rows marked `#, machine` in [`i18n/`](i18n) wait for a
  native speaker; fixing one is a one-line pull request ([how](docs/languages.md)). Malay's
  menus, toolbar and download-list columns have been reviewed by a native speaker.
- Not yet: code signing. The macOS builds are untested; on Linux, screen readers (Orca) and
  Flatpak browsers aren't supported yet.

Bug reports and feature requests are welcome in
[Issues](https://github.com/bestattraction/sedotdownloadmanager/issues).

## Licence and credits

Sedot Download Manager is free software under the
[GNU General Public License, version 3](LICENSE) or (at your option) any later version.

Built on [egui](https://github.com/emilk/egui) through
[storytold's forks](https://github.com/storytold/egui) and
[storytold/craft-libs](https://github.com/storytold/craft-libs) (MIT OR Apache-2.0),
[librqbit](https://github.com/ikatson/rqbit) (Apache-2.0),
[zip](https://github.com/zip-rs/zip2) (MIT), and [yt-dlp](https://github.com/yt-dlp/yt-dlp)
(Unlicense, run as a separate program; as are 7-Zip for 7z and RAR archives, and
[HatchZip](https://github.com/xevrion/hatchzip) (MIT OR Apache-2.0) when installed). Flags from
[flag-icons](https://github.com/lipis/flag-icons) (MIT); pictures drawn with
[rough.js](https://roughjs.com/) (MIT) in [Patrick Hand](https://fonts.google.com/specimen/Patrick+Hand)
(SIL OFL) and [Vega-Lite](https://vega.github.io/vega-lite/) (BSD-3). Bundled icons and artwork
keep their own licences, listed in [ATTRIBUTION.md](ATTRIBUTION.md).
