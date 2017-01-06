# rustc-l10n

An experimental project to bring localization to `rustc`.

As all of us know, `rustc` isn't localized in its current form, nor is there
any plan to do it in immediate future. However, among the features `rustc`
*does* have right now is a machine-readable output format, and why not start
from there instead?

`rustc-l10n` is a wrapper around `rustc`. It invokes the real `rustc` with
all the arguments it's invoked with, plus an additional `--error-format=json`
for getting the output back in machine-readable form. Then it parses the
diagnostic messages if there's any, localizes them, and renders back into the
terminal just like `rustc` would. Of course the exit status of `rustc` is
preserved and passed through as well, so it's safe to replace `rustc` with
`rustc-l10n` in scripts.


## License

Same as rustc itself; that is, dual licensed under Apache License 2.0 and the
MIT license.


## Features

Completed:

* Parsing of `rustc` JSON output (specification directly taken from `rustc`)
* Somewhat basic rendering of error messages and spans

TODO:

* Localization itself XD
* Color output
* Output of complex spans


## Why the name?

Because I couldn't think of any better one; also it's expected to eventually
merge this into rustc itself, so a name is technically not needed. So I
decided to not invest any significant effort into the naming.


<!-- vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8: -->
