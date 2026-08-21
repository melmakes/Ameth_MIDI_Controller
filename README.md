This is the repository for the Ameth MIDI Controller, the "little sister" of the Elai MIDI Controller.

Ameth uses a Xiao SAMD21 board to give the user access to 7 color-coded hall-effect-sensor-driven keys, arranged in a honeycomb-style layout. Each key is velocity-sensitive and its color corresponds to a note of the major scale...

... at least that's what it *will* be. For now, I'm trying to get the RGB LEDs working reliably...


Here's a basic list of some of the key pieces of this design:
- keycap design is my own, but derived from s-ol's hex keycap design, just modified for MX-style switches: https://hw.s-ol.nu/HEX-keycaps/#availability
- the keyswitches I'm using are Lekker L45s: https://wooting.io/product/lekker-switch-l45-v2
- the HE sensors I'm using are the Allegro A1304: https://www.allegromicro.com/en/products/sense/linear-and-angular-position/linear-sensors-1d/a1304
- the LEDs I'm using are the Neopixel 2020s by adafruit: https://www.adafruit.com/product/4684
- aaaand a lot of this project is derivative of a guide on adafruit, but I used that as more of a framework that i built everything on my own, from scratch. I mostly just used this project as a sanity check: https://learn.adafruit.com/hexpad?embeds=allow
