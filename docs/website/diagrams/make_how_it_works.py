#!/usr/bin/env python3
"""Draws how-it-works.svg: the path from "I built an app" to "I have a report", in three lanes (you, your AI coding
tool, and sv) and five steps, with an arrow at each hand-off (backlog 0436). Redrawn from the paper's figure
(docs/paper/figure-how-sv-works.html) in the website's palette (docs/brand/README.md, light). Not part of sv; nothing
in the build or the tests runs it, though a test reads what it wrote. To change a word, change it here and run:

    python3 docs/website/diagrams/make_how_it_works.py            # writes how-it-works.svg beside this script
    python3 docs/website/diagrams/make_how_it_works.py OTHER.svg  # writes it elsewhere, as the test does
"""
import pathlib
import sys

OUT = pathlib.Path(__file__).resolve().parent / "how-it-works.svg"

# The palette, light (docs/brand/README.md).
PAGE, PANEL, LINE, TEXT, SOFT, CAPTION, ACCENT, ACCENT_TEXT, BUBBLE = (
    "#EEF0EA", "#FFFFFF", "#E1E3DB", "#14201C", "#3B4842", "#5A6660", "#C2410C", "#9A3412", "#FBEFE7")
FONT = "Atkinson Hyperlegible, Verdana, Arial, sans-serif"

LANES = ["You", "Your AI coding tool", "sv"]
STEPS = ["1. Plan", "2. Build", "3. Check", "4. Questions", "5. Report"]

# (step, lane): the box's lines. The AI tool's question to you is drawn as the question bubble the palette names.
BOXES = {
    (0, 0): ["Describe the app", "you want"],
    (0, 1): ["Writes stackvet.toml", "with you"],
    (0, 2): ["Works out which", "requirements apply"],
    (1, 1): ["Writes the code,", "a feature at a time"],
    (2, 1): ["Calls stackvet_check", "after each feature,", "and fixes what it finds"],
    (2, 2): ["Reads the code; says", "first what it did", "not examine"],
    (3, 0): ["Answer; then record", "your answers with", "sv review"],
    (3, 1): ["Asks the questions", "only you can answer;", "writes them as its word"],
    (4, 0): ["Run sv report --run", "at a terminal; read", "report.html"],
    (4, 2): ["Starts the app behind", "the fence; writes the", "report folder"],
}
BUBBLES = {(3, 1)}

# Arrows between boxes: (from, to, label). Each is a hand-off between lanes, or the build going on in one lane.
ARROWS = [
    ((0, 0), (0, 1), ""),
    ((0, 1), (0, 2), "spec"),
    ((0, 1), (1, 1), ""),
    ((1, 1), (2, 1), ""),
    ((2, 1), (2, 2), "check"),
    ((2, 2), (3, 1), "questions"),
    ((3, 1), (3, 0), ""),
    ((3, 0), (4, 0), ""),
    ((4, 0), (4, 2), "report"),
]

LEFT, TOP, LANE_H, STEP_W, BOX_W, BOX_H, HEAD = 170, 56, 132, 186, 164, 84, 40
WIDTH = LEFT + STEP_W * len(STEPS) + 16
HEIGHT = TOP + LANE_H * len(LANES) + 44


def centre(step, lane):
    return LEFT + STEP_W * step + STEP_W / 2, TOP + LANE_H * lane + LANE_H / 2


def edge(a, b):
    """Where an arrow from box a to box b leaves a and meets b: the facing sides."""
    (ax, ay), (bx, by) = centre(*a), centre(*b)
    if a[0] == b[0]:  # same step: up or down
        sy = 1 if by > ay else -1
        return (ax, ay + sy * BOX_H / 2), (bx, by - sy * BOX_H / 2)
    sx = 1 if bx > ax else -1
    if a[1] == b[1]:
        return (ax + sx * BOX_W / 2, ay), (bx - sx * BOX_W / 2, by)
    # A step and a lane apart: leave from the side, arrive at the top or bottom.
    sy = 1 if by > ay else -1
    return (ax + sx * BOX_W / 2, ay), (bx, by - sy * BOX_H / 2)


def esc(s):
    return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def main():
    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}" '
        f'role="img" aria-labelledby="title desc" font-family="{FONT}">',
        '<title id="title">How StackVet works</title>',
        '<desc id="desc">Three lanes, you, your AI coding tool, and sv, across five steps. 1, plan: you describe the '
        "app; your AI coding tool writes stackvet.toml with you; sv works out which requirements apply. 2, build: the "
        "tool writes the code, a feature at a time. 3, check: after each feature the tool calls stackvet_check; sv "
        "reads the code and says first what it did not examine; the tool fixes what it finds. 4, questions: the tool "
        "asks you the questions only you can answer and writes them down as its own word; you answer and record your "
        "answers with sv review. 5, report: you run sv report --run at a terminal; sv starts the app behind its fence "
        "and writes the report folder, and you read report.html.</desc>",
        "<defs><marker id=\"head\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" "
        f'orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="{ACCENT}"/></marker></defs>',
        f'<rect width="{WIDTH}" height="{HEIGHT}" fill="{PAGE}"/>',
    ]
    for i, step in enumerate(STEPS):
        x = LEFT + STEP_W * i + STEP_W / 2
        out.append(f'<text x="{x}" y="{TOP - 18}" text-anchor="middle" font-size="15" font-weight="700" '
                   f'fill="{SOFT}">{esc(step)}</text>')
    for j, lane in enumerate(LANES):
        y = TOP + LANE_H * j
        out.append(f'<rect x="8" y="{y + 4}" width="{WIDTH - 16}" height="{LANE_H - 8}" rx="10" fill="none" '
                   f'stroke="{LINE}" stroke-width="2"/>')
        out.append(f'<text x="22" y="{y + LANE_H / 2 + 6}" font-size="16" font-weight="700" fill="{TEXT}">'
                   f"{esc(lane)}</text>")
    for (step, lane), lines in BOXES.items():
        cx, cy = centre(step, lane)
        fill, stroke, colour = (BUBBLE, ACCENT, ACCENT_TEXT) if (step, lane) in BUBBLES else (PANEL, LINE, TEXT)
        out.append(f'<rect x="{cx - BOX_W / 2}" y="{cy - BOX_H / 2}" width="{BOX_W}" height="{BOX_H}" rx="8" '
                   f'fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
        first = cy - (len(lines) - 1) * 9 + 5
        for k, line in enumerate(lines):
            out.append(f'<text x="{cx}" y="{first + k * 18}" text-anchor="middle" font-size="13.5" '
                       f'fill="{colour}">{esc(line)}</text>')
    for a, b, label in ARROWS:
        (x1, y1), (x2, y2) = edge(a, b)
        if x1 != x2 and y1 != y2:
            path = f"M{x1},{y1} L{x2},{y1} L{x2},{y2}"
        else:
            path = f"M{x1},{y1} L{x2},{y2}"
        out.append(f'<path d="{path}" fill="none" stroke="{ACCENT}" stroke-width="2" marker-end="url(#head)"/>')
        if label:
            lx = (x1 + x2) / 2 if y1 == y2 else x2
            ly = y1 - 7 if y1 == y2 else (y1 + y2) / 2
            anchor = "middle" if y1 == y2 else "start"
            dx = 0 if y1 == y2 else 7
            out.append(f'<text x="{lx + dx}" y="{ly}" text-anchor="{anchor}" font-size="12.5" font-style="italic" '
                       f'fill="{ACCENT_TEXT}">{esc(label)}</text>')
    out.append(f'<text x="{LEFT}" y="{HEIGHT - 16}" font-size="12.5" fill="{CAPTION}">The orange-tinted box is your '
               "AI coding tool asking you a question.</text>")
    out.append("</svg>")
    target = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else OUT
    with open(target, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(out) + "\n")
    print(f"{target} written")


if __name__ == "__main__":
    main()
