# Read 7z and rar archives in the upload check (V5.2.3)

**Status:** claimed by build-0242, 11 October 2026

From the open part of 0029, part 15. Only zip and gzip are read. 7z and rar each need a reader of their own, and rar's format is proprietary, so this may stay a known limit. Decide first whether either is worth reading; the tar item comes before it.

## The decision, 11 October 2026 (for the owner)

The tar item (0241) is done. What the check does today: it writes its own zip, gzip, and tar files, with no library,
and reads them back with Python's standard library (`zipfile`, `gzip`, `tarfile`). Each format it sends must be
written by hand here and checked by a reader that is not `sv`'s own.

- **rar:** no open writer exists, and the format is proprietary. The check cannot write a rar that unpacks to a stated
  size. Recommendation: keep rar a known limit, named in the report as not sent.
- **7z:** the format is open, so a file can be written by hand. Two things make it harder than tar: a 7z that unpacks
  to one gigabyte must be compressed (a stored 7z of that size is too big to send), so it needs a hand-written LZMA2
  encoder for runs of zeros, as the deflate writer already does; and no reader is in Python's standard library, so
  the test would need one.

The owner's choice, one of:

1. **Keep both as known limits**, closing 0242 with a note. No code, no new tool.
2. **Build 7z only**, with a reader for tests. The reader is a new tool, either a Python package (`py7zr`) or the
   `7z` program. Tests only, not shipped with `sv`, but a dependency all the same: it needs its own decision record,
   and the choice of reader needs the owner's yes.
3. **Build 7z with a hand-written reader check**, no tool. Possible for the stored and zeros cases, but the test would
   then only prove what the writer itself says.

Recommendation: option 1 for rar, and for 7z option 2 only if the owner wants 7z in the report's claims; otherwise
option 1 for both. Until the owner chooses, nothing here is built.
