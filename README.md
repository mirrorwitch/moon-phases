moon-phases
===========

Command-line application to show the moon phase for a given date and time, as a
text string, emoji, or numeric value.

Designed to be fast, for use in window manager bars, shell prompts and the like.

Based on the Schaefer algorithm as implemented by Fallen4Eyes's crate `moon-phase`.
Accepts dates in any format from crate `human-date-parser`, such as:
 - "2023-10-31"
 - "2023-10-31 13:12:00"
 - "Friday"
 - "next Friday"
 - "in a month"
 - "a year ago"

Includes an option to set Unicode variation characters for colour emoji or
monochrome text.  See --help for documentation.

TODO
====

 - Zodiac moon phase
 - Cartoon face emojis
 - Calendar display
 - Translations for moon phase names
