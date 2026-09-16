# Card Games
Project for fun to create a library for building playable card games as well as to implement some games in the terminal (Klondike solitaire for starters) using it.

Currently the only 3rd party crates in use are crossterm (for terminal manipulation) and rand (to shuffle cards).

Klondike Checklist
- [x] Implement rules for card/stack placement
  - [x] Implement Tableau rules
  - [x] Implement Foundation rules
- [x] Fix crash when playing King to empty Tableau stack
- [ ] Prettify UI
  - [x] Fix terminal drawing issues
  - [ ] Rearrange game elements in the typical layout for Klondike
    - [ ] Implement draing remaining CardStackVariant and TableauVariant variants