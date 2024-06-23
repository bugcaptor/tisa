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

### Show todo notes
```
$ tisa todo
```

## TODO
- [x] todo done toggling.
- [ ] todo editing.
- [ ] todo archive.
- [ ] todo search.