# Installation

Deskulpt is currently available on the following desktop platforms:

- Windows (10 and later)
- MacOS (Catalina 10.15 and later, Intel and Apple Silicon)
- Linux (X11, tested on Ubuntu)

You can download the latest release of Deskulpt from [Github Releases](https://github.com/deskulpt-apps/Deskulpt/releases/latest).

## Troubleshooting

### Windows SmartScreen Warning

Windows SmartScreen might display a warning when you try to download the installer. To bypass this warning, keep clicking "More info" and "Keep this file anyways" until the download starts.

### MacOS Gatekeeper Warning

When you open Deskulpt for the first time on macOS, you might see a warning that the app is "broken" or "unidentified". To resolve this, open a terminal and run the following command:

```bash
xattr -d com.apple.quarantine /Applications/Deskulpt.app
```

Then try opening Deskulpt again.
