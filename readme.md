# hive-rust

### work-in-progress

[Hive board game](https://www.gen42.com/download/rules/hive/Hive_English_Rules.pdf) engine implemented in Rust.  
This engine aims to be [UHP (Universal Hive Protocol)](https://github.com/jonthysell/Mzinga/wiki/UniversalHiveProtocol) compliant.

### Updates
**18/02/24** : All base logic implemented, engine is playable in human vs human scenario, no extension.  
**24/02/24** : Added UHP compliance table.

### UHP Compliance
| Command           | Implementation status |
|-------------------|----------------------|
| `info`            | ✅         |
| `newgame`         | ✅         |
| `newgame Base`    | ✅         |
| `newgame Base+X`  | ⛔          |
| `play MoveString` | ✅          |
| `pass`            | ⛔          |
| `validmoves`      | ✅          |
| `bestmove`        | ✅         |
| `undo`            | ⛔          |
| `options`         | ⛔          |  

## Perft

See https://github.com/jonthysell/Mzinga/wiki/Perft.  
With same assumptions:
- Queen not player on first player turns
- Only first bug of given type is considered a valid placing move (S1 and S2 do not generate double moves for placements)

| Depth | Base       |
|-------|------------|
| 0     | 1          |
| 1     | 4          |
| 2     | 96         |
| 3     | 1440       |
| 4     | 21600      |
| 5     | 516240     |
| 6     | 12219480   |
| 7     | 181687212* | 

*Note: depth 7 needs investigation, maybe end games not properly checked.*

Average generation speed is **700-800 KN/s (kilo nodes or moves per sec.)** on mono-threaded M1 Pro.  
Still needs to be improved.

## Graphical User Interface
I developed a GUI in order to play this engine: [link to repo.](https://github.com/alelouis/hive-gui).

https://github.com/alelouis/hive-rust/assets/6841652/74dbd1b8-3f26-4594-952b-f817843b2cb8

