# Software Development Principles

## KISS — Keep It Simple

- Prefer the simplest solution that solves the current problem
- Avoid abstractions until they earn their complexity
- Three similar lines of code is better than a premature abstraction

## YAGNI — You Aren't Gonna Need It

- Do not add features, configuration, or flexibility for hypothetical future needs
- The right amount of complexity is the minimum needed for the current task
- Build for what is needed now; extend when the need is real

## Minimal Changes

- Only make changes that are directly requested or clearly necessary
- A bug fix doesn't need surrounding code cleaned up
- Don't add docstrings, comments, or error handling to code you didn't change

## No Over-Engineering

- Don't create helpers or abstractions for one-time operations
- Don't design for hypothetical future requirements
- Don't use feature flags or backwards-compatibility shims unless required
