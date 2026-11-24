# sv report --oscal cannot work in the published image as most people run it: there is no home folder to make the seal's key in

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** In the published image (`ghcr.io/abbyshade111/stackvet-sv`, 06f5032) `HOME` is `/home/sv`, and
that folder does not exist: the `Dockerfile` makes the user with `useradd --no-create-home`, and `/home` belongs
to root. So `sv` cannot make `~/.config/stackvet`, and:

- every `sv report` in the image prints "The report could not be sealed (/home/sv/.config/stackvet could not be
  made (Permission denied (os error 13)))", and
- `sv report --oscal FILE` writes the report, does not write the OSCAL file ("its marker holds no seal"), and exits
  3, since ADR-156 writes the export only from a sealed report.

The GitHub Action knows this and starts the image with `-e HOME=/tmp` (`action.yml`). README's "For a compliance
pipeline: OSCAL" does not say it, and its other `docker run` lines mount the owner's own config folder, which a
pipeline does not have. A pipeline that is not GitHub's (this one is Woodpecker) meets exit 3 with no hint but the
two messages above. With `-e HOME=` set to a folder the image's user can write, the same run exits 0 and writes
the file.

**To do, one of these, for the owner to choose:**
- the image has a home folder its user can write, so a seal is made with no option given (the key lasts as long as
  the container, which is what the Action already accepts); or
- the README says what the Action does, beside `--oscal`, and the message that the OSCAL file was not written says
  that `HOME` is the reason and how to set it.

Either way, the message on every run of the image, about a seal nobody asked for, is worth a look.
