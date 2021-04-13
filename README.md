# SEC_treasure_hunt

## Author Alban Favre

### Intro

This is a practical rust work for school.

### Objectives

The goal of this lab is to implement a small treasure hunting game. The user has to search for a randomly generated treasure in a grid.

see doc for full pdf

### Checkmarks

#### Initialization

- [X] At startup, the player should be able to select the color of their character
  - [X] names
  - [X] color dec
  - [X] color hex
- [X] The board has a size of 15x15 blocs
  - [ ] Make this dynamic
  - [X] The player and the treasure are placed randomly on the board.
  - [X] this is based on a seed

#### Actions

- [ ] Once the game is initialized, the player can choose among three actions
  - [X] with a word, case is ignored:
    - [X] `Move`
    - [X] `Search`
    - [X] `Quit`
    - [X] `Exit`
    - [ ] `Zmove` (BONUS)
  - [X] with the starting letter, case is ignored
    - [X] `m`
    - [X] `s`
    - [X] `q`
    - [X] `e`
    - [ ] `z` (BONUS)

- [ ] The player can also directly enter a zmove (BONUS)
  - [ ] in the `(num.num)` format
  - [ ] in the `[num,num]` format
  - [ ] in the `num,num` format
  - [X] the `num` cannot be hex for num logic (except if regex is implemented)

#### Movements

- [ ] In movement mode, player can enter a coordinate
  - [ ] in the `(num.num)` format
  - [ ] in the `[num,num]` format
  - [ ] in the `num,num` format
  - [X] `num` can be hex in format `0x1`

- [ ] Errors should be raised when:
  - [ ] the desired is in outside board (BONUS yes but actually no)
  - [ ] if the destination has a bad format like `(num,num]`
  - [ ] if incorrect number of arguments are given `(4)` or `(1,2,3)`
  - [ ] explicit persing error

  #### Search

  Look for treasure in current position

  - [X] if treasure end game
  - [ ] if not give the 