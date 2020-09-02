# kaleidopaint
Kaleido paint is a painting program that repeats what you paint by rotating around the central point several times, like a kaleidoscope.

A prototype written in JavaScript, lives [here](https://projects.ekarademir.com/kaleidopaint-proto/).

In this repo, I'm writing the desktop version using Rust. I'm using [nannou](https://github.com/nannou-org/nannou) framework.

## TODO
* Add a colour picker
* Find a fas way to keep the drawing
    * This is hard, because of how nannou creates an interface. It's quite like using Web Canvas for all of the UI and the image.
      The UI elements are actually drawn at each frame that is displayed in the window. In order to keep a persistent image
      I need to find a way to separate user drawn image from nannou drawn image at each frame and repeat what user has drawn at
      each redraw.
