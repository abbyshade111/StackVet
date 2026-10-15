# StackVet logo files

The code-bracket logo in terracotta: a check mark inside square brackets, and the name in JetBrains Mono Bold
with "vet" in the accent color. The letters are drawn as outlines, so the logo looks the same everywhere,
whether or not the font is installed. JetBrains Mono is free to use under the SIL Open Font License.

| File | What it is | Where it goes |
| --- | --- | --- |
| `stackvet-logo.svg`, `stackvet-logo.png` | The full logo, for light backgrounds | The top of the README |
| `stackvet-logo-dark.svg`, `stackvet-logo-dark.png` | The full logo, for dark backgrounds | The README in dark mode, slides |
| `stackvet-mark.svg`, `stackvet-mark-512.png` | The bracket mark alone, for light backgrounds | Avatars, small spaces |
| `stackvet-mark-dark.svg`, `stackvet-mark-dark-512.png` | The bracket mark alone, for dark backgrounds | The same, on dark |
| `favicon.svg`, `favicon-32.png` | The mark on a light rounded square | The browser tab of stackvet.dev and stackvet.app |
| `apple-touch-icon.png` (180 x 180) | The same, larger | A phone's home screen, if someone saves the site |
| `social-preview.png` (1280 x 640) | The logo and a one-line description on dark | GitHub: the repository's Settings, then Social preview, then Upload an image |

## The logo's colors

| Use | Light background | Dark background |
| --- | --- | --- |
| Accent ("vet" and the check) | `#C2410C` | `#FDBA74` |
| Brackets and "stack" | `#14201C` | `#F3F4EF` |
| Dark background | | `#12201B` |

## The palette: the website, diagrams, and anything else StackVet makes

Chosen by the owner on 12 October 2026 for stackvet.dev (direction C, in the logo's own colors;
`docs/website/home-page.md`). Use it for anything with StackVet's look: the site, diagrams, slides, and figures.
The contrast is each color against the background it is used on (4.5 is the least for ordinary text; WCAG AA).

Light, the page itself:

| Use | Color | Contrast |
| --- | --- | --- |
| Page background | `#EEF0EA` | |
| Panels and cards | `#FFFFFF` | |
| Lines and borders | `#E1E3DB` | |
| Text | `#14201C` | 14.6 on the page |
| Softer text | `#3B4842` | 8.3 on the page |
| Captions | `#5A6660` | 5.2 on the page, 6.0 on a panel |
| Accent: buttons and highlights, with white text | `#C2410C` | 5.2 with white text; 4.5 on the page |
| Accent text, for words in the accent color | `#9A3412` | 6.4 on the page |
| The question bubble (the AI tool asking the person) | `#FBEFE7` | `#9A3412` on it, 6.5 |

Dark, for bands such as the report and the footer:

| Use | Color | Contrast |
| --- | --- | --- |
| Background | `#14201C` | |
| Panels | `#182A23` | |
| Text | `#F3F4EF` | 15.2 on the background |
| "Needs attention" | `#FDBA74` | 9.9 on the background, 8.9 on a panel |
| "Checked" | `#A7D7BF` | 10.5 on the background, 9.4 on a panel |

Type: Atkinson Hyperlegible for everything a person reads (made for readers with low vision); JetBrains Mono, the
logo's face, only for the wordmark and small labels. Both are free under the SIL Open Font License.

**Not yet chosen:** colors for StackVet's other statuses (checked in part, tested by the app's tests, the owner's
word, the AI tool's word, not verified, not assessed), and light-background versions of "needs attention" and
"checked". A diagram that needs them chooses them from this palette's family, says so, and adds them here once the
owner settles them. Never show a status by color alone: each carries its word too, as the reports do.

**Two tags for how much a thing applies,** settled by the owner in the lifecycle diagrams of 13 October 2026
(`docs/website/diagrams/frameworks-by-sdlc-phase.svg`, backlog 0514), for any later diagram that needs them:

- **Main:** chiefly for this, in white on the accent (`#C2410C`), its cell tinted with the bubble color (`#FBEFE7`).
- **In part:** reaches into part of this, in the accent text color (`#9A3412`) on a panel, with an accent outline.

Each tag is written out in its box, so neither rests on color alone.

To show the light or dark logo to match the reader's GitHub theme, the README can use:

```html
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/brand/stackvet-logo-dark.svg">
  <img alt="StackVet" src="docs/brand/stackvet-logo.svg" width="320">
</picture>
```

## Making them again

`source/make_logo.py` draws every SVG here from the font's outlines, and `source/render.js` turns them into the
PNGs. Neither is part of `sv`, and nothing in the build or the tests runs them. To change a color or the line on
the preview image, edit `make_logo.py`. Then make a scratch folder with JetBrains Mono's release unpacked in
a `font` folder inside it (so `font/fonts/ttf/JetBrainsMono-Bold.ttf` and `-Medium.ttf`; the release is at
github.com/JetBrains/JetBrainsMono) and an empty `files` folder for the output, and run:

```bash
pip install fonttools
python3 docs/brand/source/make_logo.py <scratch folder>
NODE_PATH="$(npm root -g)" node docs/brand/source/render.js <scratch folder>   # needs Playwright
```

and copy what lands in `<scratch folder>/files` back here. Made on 8 October 2026, when the owner chose the name
StackVet and this logo (the code bracket, in terracotta) from six drafts.
