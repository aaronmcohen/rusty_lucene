# Project: Local M3 AI Engineering Workspace
## Tech Stack: Local LLM Engine (hf.co/ibm-granite/granite-4.2-8b via Ollama)

## Global Constraints & Tool Hierarchy
Before selecting any tool, you must adhere to these absolute boundary rules:
1. **File Inspection:** You are strictly forbidden from using `bash` commands (`cat`, `less`, `more`, `head`, `tail`, `grep`) to read or inspect files. You must use `read` or `lsp`.
2. **File Mutation:** You are strictly forbidden from using shell redirections (`>`, `>>`) or utilities (`echo`, `sed`, `awk`) inside `bash` to create or modify files. You must use `write` or `edit`.
3. **Bash Scope:** The `bash` tool is an execution environment only. Limit its usage entirely to running compilers, language runtimes, packet managers, dependencies, and testing engines.

---

## Native Extended Reasoning Integration
This engine natively utilizes Granite 4.2's `<think>` chain-of-thought architecture. Before generating a tool payload, you must use your internal reasoning state to:
* Evaluate whether the user's intent requires looking at, modifying, or creating a file.
* Explicitly cross-check your intended tool choice against the *Global Constraints*.
* Discard `bash` commands instantly if they contain `cat`, `echo`, or file redirection operators (`>`, `>>`).

---

## OpenCode Tool Strategy & Operational Guidelines

### 1. Code Base Assessment
* **READ (`read`)**
  * *Context:* Use to view internal structures, verify local imports, look at existing files, or trace bugs. 
  * *Execution:* Supply targeted file line numbers (`start_line`, `end_line`) for files exceeding 10 lines to preserve context window.
* **LSP (`lsp`)**
  * *Context:* Use before code modifications to evaluate structural types, hover definitions, or trace broken symbol tokens.
  * *Execution:* Pull workspace diagnostic queries sequentially without overloading multi-file scopes.

### 2. Code Execution & Modification
* **EDIT (`edit`)**
  * *Context:* Modifying existing codebase components or patching specific function scopes.
  * *Execution:* Provide isolated unified code diff blocks. Avoid duplicating unchanged code segments.
* **WRITE (`write`)**
  * *Context:* Spawning an entirely new architecture module, configuration, boilerplate, or utility file.
  * *Execution:* Verify destination directories exist first. Fall back to automatic directory creation inside the workspace path.
* **BASH (`bash`)**
  * *Context:* Running build compilation pipelines, automated testing engines (Jest/Pytest), or confirming local dependencies.
  * *Execution:* Ensure execution parameters are strictly non-interactive. Never initiate hanging server processes or background daemons.

### 3. External Research & Telemetry
* **WEBSEARCH (`websearch`)**
  * *Context:* Broad syntax discovery or troubleshooting obscure build runtime error logs.
  * *Execution:* Input minimal, highly specific token strings. Use purely for open-ended discovery.
* **WEBFETCH (`webfetch`)**
  * *Context:* Reading formal framework documentation pages once a precise URL is isolated by websearch.
  * *Execution:* Fetch raw content blocks directly. Never attempt broad indexing or spider crawling.

### 4. Project State & Coordination
* **TODOWRITE (`todowrite`)**
  * *Context:* Managing long, multi-file software engineering pipelines or refactor lists.
  * *Execution:* Keep checklists fragmented into short, atomic milestones. Mark complete immediately after a successful testing phase.
* **TASK (`task`)**
  * *Context:* Dividing deep multi-tier feature tasks into parallelizable components.
  * *Execution:* Subcontract isolated subsystems (e.g., UI layout vs database migrations) out to unique child agents.
* **QUESTION (`question`)**
  * *Context:* Encountering systemic architectural ambiguities (e.g., missing API tokens or conflicting schema models).
  * *Execution:* Ask a single, direct question to the terminal prompt. Use smart engineering defaults to handle minor layout details without interrupting user focus.

---

## Few-Shot Tool Selection Blueprints

### Example 1: READ
* **User Input:** "What ports are defined in the docker-compose file?"
* ❌ **Forbidden Action:** `{ "tool": "bash", "arguments": { "command": "cat docker-compose.yml" } }`
*  **Correct Action:** `{ "tool": "read", "arguments": { "path": "docker-compose.yml" } }`

### Example 2: LSP
* **User Input:** "Check the type definition of the user object imported in auth.js"
* ❌ **Incorrect Action:** `{ "tool": "read", "arguments": { "path": "src/types/user.d.ts" } }` *(Inefficient for checking definitions across files)*
*  **Correct Action:** `{ "tool": "lsp", "arguments": { "query": "definition", "file": "src/auth.js", "symbol": "User" } }`

### Example 3: EDIT
* **User Input:** "Change the timeout property from 5000 to 10000 in config.json"
* ❌ **Forbidden Action:** `{ "tool": "bash", "arguments": { "command": "sed -i 's/5000/10000/g' config.json" } }`
*  **Correct Action:** `{ "tool": "edit", "arguments": { "path": "config.json", "diff": "@@ -4,3 +4,3 @@\n-  \"timeout\": 5000,\n+  \"timeout\": 10000," } }`

### Example 4: WRITE
* **User Input:** "Initialize a blank readme for this sub-module"
* ❌ **Forbidden Action:** `{ "tool": "bash", "arguments": { "command": "echo '# Submodule' > README.md" } }`
*  **Correct Action:** `{ "tool": "write", "arguments": { "path": "README.md", "content": "# Submodule\n" } }`

### Example 5: BASH
* **User Input:** "Compile the current package and make sure there are no errors."
* ❌ **Incorrect Action:** `{ "tool": "read", "arguments": { "path": "package.json" } }` *(Does not execute code or trigger build chains)*
*  **Correct Action:** `{ "tool": "bash", "arguments": { "command": "npm run build" } }`

### Example 6: WEBSEARCH
* **User Input:** "How do I fix the 'Granite token context overflow' error?"
* ❌ **Incorrect Action:** `{ "tool": "webfetch", "arguments": { "url": "https://google.com" } }` *(Directly fetching a search engine UI is invalid)*
*  **Correct Action:** `{ "tool": "websearch", "arguments": { "query": "Granite token context overflow error resolution" } }`

### Example 7: WEBFETCH
* **User Input:** "Read the documentation page on context caching located at hf.co/docs/granite/context"
* ❌ **Incorrect Action:** `{ "tool": "websearch", "arguments": { "query": "hf.co/docs/granite/context" } }` *(Unnecessary lookup since exact URL is known)*
*  **Correct Action:** `{ "tool": "webfetch", "arguments": { "url": "https://hf.co" } }`

### Example 8: TODOWRITE
* **User Input:** "We need to update our dependencies, fix the broken button component, and write tests."
* ❌ **Incorrect Action:** `{ "tool": "write", "arguments": { "path": "todo.txt", "content": "1. Fix deps\n2. Fix button" } }` *(Bypasses systemic state tracker)*
*  **Correct Action:** `{ "tool": "todowrite", "arguments": { "tasks": ["Upgrade packages via package.json", "Patch UI component boundary bug", "Execute npm run test"] } }`

### Example 9: TASK
* **User Input:** "We need to parallelize the migration process. Handle the database schema split and the client state syncing separately."
* ❌ **Incorrect Action:** `{ "tool": "bash", "arguments": { "command": "./run-migrations.sh & ./sync-state.sh &" } }` *(Backgrounding tasks in single terminal blocks is fragile)*
*  **Correct Action:** `{ "tool": "task", "arguments": { "subtasks": [ { "agent": "db-worker", "scope": "schema-split" }, { "agent": "ui-worker", "scope": "state-sync" } ] } }`

### Example 10: QUESTION
* **User Input:** "I see two database URIs in the environment configurations, but I don't know which one belongs to staging."
* ❌ **Incorrect Action:** `{ "tool": "bash", "arguments": { "command": "echo 'Which DB do I use?'" } }` *(Printing to stdout does not halt agent loops or solicit standard feedback)*
*  **Correct Action:** `{ "tool": "question", "arguments": { "query": "Ambiguous configurations found: Should staging point to DB_URI_A or DB_URI_B?" } }`

<!-- SPECKIT START -->
For additional context about technologies to be used, project structure,
shell commands, and other important information, read the current plan
at specs/001-rust-lucene-scaffolding/plan.md
<!-- SPECKIT END -->
