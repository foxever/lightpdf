# Vibe Coding Principles

This document outlines the principles for AI-assisted development in the lingpdf project.

## Core Philosophy

Vibe coding is a collaborative development approach where human intent meets AI capability. The goal is to produce high-quality, maintainable code through natural language communication.

## Principles

### 1. Intent Over Implementation

- Describe **what** you want, not **how** to implement it
- Let the AI figure out the best implementation approach
- Focus on the problem, not the solution details

**Good**: "Add a bookmark feature that saves the current page"
**Bad**: "Create a HashMap, add a bookmark button, connect to database..."

### 2. Incremental Development

- Build features step by step
- Verify each step before proceeding
- Fix issues immediately when found
- Never batch multiple untested changes

### 3. Code Quality First

- Clean code over quick fixes
- Follow existing patterns and conventions
- Maintain consistency with the codebase
- Remove unused code and warnings

### 4. Communication Clarity

- Be specific about requirements
- Provide context when needed
- Ask for clarification when ambiguous
- Confirm understanding before implementation

### 5. Verification Loop

1. Implement feature
2. Run and test
3. Fix issues
4. Repeat until working
5. Clean up and optimize

### 6. Documentation as Code

- Code should be self-explanatory
- Use meaningful names
- Document "why", not "what"
- Keep documentation in sync with code

## Workflow

### Starting a Feature

```
User: Describe the feature in natural language
AI: Propose implementation approach
User: Confirm or adjust
AI: Implement step by step
User: Verify and provide feedback
```

### Fixing Issues

```
User: Report the issue with context
AI: Analyze and propose fix
User: Confirm approach
AI: Implement fix
User: Verify resolution
```

### Code Review

```
User: Request review of specific code
AI: Analyze for issues, patterns, improvements
User: Decide on changes
AI: Implement approved changes
```

## Best Practices

### For Humans

- Provide clear, specific requirements
- Give feedback on AI suggestions
- Verify code before committing
- Maintain project vision

### For AI

- Ask clarifying questions when needed
- Explain reasoning for decisions
- Follow project conventions
- Prioritize code quality over speed

## Anti-Patterns to Avoid

- **Vague requests**: "Make it better" without specifics
- **Assumption jumping**: Implementing without confirming understanding
- **Batch changes**: Multiple untested features at once
- **Ignoring warnings**: Leaving issues for later
- **Copy-paste coding**: Duplicating code instead of abstracting
- **External app dependencies**: Calling external applications (like browsers, system tools) to implement features. Features should be self-contained within the application

## Project-Specific Guidelines

### lingpdf Conventions

- Use GPUI patterns for UI components
- Follow Rust idiomatic code style
- Keep PDF logic in `src/pdf/` module
- Platform-specific code in `src/platform/`
- Test on target platforms before release

### Commit Messages

- Describe what changed and why
- Reference issues when applicable
- Keep commits atomic and focused

---

*These principles guide collaborative development between humans and AI, ensuring efficient, high-quality code production.*
