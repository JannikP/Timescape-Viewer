# Typography and Iconography in Timescape-Viewer

The Timescape-Viewer uses the [Fira Sans Condensed][1] font in its Regular variant
for all purposes. As there are very few headlines and captions there seams no
need for a separate title styles beyond using a larger font size. The font is
licensed under the [SIL Open Font License Version 1.1](OFL.txt).

The icons are from the [Remix Icon][2] collection where ever possible. Its clear,
unrounded corners should match well with the UI design of Timescape-Viewer that
doesn't use rounded corners as well.

For easy and efficient integration into the final application the icons are
added to a joint font file called `FiraSansCondensed-Regular-Expanded` using
[FontForge][3].

## How to add a new icon

1. Find a suitable icon on [RemixIcon.com][2] and download the .svg file.
2. Open an empty glyph from the [private use area][4] (starting from U+E000)
   in [FontForge][3].
3. Click File > Import... and select the .svg file. Click import with all
   options left to default. The outline of the icon should appear in the main
   square.
4. Save and close the glyph.
5. Right click the glyph and select "glyph information..."
   1. Give a descriptive name that fits the use of the icon in the app. The
      name can be different from its name on Remix Icon.
   2. Set the comment to: `Icon "_name_" from https://remixicon.com`. Use Remix
      Icon's naming here, even if the icon is used with a different meaning in
      Timescape-Viewer.
6. Save the font and click File > Generate and override the
   [FiraSansCondensed-Regular-Expanded.ttf](FiraSansCondensed-Regular-Expanded.ttf)
   in the `/assets/fonts` folder. Leave all options as they are. Warnings
   appear but so far iced seems to be ok with the generated file.
7. Add a constant for the icon to the [src/constants/icons.rs][5] file.
   Use the name of the icon from the font (See step 5.1.), add `_ICON` and
   write it in UPPER_SNAKE_CASE.
8. Use the new icon in the user interface. A frequent use is in icon + text
   buttons. The best way, I found so far, is to `concat()` a slice of `&str`s
   like that:

```rust
use crate::constants::icons::LINE_CHART_ICON;

text(
    [
        LINE_CHART_ICON,
        " ",
        t!("timescape.no_scope.add_line_chart").as_ref()
    ]
    .concat()
)
```

[1]: https://github.com/mozilla/Fira
[2]: https://remixicon.com
[3]: https://fontforge.org
[4]: https://en.wikipedia.org/wiki/Private_Use_Areas
[5]: ../../src/constants/icons.rs
