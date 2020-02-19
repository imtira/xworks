# Xworks - a dead-simple FAQ-style static site generator.

No, really. Dead. Simple. No frameworks, no extra style-sheets — It spits out a single HTML file,
with 

## What do I give it?

A markdown file. Any file that follows the [Commonmark](https://commonmark.org/) specification is
allowed.

## What do I get?

A HTML file, with the name same as the source, containing both what was in the markdown file as html
and a table of contents appended to the top. It'll look just like
[anarchy.works](http://anarchy.works)!

.. Well, not *exactly*. Xworks defaults to indexing sub-headings. If you'd like to disable this,
you can use `-i`, or `--no-index-subheadings`.

Xworks builds the table of contents by using `#` for headings and `##` for
sub-headings. You can look at `example/site.[md, html]` for an example.

## How do I use it?

### Make a page

`xworks [source.md]`

That's it.

### Other options

There's two optional arguments.

`-i`, `--no-index-subtitles`: As noted in *What do I get?*, this disables
indexing subtitles (i.e. `1.1. Subsection of 1`).

`-l`, `--lang`: Set the `<html>` lang attribute. By default, it's "EN".