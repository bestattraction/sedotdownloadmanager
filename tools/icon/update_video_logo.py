import base64
from pathlib import Path

root = Path(r"F:\Video Maker\Sedot Download Manager")
icon_path = root / "extension" / "icons" / "icon-48.png"
video_path = root / "extension" / "video.js"

icon_b64 = base64.b64encode(icon_path.read_bytes()).decode("ascii")
content = video_path.read_text(encoding="utf-8")

# Replace const LOGO = ...
import re
new_content = re.sub(r'const LOGO = "data:image/png;base64,[^"]+";', f'const LOGO = "data:image/png;base64,{icon_b64}";', content)

video_path.write_text(new_content, encoding="utf-8")
print(f"Updated video.js LOGO with {len(icon_b64)} chars base64")
