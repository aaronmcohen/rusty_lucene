# Project: Local M3 AI Engineering Workspace
## Tech Stack: Local LLM Engine (hf.co/ibm-granite/granite-4.1-8b via Ollama)

## OpenCode Tool Strategy & Operational Guidelines

You must reference this index before attempting any action. Execute your task by matching the strict criteria below:

### 1. Code Base Assessment
* **READ (`read`)**
  * *When:* Use to view internal structures, verify local imports, or trace bugs.
  * *How:* Explicitly supply targeted file line numbers for targets over 150 lines to optimize the context window.
* **LSP (`lsp`)**
  * *When:* Use before typing code to evaluate type structures, hover definitions, or trace broken symbol tokens.
  * *How:* Pull static workspace diagnostics queries sequentially.

### 2. Code Execution & Modification
* **EDIT (`edit`)**
  * *When:* Modifying existing codebase components or patching specific function scopes.
  * *How:* Provide isolated unified code diff blocks. Avoid duplicating unchanged blocks.
* **WRITE (`write`)**
  * *When:* Spawning an entirely new architecture module, configuration, or utility file.
  * *How:* Ensure destination directories exist first. Fall back to automatic directory creation inside the workspace path.
* **BASH (`bash`)**
  * *When:* Running build compilation pipelines, automated testing engines (Jest/Pytest), or confirming local dependencies.
  * *How:* Ensure execution parameters are strictly non-interactive. Never initiate hanging server processes.

### 3. External Research & Telemetry
* **WEBSEARCH (`websearch`)**
  * *When:* Broad syntax discovery or troubleshooting obscure build runtime error logs.
  * *How:* Input minimal, highly specific token strings. Use this purely for discovery.
* **WEBFETCH (`webfetch`)**
  * *When:* Reading formal framework documentation pages once a precise URL is isolated by websearch.
  * *How:* Fetch raw data blocks directly. Never attempt broad indexing or spider crawling.

### 4. Project State & Coordination
* **TODOWRITE (`todowrite`)**
  * *When:* Managing long, multi-file software engineering pipelines or refactor lists.
  * *How:* Keep checklists fragmented into short, atomic milestones. Mark complete immediately after a successful testing phase.
* **TASK (`task`)**
  * *When:* Dividing deep multi-tier feature tasks into parallelizable components.
  * *How:* Subcontract isolated subsystems (e.g., UI interface layout vs database migrations) out to unique child agents.
* **SKILL (`skill`)**
  * *When:* Ingesting customized instructions (`SKILL.md`) for distinct architectural frameworks.
  * *How:* Dynamically load specific skill name schemas when shifting into specialized workspace workflows.
* **QUESTION (`question`)**
  * *When:* Encountering systemic architectural ambiguities (e.g., missing API tokens or conflicting schema models).
  * *How:* Ask a single, direct question to the terminal prompt. Use smart engineering defaults to handle minor layout details without interrupting user focus.

