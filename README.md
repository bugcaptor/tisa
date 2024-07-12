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

#### How to Processing Game

- 프로그램은 오토마톤이다. 비즈니스 로직의 대부분도 데이터로 제어되게 한다.
- 한 턴에 진행해야 할 액션을 담은 데이터가 있다.
- 그 액션은 무엇일까.
- 캐릭터가 현재 할 액션을 평가한다.
    - 이동
    - 공격
    - 대화
    - 아이템 사용
    - 스킬 사용
- 캐릭터는 기억을 갖고 있고 계획을 갖고 있다. 액션 평가할 때 이 메모리를 재평가하는 방식으로 계획을 보정한다.
- 캐릭터가 할 행동을 결정하면 행동을 수행한다.
- 그러면 다른 캐릭터와 월드도 턴을 비슷하게 진행한다.
- 즉 엔티티는 할 수 있는 행동과, 기억과 계획, 행동 수행 능력을 가진다.
- 턴을 구성하는 엔티티 목록이 있어야겠구나.
- 월드에 있는 모든 엔티티는 턴을 진행한다. 나중에 무거워지면 멀리 있는 엔티티는 약식 진행을 하더라도 어쨌든 턴은 진행한다.



