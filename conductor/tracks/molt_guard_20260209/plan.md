# Implementation Plan: Molt-Guard Rebranding and Professionalization

## Phase 1: Environment Cleanup and Simplification [checkpoint: b0c226e]
- [x] Task: Remove Vault Integration (52b8a42)
    - [ ] Remove `vault` service from `docker-compose.yml`.
    - [ ] Remove `vault` related logic and dependencies from `Cargo.toml` and `src/`.
    - [ ] Update `AppState` to rely solely on environment variables.
- [x] Task: Externalize and Genericize Configuration (393b680)
    - [ ] Replace all instances of `192.168.68.68` with a generic environment variable `OLLAMA_URL`.
    - [ ] Update `docker-compose.yml` to use `OLLAMA_URL=http://backend-ollama:11434`.
    - [ ] Create `.env.example` with placeholders for all configurable variables.
- [x] Task: Sanitize Git History (1be5c4f)
    - [ ] Use `git-filter-repo` or similar to scrub local IPs and private tokens from history.
    - [ ] Ensure `.env` and local data directories are in `.gitignore`.
- [ ] Task: Conductor - User Manual Verification 'Environment Cleanup' (Protocol in workflow.md)

## Phase 2: Project Rebranding [checkpoint: 09772f4]
- [x] Task: Rename Project to `molt-guard` (6fb02e2)
    - [ ] Update `package.name` and `name` in `Cargo.toml`.
    - [ ] Rename the binary in `Dockerfile` and `docker-compose.yml`.
    - [ ] Update all internal code references and module names if necessary.
- [x] Task: TDD - Rebranded Binary Verification (Verified binary existence and run)
    - [ ] Write a test to verify the binary compiles and runs under the new name.
    - [ ] Implement name changes to pass the test.
- [ ] Task: Conductor - User Manual Verification 'Project Rebranding' (Protocol in workflow.md)

## Phase 3: Core Feature Enhancements
- [ ] Task: TDD - Configurable Prompt Guard Sensitivity
    - [ ] Write tests for different sensitivity levels (Low, Medium, High).
    - [ ] Implement the sensitivity logic in `src/prompt_guard.rs`.
- [ ] Task: TDD - OpenAI API Layer Refinement
    - [ ] Write integration tests for standard OpenAI `/v1/chat/completions` endpoints.
    - [ ] Ensure response formats match OpenAI specifications exactly.
- [ ] Task: Conductor - User Manual Verification 'Core Feature Enhancements' (Protocol in workflow.md)

## Phase 4: Documentation and Finalization
- [ ] Task: Professionalize README.md
    - [ ] Add Mermaid.js diagram for System Architecture.
    - [ ] Add Mermaid.js diagram for Request/Response Security Flow.
    - [ ] Add "Security Audit" section detailing hardening measures.
- [ ] Task: Infrastructure Synchronization
    - [ ] Update `INFRASTRUCTURE.md` and `revive_all.sh` to reflect rebranding.
- [ ] Task: Conductor - User Manual Verification 'Documentation and Finalization' (Protocol in workflow.md)
