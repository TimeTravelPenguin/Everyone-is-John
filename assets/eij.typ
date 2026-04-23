#import "@preview/catppuccin:1.1.0": latte, mocha

#let theme = sys.inputs.at("theme")
#let flavor = if theme == "mocha" {
  mocha
} else if theme == "latte" {
  latte
} else if theme == "plain" {
  (colors: (text: (rgb: white.rgb())))
} else {
  panic("Unsupported theme")
}

#let text_fill = flavor.colors.text.rgb

#let sz = auto
#let margin = auto // 5pt
#set page(width: sz, height: sz, margin: margin, fill: none)

#set text(
  size: 2em,
  fill: text_fill,
)
#set align(center + horizon)

// n: number of rulers in each direction
#let draw_rulers() = {
  place(
    line(length: 200%, stroke: 0.3pt + gray),
    dy: sz / 2 - margin,
    center,
  )

  place(
    line(length: 200%, stroke: 0.3pt + gray, angle: 90deg),
    dx: sz / 2 - margin,
    horizon,
  )
}

// #draw_rulers()

#let icon = block({
  $J$
  place(
    text("i", style: "italic", size: 0.5em),
    dx: 3.9pt,
    dy: 0.5pt,
    horizon + left,
  )
  place(
    text("e", style: "italic", size: 0.7em),
    dx: -1.0pt,
    dy: -3.0pt,
    horizon + left,
  )
})

#scale(icon, 900%)
