# Cursive Flexbox
This aims to be a complete flexbox implementation for Cursive, a Rust TUI
library. It should be complete in the sense that it supports most things that
the CSS3 flexbox supports.

> Development of this library just started so of course, it isn't complete yet.
> Progress will be tracked below. API design is absolutely not final, and won't
> be before all major functionality is implemented.

## Currently Implemented
- [ ] direction
    - [x] row
    - [ ] row-reverse
    - [x] column
    - [ ] column-reverse
- [x] justify-content
    - [x] start
    - [x] end
    - [x] center
    - [x] space-between
    - [x] space-around
    - [x] space-evenly
- [x] align-items
    - [x] start
    - [x] end
    - [x] center
    - [x] stretch
- [x] align-content
    - [x] start
    - [x] end
    - [x] center
    - [x] stretch
    - [x] space-between
    - [x] space-around
- [ ] gap
    - [x] main-axis-gap
    - [ ] cross-axis-gap
- [ ] flex-grow

## API and Guidelines
Once more things have been implemented and I have a general overview of the
status and implementation of the project, I'll start to work towards adhering to
[the general Rust guidelines](https://rust-lang.github.io/api-guidelines/).

## Contributions
Contributions are very much welcome. Keep in mind that the project just started
so there isn't much implemented yet.
