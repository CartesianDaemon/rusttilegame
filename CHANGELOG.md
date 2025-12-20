## Unreleased

Dec 19/20

- Fix proportions of lev_chooser, supply and prog for mobile screen.
  Also for different widths and lengths of supply and programs.
- Towards menu overlay screens for help, etc.

## 1.6.3

Dec 14:

- Refactor: Using quad_timestamp and chrono.
- Refactor: Level chooser and save game traits.
- Refactor: Save outcome of each execution. Including abbreviating repeats.
- Feature/refactor: Cancel from won screen. Simplify other run/cancel controls.

## 13 Dec: presumed 1.6.2

- Gameplay: Else instr.
  - Refactor: .blocked. Test. .prev_ip.
- Gameplay: Add experimental levels with Else, including Maze.
- Feature: Update controls to allow for starting and stopping program with mouse or keyboard.
  - Refactor: Ticker, removing Input, etc.
- Feature: Level select.
- Feature: Persist unlocked levels between games.
- Feature: Cycle through different tick intervals.
- Feature/Fix: Correctly highlight current instr inside subprog.
- Refactor: Split into two engines

- Gameplay: Tweak existing levels.
- Feature/QoL: Tweak size of text on displayed ops.
- Feature: Keep mouse within smaller prog op when dragging.
- Refactor: Adjust fmt::display and fmt::debug for opcodes and instrs.

- Refactor: Rename to Scene.
- Fix: Remove v_connector below LOOP instr.
- Fix: Prevent dropping on x2 instruction when no more room

## 4 Dec: tile_engine 1.6.1, prog_puzz 1.6.1

- Animation for winning and death.
- Tweak L5&6.
- Split L10 to L10&11.

## 3 Dec: tile_engine 1.6.0, prog_puzz 1.6.0

- Engine for Prog puzz: UI for nested instr.
- Engine for Prog puzz: Separate Op and Instr, remove Node, simplify Subprog.
- Engine: Fix background colour.
- Prog Puzz: Make 10 levels.

## 22 Nov: tile_engine 1.5.6, prog_puzz 1.5.3

- Make playable release for prog_puzz.

## 22 Nov: tile_engine 1.5.5, prog_puzz 1.5.2

- Engine for Prog Puzz: Implement nested and repeat instrs.
- Prog puzz for engine: Streamline tests, add logging of instr execution during tests.

21 Nov

- Engine: Fixed rendering of map on wasm.
- Engine: Fixed screen blacking and flickering on windows.

## 20 Nov: tile_engine 1.5.1, prog_puzz 1.5.1

- Prog puzzle towards Instr with nested Instrs, separate to Op.
- Prog puzzle towards simpler tests of moving round Arena.

## 19 Nov: tile_engine 1.5.0, prog_puzz 1.5.0 # Not necessarily in sync. Should have been prog puzz 0.5.

- Prog puzzle drags and drops between supply and prog.
- Prog puzzle displays arena.
- Prog puzzle executes program, switches back and forth into running mode, and moves to next level.
- Prog puzzle using crab graphics, yay.
- Logging.
- Refactoring.
- Rotating objs when drawing.

10 Nov 2025

- Add Prog Puzzle.
- Split engine to create Push Puzzle and Prog Puzzle games in separate crates.
- Refactoring engine. Splitting out MovementLogic etc.
- Fix a lot of outstanding todos.
- Add html files for different builds

2 Nov

## 1.0.2 1 Nov 2025 c11af9f354b

- Make wasm release of latest version.

4 May 2025

- Remove ghost visualisations again

## Retroactive 1.0.1 26 Apr 2025

- Ongoing simplifications
- Include fish animations
- Work on expanding initial design into several puzzles
- Publish expanded version on github pages (??)

## Retroactive 1.0.0 ????

- Publish first version on Github pages.
- ????

## Notional 0.4.0 ????

- Implement engine and outline of biobot game.

## Initial 0.1.0 Apr 30 2024

N/A
