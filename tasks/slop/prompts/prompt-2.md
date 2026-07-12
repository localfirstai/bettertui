# BetterTUI TypeScript Standardisation

## Remove Legacy Import Patterns & Modernise the TypeScript Codebase

Use the following skills:

- ralph-loop
- caveman

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MISSION

Standardise the entire TypeScript codebase before the architecture freeze.

This task is NOT about adding features.

This task is NOT about refactoring the Rust engine.

This task exists only to modernise and standardise the TypeScript layer.

The entire repository should follow ONE consistent TypeScript coding standard.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FIRST STEP (MANDATORY)

Audit the TypeScript configuration before making ANY code changes.

Study

- root tsconfig.json
- every package tsconfig.json
- package.json
- turbo.json
- pnpm-workspace.yaml
- biome.json

Determine

- module
- moduleResolution
- target
- declaration settings
- ESM/CommonJS strategy
- path aliases
- bundler strategy

Determine whether .js extensions are actually required.

Do NOT blindly remove them.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

GOAL

If the current TypeScript configuration allows extensionless imports,
standardise the entire repository to use extensionless TypeScript imports.

Example

BAD

import { Runtime } from "./runtime.js";
import { Theme } from "../theme/index.js";

GOOD

import { Runtime } from "./runtime";
import { Theme } from "../theme";

The final codebase should read like a native TypeScript project.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 1

Import Audit

Audit every TypeScript package.

packages/

Locate

- imports ending with .js
- imports ending with .ts
- imports ending with .tsx
- mixed import styles
- duplicated import paths
- incorrect relative imports
- barrel import inconsistencies

Generate an internal migration plan.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 2

Import Standardisation

If supported by the compiler configuration:

Convert every local import to extensionless imports.

Example

"./runtime.js"

↓

"./runtime"

Example

"../hooks/index.js"

↓

"../hooks"

Prefer directory imports where an index.ts exists.

Avoid

../../../../foo

where cleaner exports already exist.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 3

Package Boundary Review

Audit package imports.

Ensure every package imports through public APIs whenever appropriate.

Example

GOOD

@bettertui/shared

BAD

@bettertui/shared/src/theme

Avoid deep imports unless absolutely required.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 4

Circular Dependency Audit

Detect

- circular imports
- indirect circular imports
- self-imports
- duplicate module references

Resolve every unnecessary cycle.

Document unavoidable cycles.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 5

Import Ordering

Apply one consistent import order.

Example

1. Node built-ins

2. External packages

3. BetterTUI packages

4. Internal aliases

5. Relative imports

Within each group

Alphabetical.

Remove

- duplicated imports
- unused imports
- redundant namespace imports

Use Biome organiseImports where appropriate.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 6

Barrel File Review

Audit every index.ts.

Ensure

- no duplicate exports
- no conflicting exports
- no accidental public APIs
- no dead exports

Only export stable APIs.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 7

Path Alias Review

Review every alias.

Ensure consistency across the monorepo.

Avoid unnecessary relative paths where aliases already exist.

Remove unused aliases.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 8

TypeScript Modernisation

Audit for

- unnecessary any
- unnecessary unknown
- duplicated utility types
- duplicated interfaces
- duplicated enums
- duplicated literal unions

Prefer

readonly

const assertions

satisfies

discriminated unions

template literal types

utility types

modern TypeScript features where they improve readability.

Do NOT rewrite stable code unnecessarily.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 9

Strict Mode Review

Ensure compatibility with

strict

noUncheckedIndexedAccess

exactOptionalPropertyTypes

noImplicitOverride

noPropertyAccessFromIndexSignature

verbatimModuleSyntax

isolatedModules

Do not weaken compiler settings.

Fix the code instead.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 10

Public API Consistency

Review every exported API.

Ensure

consistent naming

consistent casing

consistent generic constraints

consistent overloads

Remove experimental exports.

Remove dead exports.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PHASE 11

Repository Consistency

Every package should follow the same conventions.

Naming

Imports

Exports

Folder structure

Type definitions

Runtime organisation

Error handling

Utilities

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

QUALITY GATES

The following MUST pass.

pnpm typecheck

pnpm build

pnpm lint

pnpm format:check

No new warnings.

No TypeScript errors.

No Biome violations.

No broken imports.

No circular dependency regressions.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DO NOT

❌ Add new features.

❌ Change runtime behaviour.

❌ Modify Rust code.

❌ Change public APIs unless fixing incorrect exports.

❌ Introduce breaking changes.

❌ Weaken compiler settings.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

FINAL REPORT

Output ONLY to the terminal.

Do NOT create a report file.

Include

1. TypeScript Configuration Summary

2. Import Migration Summary

3. Number of imports updated

4. Circular dependencies removed

5. Public API changes

6. Strict mode improvements

7. Compiler improvements

8. Remaining technical debt

9. Quality gate results

Finally answer

1. Does the repository now follow one consistent TypeScript import standard?

2. Are all local imports using the preferred project convention?

3. Is the TypeScript layer ready for the package restructuring into packages/core?

Answer ONLY based on the implementation.
