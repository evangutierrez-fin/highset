# HighSet: guía para ti (el dueño)

Esta carpeta ya tiene **toda la organización** para que otro Claude construya HighSet sin que tú tengas que diseñar nada. Aquí está lo que decidí con tus respuestas, lo que falta que decidas, y cómo arrancar.

## Qué hay en el repo

| Archivo | Para qué sirve |
|---|---|
| `AGENTS.md` / `CLAUDE.md` | Las instrucciones del constructor: cómo trabajar, cuándo algo está "hecho", reglas de código |
| `docs/PRD.md` | Qué hace HighSet y por qué (requisitos con IDs) |
| `docs/ARCHITECTURE.md` | Cómo está hecho por dentro, con diagramas |
| `docs/ROADMAP.md` | Fases, hitos (M1–M5) y fechas estimadas |
| `docs/specs/` | 18 specs, una por módulo |
| `docs/decisions/` | 23 ADRs: cada decisión con su porqué |
| `docs/tasks/` | 60 tareas con criterios de aceptación; `README.md` es el tablero |
| `docs/OPEN-QUESTIONS.md` | Lo que solo tú puedes decidir |
| `.claude/` | Subagentes `planner` y `reviewer`; skills `highset-task`, `highset-adr` y `phase-review` |
| `docs/inputs/` | Tus respuestas originales, como fuente de verdad |

Los documentos técnicos están en inglés porque así lo elegiste (Q12.8). Todo lo que es para ti está en español: esta guía, la de revisión y los reportes de cada hito.

## Cómo interpreté tus notas

- **«Si lo de nombrar ramas es como el @ de Cursor, agrégalo»:** no es lo mismo. El @ de Cursor sirve para **referenciar contexto** en un prompt. Agregué **menciones `@`**: `@kb:guia-estilo`, `@task:T-3`, `@file:src/main.rs`, `@spec:…`, `@url:…`, con un buscador fuzzy. El nombrado automático **no** se incluye; las ramas y los commits usan plantillas fijas (ADR 0021).
- **«Conocimiento que solo tenga una IA por proyecto»:** cada nota tiene un **nivel** (global, workspace, proyecto, o **local privado**, que no se sube a git) y una **audiencia** opcional: qué agentes, perfiles o tareas pueden verla. Ejemplo: «solo Claude, solo en este proyecto, sin compartir» (ADR 0020).
- **«Archivos markdown con un nombre para un propósito»:** HighSet reconoce nombres con significado fijo:
  - `PROJECT.md`: de qué trata el proyecto.
  - `*.instructions.md`: reglas.
  - `*.prompt.md`: prompts reutilizables.
  - `*.agent.md`: perfiles.
  - `skills/*/SKILL.md`: skills.
  - `MEMORY.md`: memoria.

  Con ellos genera `CLAUDE.md`, `AGENTS.md`, etc. Son las mismas convenciones de VS Code/Copilot y Agent Skills, para seguir el estándar que pediste.
- **Comando:** `highset`, como lo cambiaste. Si quieres algo más corto, pon un alias en tu shell (`alias hs=highset`).
- **Superset:** HighSet lo reemplaza. Por eso el **dashboard** muestra todos los agentes de todos los proyectos en una sola pantalla.
- **Atajos «también convencionales»:** Vim y las teclas convencionales (flechas, Ctrl+…) funcionan **al mismo tiempo**, y todo se puede reasignar.
- **Nube o carpeta sincronizada:** tus datos son archivos normales en una carpeta que puedes mover a iCloud, Syncthing o un repo git. La base de datos es solo un caché local que nunca se sincroniza, así que no se corrompe (ADR 0022).
- **«Que no sea lenta ni pesada»:** lo convertí en **presupuestos de rendimiento obligatorios**, que el CI y el release verifican. Por ejemplo: abrir en ≤ 150 ms, responder a cada tecla en ≤ 16 ms, y casi 0% de CPU sin uso (ADR 0017).
- **«Seguir el estándar profesional»:** AGENTS.md, Agent Skills, MCP, ACP, Spec Kit de GitHub, ADRs en formato MADR, Conventional Commits, SemVer y Keep a Changelog (ADR 0018).
- **No lees Rust:** los tests son la especificación. Un agente revisor revisa cada tarea, y tú revisas **comportamiento** al final de cada hito, con comandos para copiar y pegar.

## Decisiones que ya tomaste (2026-10-03)

1. **Licencia: MIT.** El archivo `LICENSE` ya está en la raíz (ADR 0023).
2. **Tiempo: 8 semanas** con el alcance completo del MVP. El hito final, M5, cae alrededor del 1 de diciembre.
3. **GitHub:** cuenta `evangutierrez-fin` (la que tiene sesión en tu `gh`). El repo será `github.com/evangutierrez-fin/highset` y el tap de Homebrew `evangutierrez-fin/homebrew-tap`.

No hay preguntas abiertas. Si el constructor necesita que decidas algo, lo anotará en `docs/OPEN-QUESTIONS.md` y te preguntará en español.

## Antes de arrancar (una sola vez)

- **Rust:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
- **git** (ya lo tienes) y **gh** (`brew install gh`, opcional, para crear PRs).
- **Los agentes que quieras usar:** Claude Code, Codex CLI, opencode, Cursor CLI. El constructor te dirá si falta algún adaptador ACP.

## Cómo arrancar la construcción

**Fases 0 y 1 (en secuencia, hasta M1).** Abre una terminal en esta carpeta, corre `claude` y pega:

> Eres el constructor de HighSet. Lee `AGENTS.md` y `docs/README.md`. Trabaja las tareas de las lanes `infra` y luego `core` (Fases 0 y 1) en orden, una a la vez, con la skill `highset-task`. Al terminar la última tarea de M1, usa la skill `phase-review` y detente para mi revisión. Háblame en español.

**Después de M1 (4 lanes en paralelo).** Abre **4 terminales** en esta carpeta, corre `claude` en cada una y pega el mismo texto cambiando la lane (`agents`, `context`, `tui`, `platform`):

> Eres el constructor de la lane `agents` de HighSet. Lee `AGENTS.md`. Trabaja solo las tareas de tu lane, en orden, con la skill `highset-task`, cada una en su propio git worktree. No toques crates de otras lanes. Cuando termines tus tareas del hito actual (ver `docs/ROADMAP.md`), avísame en español y espera.

**Al cerrar cada hito**, cuando las 4 lanes te avisen, abre una sesión más y pide:

> Usa la skill `phase-review` para el hito M<N> de HighSet.

Luego revisa con [`revision-de-fases.md`](revision-de-fases.md) y responde «M<N> aprobado» o «Cambios: …». El constructor convierte tus cambios en tareas.

**Desde M4 (dogfooding):** HighSet ya sabe manejar agentes en worktrees, así que en lugar de 4 terminales puedes usar el propio HighSet para correr las lanes.
