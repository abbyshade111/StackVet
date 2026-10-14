# Prompt 2 and the page say the person may call the folder anything, not only my-app

**Status:** open

The owner's note, 14 October 2026, after the three-tool trial: in the VS Code trial the tool found `new-app` already there and the
owner said "choose a different name"; the prompts and the page name `my-app` as if it were the name. `docs/prompts/setup.md:30`
("an empty folder called `my-app` in your home folder is a good one") and prompt 2 ("offer to make one called `my-app` in
my home folder") should both say the name is the person's to choose: "called `my-app`, or any name you like: use yours
wherever these prompts say `my-app`". One sentence in each place, and the test that holds the page to the guide
(`crates/sv-cli/tests/setup_prompts_guide.rs`) keeps them together. For the wording lane with the other prompt items
(0545 to 0557).
