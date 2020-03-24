# Xworks - a dead-simple FAQ-style static site generator.

No, really. Dead. Simple. No frameworks, no extra style-sheets — It spits out a single HTML file,
with 60~ lines of pretty in-line CSS. *And* it's prettier than
[motherfuckingwebsite](http://motherfuckingwebsite.com/).

## What do I give it?

A markdown file. Any file that follows the [CommonMark](https://commonmark.org/) specification is
allowed.

## What do I get?

A HTML file, with the name same as the source, containing both what was in the markdown file as html
and a table of contents appended to the top. It'll look just like
[anarchy.works](http://anarchy.works)!

.. Well, not *exactly*. Xworks defaults to indexing sub-headings. If you'd like to disable this,
you can use `-i`, or `--no-index-subheadings`.

Xworks builds the table of contents by using `#` for headings and `##` for sub-headings. You can
look at `example/site.[md, html]` for an example.

For a visual example of output, you can check out [t1ra.github.io/xworks](https://t1ra.github.io/xworks).

## How do I use it?

### Make a page

`xworks [source.md]`

That's it.

### Other options

There's three optional arguments.

`-i`, `--no-index-subtitles`: Disable subtitle indexing (i.e. `1.1. Subsection of 1`).

`-l`, `--lang`: Set the `<html>` lang attribute. By default, it's "EN".

`-t`, `--title`: Set the page title. By default, it's the name of the Markdown file, without the
extension.
