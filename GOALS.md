# Project Goals
I believe strongly in being verbose about the goals and long term roadmap of a project. The goals
below should stay the same in the future. New ones that are compatible with the current ones can be
added.

# Goals
- Strictly follow [semantic versioning 2.0.0](https://semver.org/spec/v2.0.0.html).
- Dependencies should only be added if they provide considerable benefit.
- Depencencies should be maintained at the point in time they are added.
- Filetypes with available formatting tools are checked for style by CI. Code that doesn't follow
  this style is rejected. This keeps the codebase clean and nice for everyone to work on.
- Correctness of the code, both in the sense of the code doing what it is supposed to do, and in the
  sense of the library following the specifications that are listed in the goals.

## Before 1.0.0
- Refine/reconsider the public API so it can become stable.
- Add tests to the public API to avoid breaking changes.
- Aim for a first release. I don't want this project to stay in 'unstable limbo' indefinitely. This
  doesn't mean the first release will be rushed, but having a first major release and stabilizing
  the public API is a goal.

# Other Goals
> These goals are hard to formally write down or check.
- After the first major release, extra care has to be taken to prevent breaking changes to the
  public API.
- Follow [the Rust API Design
  Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html).
- The flexbox implementation tries to adhere to the [CSS3 flexbox
  specification](https://www.w3.org/TR/2018/CR-css-flexbox-1-20181119/) where it makes sense in a
  TUI context.

# Non-goals
> Things that aren't goals right now and arent considered to become goals in the near future.
- Optimize for performance. While the library should be performant and I try to make things go fast,
  it is not a main goal of this project.
- Optimize for memory usage. The dependency limitation goal should keep memory usage low, but low
  memory usage is not a main goal.
