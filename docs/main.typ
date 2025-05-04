#import "@preview/touying:0.6.1": *
#import themes.university: *

#show: university-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: "libp2p",
    subtitle: "Modular Peer-to-Peer Networking Stack for Rust",
    short-title: "github.com/erhant/rustconnect-libp2p",
    author: "Erhan Tezcan",
    date: "4.5.2025",
    institution: "Blockchain Dev. @ Dria",
  ),
  config-colors(
    primary: rgb("#B7410E"), // rust orange
    secondary: rgb("#EC5800"), // bright rust
    tertiary: rgb("#F74C00"), // warm orange
    neutral: rgb("#2C2C2C"), // dark grey
    neutral-darkest: rgb("#1C1C1C"), // darker grey
  ),
)

#title-slide()
#outline()

#include "introduction.typ"
#include "identity.typ"
#include "discovery.typ"
#include "communication.typ"
#include "usage.typ"
#include "caveats.typ"
#include "ffi.typ"

= Demo?
= Thank You!
