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
| `bestmove`        | ⛔          |
| `undo`            | ⛔          |
| `options`         | ⛔          |