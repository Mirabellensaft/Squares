![purple square logo](https://github.com/ferrous-systems/Squares/blob/master/example%20images/logo.png " ")
# Squares - coloring squares from http requests

The program generates a grid in the size of your choice. The squares can be colored
by sending JSON objects via POST requests containing the coordinates of the square
in the grid as well as RGB values.


## Download
  Get the zip file from [Ferrous Systems - Squares](https://github.com/ferrous-systems/Squares/archive/master.zip) or clone the repository from `git@github.com:ferrous-systems/Squares.git`.


## Requirements
  Rust nightly
  ```
  $ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
  ```
  sdl2 library
  [SDL Wiki](https://wiki.libsdl.org/Installation)

## Usage
To run the program:
```
$YourDirectory/squares/squares cargo run <rows> <columns>
```
Example:
```
$YourDirectory/squares/squares cargo run 4 6
```
produces a grid with 4 rows and 6 columns:
![clear grid](https://github.com/ferrous-systems/Squares/blob/master/example%20images/5.png " ")

## Controls
- toggle fullscreen: space
- clear grid: return
- quit: esc
- toggle pause: B


## How to color pixels:
To color squares send POST requests of the following format to hostname/cell:
```
{"row":<i32>,"column":<i32>,"red":<u8>,"green":<u8>,"blue":<u8>}
```
Allowed values:
- colors: 0-255
- row and column: 0 - your specified maximum - 1

Example with curl:

```
curl --request POST --data '{"row":2,"column":4,"red":250,"green":68,"blue":199}' http://localhost:8000/cell
```

Running this will change the color of the square in row 2 and column 4 to pink.
![pink square in row 2 and column 4](https://github.com/ferrous-systems/Squares/blob/master/example%20images/2.png " ")

Example with squares_test (only on localhost):
```
$YourDirectory/squares/squares_test cargo run <row> <column> <red> <green> <blue>
```
```
$YourDirectory/squares/squares_test cargo run 3 4 77 46 90
```
Running this will change the color of the square in row 3 and column 4 to purple.
![pink square in row 2 and column 4 and purple square in row 3 and column 4](https://github.com/ferrous-systems/Squares/blob/master/example%20images/3.png " ")

## How to draw lines

To draw a line, send POST requests of the following format to hostname/line:

```
{"row":<i32>,"column":<i32>,"red":<u8>,"green":<u8>,"blue":<u8>,"direction":<i32>,"length":<i32>}
```

Allowed values:
- colors: 0-255
- row and column: 0 - your specified maximum - 1
- direction: 1 for vertical, 0 for horizontal
- length:

The coordinates mark the starting point of the line. The length is the length of the entire line. The length of the line has to be inside 


## Intervention
The program can be intervened by sending GET requests.

```
http://localhost:8000/intervention/true
or
http://localhost:8000/intervention/false
```
Sending a GET request containing `true` will pause the animation with a checker board screen. Sending `false` resets the grid and the animation continues from a blank screen.


![checker board](https://github.com/ferrous-systems/Squares/blob/master/example%20images/4.png " ")


This can be used to signal viewers that something else will happen on the screen, or to just reset the grid. Hitting `b` on the computer, the program is running on,  will pause the animation without clearing the screen. Hitting `return` will clear the screen without pausing the animation.  
