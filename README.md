# Xworks - a dead-simple FAQ-style static site generator.

## What do I give it?

A markdown file. Any file that follows [Commonmark](https://commonmark.org/) specs is allowed.

## What do I get?

A HTML file, with the name same as the source, containing both what was in the markdown file as html and a table of contents appended to the top. It'll look just like  [anarchy.works](http://anarchy.works)!

.. Well, not *exactly*. Xworks defaults to indexing sub-headings. If you'd like to disable this,
you can use `-i`, or `--no-index-subheadings`.

Xworks builds the table of contents by using `#` for headings and `##` for
sub-headings. You can look at `example/site.[md, html]` for an example.

## How do I use it?

```bash
xworks [source.md]
```

That's it.