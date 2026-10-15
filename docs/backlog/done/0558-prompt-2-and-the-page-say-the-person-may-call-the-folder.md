# Prompt 2 and the page say the person may call the folder anything, not only my-app

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

The owner's note, 14 October 2026, after the three-tool trial: in the VS Code trial the tool found `new-app` already there and the
owner said "choose a different name"; the prompts and the page name `my-app` as if it were the name. `docs/prompts/setup.md:30`
("an empty folder called `my-app` in your home folder is a good one") and prompt 2 ("offer to make one called `my-app` in
my home folder") should both say the name is the person's to choose: "called `my-app`, or any name you like: use yours
wherever these prompts say `my-app`". One sentence in each place, and the test that holds the page to the guide
(`crates/sv-cli/tests/setup_prompts_guide.rs`) keeps them together. For the wording lane with the other prompt items
(0545 to 0557).

**Done, 15 October 2026:** the setup page now says "an empty folder in your home folder is a good one, called `my-app`
or any name you like", and prompt 2 "offer to make one in my home folder, called `my-app` or any name I choose: ask me
which. Ask before you make it." The item's "use yours wherever these prompts say `my-app`" was left out: these are the
only two places the page says `my-app`. The test the item named (`setup_prompts_guide.rs`) holds the "Tried so far"
paragraph, not this, so a sibling test, `crates/sv-cli/tests/setup_folder_name.rs`, holds both sentences; with the page
put back as it was, it went red. `setup_prompts.rs`, which held the old words of prompt 2, now holds the new.
