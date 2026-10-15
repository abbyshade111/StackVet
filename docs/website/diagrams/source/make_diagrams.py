"""Two diagrams in the StackVet website palette (docs/brand/README.md):
the SDLC as OWASP's six-phase cycle, and where Secure by Design, ASVS, and AISVS touch each phase."""
import math
import sys
from pathlib import Path
from xml.sax.saxutils import escape

OUT = Path(sys.argv[1])
OUT.mkdir(parents=True, exist_ok=True)

PAGE, PANEL, LINE = "#EEF0EA", "#FFFFFF", "#E1E3DB"
INK, SOFT, CAPTION = "#14201C", "#3B4842", "#5A6660"
ACCENT, ACCENT_TEXT, BUBBLE = "#C2410C", "#9A3412", "#FBEFE7"
FONT = "'Atkinson Hyperlegible', 'Helvetica Neue', Helvetica, Arial, sans-serif"
MONO = "'JetBrains Mono', Menlo, monospace"

PHASES = ["Planning", "Analysis", "Design", "Implementation", "Testing & Integration", "Maintenance"]
SOURCE = ("After the typical SDLC diagram on OWASP's Integration Standards page, "
          "owasp.github.io/www-project-integration-standards/writeups/owasp_in_sdlc")


def wrap(text, size, width):
    """Lines of text that fit in width, at about 0.53 em a character."""
    per = max(8, int(width / (size * 0.53)))
    lines, line = [], ""
    for word in text.split():
        if len(line) + len(word) + (1 if line else 0) > per:
            lines.append(line)
            line = word
        else:
            line = f"{line} {word}" if line else word
    if line:
        lines.append(line)
    return lines


def text(x, y, s, size=16, fill=INK, weight="normal", anchor="start", family=FONT, extra=""):
    return (f'<text x="{x:.1f}" y="{y:.1f}" font-family="{family}" font-size="{size}" fill="{fill}" '
            f'font-weight="{weight}" text-anchor="{anchor}"{extra}>{escape(s)}</text>')


def svg(w, h, body, title, desc):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}" '
            f'role="img" aria-labelledby="t d">\n<title id="t">{escape(title)}</title>\n'
            f'<desc id="d">{escape(desc)}</desc>\n<rect width="{w}" height="{h}" fill="{PAGE}"/>\n'
            + "\n".join(body) + "\n</svg>\n")


# 1. The cycle.
def cycle():
    w, h = 900, 900
    cx, cy, r_out, r_in = 450, 470, 330, 175
    gap = 3.0  # degrees between segments
    body = [text(48, 70, "The software development lifecycle", 30, INK, "bold"),
            text(48, 104, "Six phases, repeated for each change: Maintenance leads back to Planning.", 17, SOFT)]
    for i, name in enumerate(PHASES):
        a0 = math.radians(-90 + i * 60 + gap / 2)
        a1 = math.radians(-90 + (i + 1) * 60 - gap / 2)
        tip = math.radians(-90 + (i + 1) * 60 + 4)  # the arrow's point, into the next segment's notch
        notch = math.radians(-90 + i * 60 + gap / 2 + 4)
        rm = (r_out + r_in) / 2
        p = lambda r, a: (cx + r * math.cos(a), cy + r * math.sin(a))
        pts = [p(r_out, a0), None, p(r_out, a1), p(rm, tip), p(r_in, a1), None, p(r_in, a0), p(rm, notch)]
        d = (f"M{pts[0][0]:.1f},{pts[0][1]:.1f} A{r_out},{r_out} 0 0 1 {pts[2][0]:.1f},{pts[2][1]:.1f} "
             f"L{pts[3][0]:.1f},{pts[3][1]:.1f} L{pts[4][0]:.1f},{pts[4][1]:.1f} "
             f"A{r_in},{r_in} 0 0 0 {pts[6][0]:.1f},{pts[6][1]:.1f} L{pts[7][0]:.1f},{pts[7][1]:.1f} Z")
        fill, num, label = (PANEL, ACCENT_TEXT, INK)
        body.append(f'<path d="{d}" fill="{fill}" stroke="{ACCENT}" stroke-width="2.5" stroke-linejoin="round"/>')
        mid = math.radians(-90 + i * 60 + 30)
        tx, ty = p(rm, mid)
        body.append(text(tx, ty - 8, str(i + 1), 30, num, "bold", "middle", MONO))
        words = name.split(" & ")
        if len(words) == 2:
            body.append(text(tx, ty + 20, words[0] + " &", 17, label, "bold", "middle"))
            body.append(text(tx, ty + 40, words[1], 17, label, "bold", "middle"))
        else:
            body.append(text(tx, ty + 22, name, 17, label, "bold", "middle"))
    body.append(f'<circle cx="{cx}" cy="{cy}" r="{r_in - 18}" fill="{BUBBLE}"/>')
    body.append(text(cx, cy + 22, "SDLC", 64, ACCENT_TEXT, "bold", "middle"))
    for k, line in enumerate(wrap(SOURCE + ".", 13, w - 96)):
        body.append(text(48, h - 48 + k * 18, line, 13, CAPTION))
    desc = ("A ring of six phases in order: " + ", ".join(f"{i + 1}, {n}" for i, n in enumerate(PHASES))
            + ". Each points to the next, and Maintenance leads back to Planning.")
    return svg(w, h, body, "The software development lifecycle", desc)


# 2. The frameworks against the phases. Each cell: (kind, what), kind "main" or "in part".
ROWS = [
    ("Secure by Design", "OWASP checklist 0.5", "Decided before code, in planning and design", [
        ("main", "Capture security requirements: its first step, named Planning."),
        ("in part", "Part of the same first step: the requirements the design must meet."),
        ("main", "Its core: architecture, the principles and patterns, the five-part review checklist, peer "
                 "review, risk triage, and threat modeling (steps 2 to 8)."),
        ("in part", "Handoff to development with the finished design (step 9)."),
        ("in part", "Plans the tests in design: ASVS-aligned and negative tests (MT-03, MT-04)."),
        ("in part", "Design-drift watch (step 10); logging, alerts, and a rehearsed incident plan chosen up "
                 "front (MT-01, MT-02, MT-06)."),
    ]),
    ("ASVS", "OWASP 5.0", "Requirements to build to, and to verify against", [
        ("in part", "Choose a level, 1 to 3, from what the app holds; the requirements double as a checklist."),
        ("main", "Which requirements apply; each chapter's documentation section says what to decide and "
                 "write down (V2.1, V8.1, V15.1)."),
        ("in part", "Architecture requirements (V1.1, V15.2) and recorded decisions, such as the inventory of "
                 "cryptography (V11.1)."),
        ("main", "Build to the 17 chapters: encoding, validation, sign-in, sessions, authorization, "
                 "cryptography, and more."),
        ("main", "Its purpose: verify each requirement. It is the Application Security Verification "
                 "Standard."),
        ("in part", "Deadlines for updating dependencies (V15.1, V15.2), configuration (V13), and security "
                 "logging (V16)."),
    ]),
    ("AISVS", "OWASP 1.0", "For apps that use AI, and, in its Appendix C, apps built with AI", [
        ("in part", "A workflow that covers every phase and keeps its security gates (AC.1); qualify the AI "
                 "tools and model their threats (AC.2)."),
        ("in part", "Write down the AI threat scenarios behind each guardrail (AC.1)."),
        ("main", "Access control for AI parts (C5), agents and orchestration (C9), MCP (C10), memory and "
                 "vector stores (C8)."),
        ("main", "Training data (C1), input (C2), output (C7), the model supply chain (C6); prompts (AC.3) "
                 "and checking AI-written code (AC.4)."),
        ("main", "Model validation (C3.2), adversarial robustness (C11), and adversarial testing (AC.6)."),
        ("main", "Controlled deployment and rollback (C3.3), monitoring and drift (C12), and containing a "
                 "compromise (AC.14)."),
    ]),
]


def matrix():
    left, top, label_w, col_w, head_h, row_h, pad = 40, 150, 230, 222, 64, 236, 8
    w = left * 2 + label_w + col_w * 6
    h = top + head_h + row_h * len(ROWS) + 110
    body = [text(left, 62, "Where Secure by Design, ASVS, and AISVS touch the lifecycle", 30, INK, "bold"),
            text(left, 96, "Each framework against OWASP's six phases. “Main” marks a phase the "
                 "framework is chiefly for; “In part”, one it reaches into for part of its work.", 17, SOFT)]
    # Legend.
    lx = left
    body.append(f'<rect x="{lx}" y="114" width="58" height="22" rx="4" fill="{ACCENT}"/>')
    body.append(text(lx + 29, 130, "Main", 13, "#FFFFFF", "bold", "middle", MONO))
    body.append(text(lx + 68, 131, "chiefly for this phase", 15, SOFT))
    lx += 260
    body.append(f'<rect x="{lx}" y="114" width="70" height="22" rx="4" fill="{PANEL}" stroke="{ACCENT}" '
                f'stroke-width="1.5"/>')
    body.append(text(lx + 35, 130, "In part", 13, ACCENT_TEXT, "bold", "middle", MONO))
    body.append(text(lx + 80, 131, "reaches into part of this phase", 15, SOFT))
    # Phase headings.
    for i, name in enumerate(PHASES):
        x = left + label_w + i * col_w
        body.append(f'<rect x="{x + pad / 2}" y="{top}" width="{col_w - pad}" height="{head_h - pad}" rx="8" '
                    f'fill="{INK}"/>')
        body.append(text(x + 18, top + 25, str(i + 1), 15, "#FDBA74", "bold", "start", MONO))
        lines = wrap(name, 16, col_w - 56)
        for k, line in enumerate(lines):
            body.append(text(x + 38, top + 25 + k * 19, line, 16, "#F3F4EF", "bold"))
    # Rows.
    for r, (name, edition, gist, cells) in enumerate(ROWS):
        y = top + head_h + r * row_h
        body.append(f'<rect x="{left}" y="{y + pad / 2}" width="{label_w - pad}" height="{row_h - pad}" '
                    f'rx="8" fill="{PANEL}" stroke="{LINE}" stroke-width="1.5"/>')
        body.append(text(left + 16, y + 40, name, 22, INK, "bold"))
        body.append(text(left + 16, y + 64, edition, 14, CAPTION, "normal", "start", MONO))
        for k, line in enumerate(wrap(gist, 16, label_w - 40)):
            body.append(text(left + 16, y + 96 + k * 21, line, 16, SOFT))
        for i, (kind, what) in enumerate(cells):
            x = left + label_w + i * col_w
            cx0, cy0, cw, ch = x + pad / 2, y + pad / 2, col_w - pad, row_h - pad
            if kind == "main":
                body.append(f'<rect x="{cx0}" y="{cy0}" width="{cw}" height="{ch}" rx="8" fill="{BUBBLE}" '
                            f'stroke="{ACCENT}" stroke-width="2"/>')
                body.append(f'<rect x="{cx0 + 12}" y="{cy0 + 12}" width="50" height="20" rx="4" fill="{ACCENT}"/>')
                body.append(text(cx0 + 37, cy0 + 27, "Main", 12, "#FFFFFF", "bold", "middle", MONO))
            else:
                body.append(f'<rect x="{cx0}" y="{cy0}" width="{cw}" height="{ch}" rx="8" fill="{PANEL}" '
                            f'stroke="{LINE}" stroke-width="1.5"/>')
                body.append(f'<rect x="{cx0 + 12}" y="{cy0 + 12}" width="66" height="20" rx="4" fill="{PANEL}" '
                            f'stroke="{ACCENT}" stroke-width="1.2"/>')
                body.append(text(cx0 + 45, cy0 + 27, "In part", 12, ACCENT_TEXT, "bold", "middle", MONO))
            lines = wrap(what, 14.5, cw - 26)
            assert len(lines) <= 9, (name, PHASES[i], lines)
            for k, line in enumerate(lines):
                body.append(text(cx0 + 13, cy0 + 56 + k * 19.5, line, 14.5, INK))
    yb = top + head_h + row_h * len(ROWS) + 30
    notes = ("Sources: the frameworks as StackVet carries them (data/frameworks): the Secure by Design "
             "checklist's ten process steps and its controls, ASVS 5.0's chapters and sections, AISVS 1.0's "
             "chapters and Appendix C.")
    for k, line in enumerate(wrap(notes, 13, w - 2 * left)):
        body.append(text(left, yb + k * 18, line, 13, CAPTION))
    body.append(text(left, yb + 40, "Phases: " + SOURCE + ".", 13, CAPTION))
    desc_parts = []
    for name, edition, gist, cells in ROWS:
        desc_parts.append(f"{name} ({edition}), {gist}: " + "; ".join(
            f"{PHASES[i]}, {'main' if k == 'main' else 'in part'}: {what}" for i, (k, what) in enumerate(cells)))
    return svg(w, h, body, "Where Secure by Design, ASVS, and AISVS touch the lifecycle", " ".join(desc_parts))


(OUT / "sdlc-cycle.svg").write_text(cycle(), encoding="utf-8")
(OUT / "frameworks-by-sdlc-phase.svg").write_text(matrix(), encoding="utf-8")
print("wrote", OUT)
