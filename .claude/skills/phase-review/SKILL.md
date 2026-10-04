---
name: phase-review
description: Write the Spanish milestone report for the HighSet owner (who does not read code) and stop for their review. Use when the last task of a milestone (M1–M5 in docs/ROADMAP.md) is done, or when the owner asks for a status report.
---

# Milestone report for the owner

The owner reviews **behavior, not code**, and reads Spanish. Write the report so they can verify everything by copying and pasting commands.

1. **Collect:**
   - every task of the milestone (`docs/tasks/README.md`) and its verification log;
   - ADRs created or superseded since the last report;
   - open questions;
   - CI status;
   - measured performance numbers.
2. **Write** `docs/es/reportes/M<N>.md` with this structure (in Spanish, plain and direct):

   ```markdown
   # Reporte M<N>: <nombre del hito>

   Fecha: AAAA-MM-DD · Tareas: X de Y completas · CI: verde/rojo

   ## Qué ya funciona
   - <capacidad en lenguaje de usuario, no de código>

   ## Pruébalo tú (copiar y pegar)
   1. `<comando>`: qué deberías ver.
   2. …
   (Usa las demos de docs/es/revision-de-fases.md para este hito, ajustadas a lo que se construyó.)

   ## Decisiones que tomé (ADRs)
   - ADR NNNN: <una línea: qué y por qué>

   ## Necesito que decidas
   - <OQ-n: pregunta + mi recomendación>

   ## Problemas conocidos
   - <qué falla o falta, y su impacto>

   ## Números de rendimiento
   | Métrica | Presupuesto | Medido |

   ## Lo que sigue
   - <siguiente hito y su primera tarea por lane>
   ```

3. **Check the demo steps.** Run every command in "Pruébalo tú" yourself on a clean `HIGHSET_HOME` before publishing. Don't include a step you haven't run.
4. **Commit** with `docs(es): reporte M<N>`.
5. **Stop.** Tell the owner in Spanish, in 2–3 lines, that the milestone is ready for review and where the report is. Don't start the next milestone until the owner answers.
