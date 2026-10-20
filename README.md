# Crab Force

Rewrite of [Nexus Force](https://github.com/MasterTemple/LEGO-Universe-Discord-Bot) in Rust 🦀

A Discord bot for looking up LEGO Universe game data: items, drop rates, vendors, missions,
skills, activities, and more.

## Setup

1. Create a Discord application and bot at <https://discord.com/developers/applications>.
   No privileged intents are needed. Invite it with the `bot` and `applications.commands` scopes.
2. Get the data files from a LEGO Universe client:
   - `locale.xml`: `PATH_TO_LU_CLIENT/locale/locale.xml`
   - `cdclient.sqlite`: convert the client's `cdclient.fdb` at <https://fdb.lu-dev.net/>
     (unpack only the fdb from a packed client with [lunpack](https://lu-dev.net/LUnpack/))
3. `cp .env.template .env` and fill it in.
4. Check that the data loads: `cargo run --release -- check`
5. Run the bot: `cargo run --release`

`/reload` reloads `cdclient.sqlite` and `locale.xml` without restarting. It is restricted to
`ADMIN_ROLE_IDS`, or to Administrators when that is blank.

## Commands

Every name argument has autocomplete and accepts a name (typos and word order are forgiven), an
id, or a pasted `Name [id]`.

| Command | Shows |
| --- | --- |
| `/item`, `/brick` | Stats: rarity, equip slots, armor/life/imagination, cost, level requirement, item set |
| `/get` | Every way an item can be obtained |
| `/drop`, `/unpack`, `/reward` | Smashables, packages, or activities that give an item, with the chance of getting at least one |
| `/buy` | Vendors that sell an item and the price at each |
| `/earn` | Missions and achievements that reward an item |
| `/preconditions`, `/skills` | Requirements to use an item, and the skills it grants |
| `/enemy`, `/smashable` | An enemy's stats, and everything it drops |
| `/package`, `/loottable` | What a package contains, and the items in a loot table by tier |
| `/vendor`, `/npc` | What a vendor sells, and the missions an NPC offers/accepts |
| `/mission`, `/achievement` | Objective, prerequisites, giver, and rewards |
| `/activity` | Rewards for each tier of an activity |
| `/skill`, `/skillitems`, `/cooldowngroup` | A skill, the items with it, and the skills sharing its cooldown |
| `/level` | U-Score needed for a level |
| `/search` | Search everything (optionally one category) |
| `/reload` | Reload the data files (admins) |

Buttons under each result jump to related commands, and menus open referenced objects.

## Testing without Discord

These only need `CDCLIENT_PATH` and `LOCALE_PATH`:

```sh
cargo run --release -- check                       # load the data and report problems
cargo run --release -- preview drop "notion potion" # print a command's response as JSON
PAGE=2 cargo run --release -- preview vendor 9708   # PAGE / TIER select pages and tiers
cargo run --release -- search bat lord              # print /search results
cargo run --release -- sweep                        # run every command on everything and check Discord's limits
cargo test
```

## Drop chances

For each roll in a loot matrix: the roll happens with chance `percent`, drops `minToDrop..=maxToDrop`
items, and each item's rarity comes from the rarity table (`randmax` is cumulative), then an item of
that rarity is picked uniformly from the loot table. Shown chances are the chance of getting
**at least one** of the item per smash/package/activity, combining every roll that can give it.
