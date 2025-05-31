SourceTrait Note
================================================================================
[![Crate Badge]][Crate] [![License Badge]][License]

*A very, very simple Markdown notes manager*

This command-line tool manages note files in your preferred directory, organized
under one of the following four categories:
- Today
- Idea
- Todo
- Plan

The *Today* category is sharded by year and month. The *Plan* category is either
singular or topical. Trest of the categories are named with their topic.

When running the *today* subcommand, you can optionally carry over your daily
list from the previous day.

Usage
--------------------------------------------------------------------------------
For help, see `srctrait-note --help`.

`srctrait-note <command>`
- `today` Edits today's note
  - `today from <date>` Carries over notes from the previous date into today's
- `yesterday` Edits yesterday's note
- `day` Edits the given day's note
- `idea <topic>` Edits an idea note for that topic
- `plan` Edits your master plan
- `plan <topic>` Edits topical plan
- `todo <topic>` Edits a topical TODO list
- `config` Edits the command's config file. Configures the notes dir and editor command.
- `pick <note type>` Uses [yazi](https://github.com/sxyazi/yazi) to choose a note to edit

Installation
--------------------------------------------------------------------------------
With Rust install, run:

`cargo install srctrait-note-cli`

The standalone command's default name is **srctrait-note**. It's recommended
to make an alias to something like *note* for ease of use.

In Fish, this would be something like:
```bash
alias note "srctrait-note"
funcsave note
```


Repository
--------------------------------------------------------------------------------
Contributors, please review [SRCTRAIT.md](./SRCTRAIT.md).  

Found a bug? Search for an existing issue on GitHub.  
If an issue exists, chime in to add weight to it.  
If not, creation one and let us know. 


License (AGPL3)
--------------------------------------------------------------------------------
SourceTrait Note: Markdown notes manager  
[SourceTrait](https://sourcetrait.com)  
Copyright (C) 2025 [Asmov LLC](https://asmov.software)  

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a [copy](./LICENSE-AGPL-3.txt) of the
GNU Affero General Public License along with this program.
If not, see https://www.gnu.org/licenses/.

[Crate]: https://crates.io/crates/srctrait-note-cli
[Crate Badge]: https://img.shields.io/crates/v/srctrait-note-cli.svg
[License]: (#License-AGPL3)
[License Badge]: https://img.shields.io/badge/license-AGPL3-blue.svg
