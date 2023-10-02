moon-phases
===========

Command-line application to show the moon phase for a given date and time, as a
text string, emoji, or numeric value.  It can also show the moon sign.

Designed to be fast, for use in window manager bars, shell prompts and the like.
(On a raspberry pi 4, latency has been under 10ms for all modes; on a modern
desktop, under 1.5ms.)

Based on the Schaefer algorithm as implemented by Fallen4Eyes's crate `moon-phase`.
Accepts dates in any format from crate `human-date-parser`, such as:
 - "2023-10-31"
 - "2023-10-31 13:12:00"
 - "Friday"
 - "next Friday"
 - "in a month"
 - "a year ago"

Includes an option to set Unicode variation characters for colour emoji or
monochrome text.  See `--help` for documentation.

TODO
====

 - Make exclusive options exclusive
 - Cartoon face emojis
 - Calendar display
 - Translations for moon phase names

BUGS
====

Panicking when passed dates out of range rather than gracefully exiting
(reported to human-date-parser
https://github.com/technologicalMayhem/human-date-parser/issues/1 )
