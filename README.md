# SEC_treasure_hunt

## Author Alban Favre

### Intro

This is a practical rust work for school.

### Objectives

The goal of this lab is to implement a small treasure hunting game. The user has to search for a randomly generated treasure in a grid.

See doc for the full lab documentation pdf

### Report

The report explains some design choices and can be found in doc.

### Checkmarks

#### Initialization
- [x] At startup,
  - [X] The player should be able to select the color of their character
    - [X] names
    - [X] color dec
    - [X] color hex
  - [X] The board has a size of 15x15 blocs
    - [ ] Make this dynamic (abandonned)
  - [X] The player and the treasure are placed randomly on the board.
  - [x] this is based on a seed
  - [x] At startup, the player should be able to select (BONUS)
    - [x] it's tile
    - [x] the game's seed


#### Actions

- [x] Once the game is initialized, the player can choose among three actions
  - [X] with a word, case is ignored:
    - [X] `Move`
    - [X] `Search`
    - [X] `Quit`
    - [X] `Exit`
    - [x] `Zmove` (BONUS)
  - [X] with the starting letter, case is ignored
    - [X] `m`
    - [X] `s`
    - [X] `q`
    - [X] `e`
    - [x] `z` (BONUS)

- [x] The player can also directly enter a zmove (BONUS)
  - [x] in the `(num.num)` format
  - [x] in the `[num,num]` format
  - [x] in the `num,num` format
  - [X] the `num` can be hex

#### Movements

- [x] In movement mode, player can enter a coordinate
  - [x] in the `(num.num)` format
  - [x] in the `[num,num]` format
  - [x] in the `num,num` format
  - [X] `num` can be hex in format `0x1`

- [x] Errors should be raised when:
  - [x] the desired is in outside board (BONUS yes but actually no)
  - [x] if the destination has a bad format like `(num,num]`
  - [x] if incorrect number of arguments are given `(4)` or `(1,2,3)`
  - [x] explicit parsing error

#### Search

- [x] Look for treasure in current position
  - [x] if treasure end game
  - [x] if not give the shortest dist (in the format explained in report) to treasure
  - [x] The tracker is update to reflect the already searched positions

#### Inputs

- [x] validate inputs
  - [x] Display an explicit error message
  - [x] Immediately request a new entry
  - [x] accept `12,43` format for streamlining (BONUS)

#### Bonus
  

Described in details in report in doc

