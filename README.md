# Text Interface Secretary Agent

## Purpose
- Learning and experimenting with the Rust language.
- Some personal use.

## Summary of Feature Set
- Command-line interface.
- A very personal schedule and task management assistant that reads and summarizes text in the target folder.
- Like Git, each task is completed with a single command.

## Why bother making it a CLI?
- To achieve the effect of learning the language and framework with minimal boilerplate coding and less structural concerns, allowing for rapid development through a text-based CLI.
- Managing knowledge and tasks through CLI interactions is personally enjoyable.

## How to config
- Make json file with tisa executable. See example format `config.json.example`.

## How to use

### Search
```
$ tisa search <TEXT>
```

This feature is just experimental for coding!

### Manage Todo
```
$ tisa todo list
$ tisa todo add
$ tisa todo done <index>
$ tisa todo edit <index>
$ tisa todo archive
$ tisa todo search <word>
```

### Gaming Feature

#### Data Structure

Basic concept of data structure
: `/path/variable_key.md` has value.

Programmer can access the value like get_value_of(`/Player/Identity/Name`) to get the value in a code.

```
- <root> : Game name or Player name
    - Player
        - Identity
            - Name.md
        - Location
            - Current location.md
        - Bio
            - Age.md
            - Height.md
            - Weight.md
        - Attribute
            - Level.md
            - HP.md
            - MP.md
            - EXP.md
            - Gold.md
        - Stat
            - Strength
            - Dexterity
            - Intelligence
            - Luck
        - Inventory
            - Item name.md
        - Skill
            - Skill name.md
        - Quest
            - Quest name.md
```

#### Turn

A turn advances by a cli program execution.
All variables and memory are stored in the file system.
But some explanation is printed in the console.



