# Putting Simulator
From my youtube series Code With Me: Putting Simulator

This project allows you to create holes, play the holes, and simulate thousands of shots at the holes to find your hole in one shots

## Dependencies
1. macroquad
2. colors-transform

## Play
This project will let you play a hole that you have created. It accepts a `.hole` file as the first argument to the program.

### Example Usage
`cargo run -- res/test.hole`

### Controls
| Input         | Command    |
| ------------- | ---------- |
| Mouse Pointer | Aim Shot   |
| Space         | Shoot Ball |
| R             | Reset Ball |

## Edit
This project lets you create a `.hole` file. It accepts an optional `.hole` file as the first argument to the program as a starting point.

### Example Usage
`cargo run`

`cargo run -- res/test.hole`

### Controls
| Input | Command                                        |
| ----- | ---------------------------------------------- |
| B     | Place Ball at Mouse Pointer                    |
| C     | Place Cup at Mouse Pointer                     |
| W     | Place Wall Anchor at Mouse Pointer             |
| E     | Create a wall between Anchor and Mouse Pointer |
| S     | Save hole into `out.hole`                      |

## Sim
This project simulates the ball being shot at multiple powers and angles. It accepts 2 arguments. The first argument is how manu pixels apart each shot is. The 2nd argument is a `.hole` file to create the hole from.

### Example Usage
`cargo run -- 1 res/test.hole`

`cargo run -- 25 res/test.hole`

## Hole
This project houses the shared library files between the projects. You will find code for the Ball, Walls, and Hole in this project.
