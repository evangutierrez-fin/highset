# Cómo revisar cada hito (sin leer código)

En cada hito el constructor deja un reporte en `docs/es/reportes/M<N>.md` con comandos para copiar y pegar. Esta guía te dice **qué buscar**. Los comandos de abajo son orientativos; el reporte trae los exactos de lo que se construyó.

**Cómo responder**

- Si todo está bien: escribe en la sesión «M<N> aprobado».
- Si algo no te gusta: «Cambios: …», en tus palabras, sin tecnicismos. El constructor lo convierte en tareas.
- Si algo te parece lento o confuso, dilo aunque «funcione». La lentitud es tu único no negociable.

**Siempre revisa:**

- [ ] ¿Se siente rápido? Abrir, moverte y escribir no debe tener ninguna espera perceptible.
- [ ] ¿Los mensajes de error dicen qué pasó y cómo arreglarlo?
- [ ] ¿Está en español cuando tu sistema está en español?
- [ ] ¿El reporte lista decisiones (ADRs) que no te convencen?

## M1: El núcleo vive

Qué debe funcionar: workspaces, proyectos y tareas desde la terminal; captura rápida; nada se pierde.

```bash
highset daemon status                       # arranca solo y dice versión
highset ws add trabajo
cd ~/algun/repo && highset project add --ws trabajo
highset add "idea: probar HighSet"          # debe sentirse instantáneo
highset task add "Primera tarea" --priority high
highset task list
highset task move T-0001 done                # debe negarse y explicar por qué (faltan fases)
highset task edit T-0001                     # abre tu editor; guarda y vuelve
rm ~/.highset/state/highset.db && highset reindex && highset task list   # todo sigue ahí
LANG=es_MX.UTF-8 highset --help
```

Pregúntate: ¿los nombres de los comandos se entienden? ¿Te gustaría otro flujo para capturar ideas?

## M2: Agentes en vivo

Qué debe funcionar: lanzar Claude Code en una tarea, cada tarea en su propia rama, verlo en vivo en la TUI, aprobar permisos, y que nada se cierre si cierras la app.

1. Abre `highset`. Usa `ctrl+k` para buscar cualquier cosa y `?` para ver los atajos.
2. En el kanban, elige una tarea y presiona `s` para iniciar Claude Code.
3. Inicia otra tarea con otro agente. Deben correr los dos al mismo tiempo.
4. Cuando Claude pida permiso, debe llegarte una notificación del sistema. Apruébalo desde HighSet.
5. Cierra HighSet (`q`) y vuelve a abrirlo. Las sesiones deben seguir corriendo.
6. `highset session log <id>`: el historial guardado.

Pregúntate: ¿se entiende qué hace cada agente? ¿Te enteras a tiempo cuando uno te espera?

## M3: Contexto y arnés

Qué debe funcionar: conocimiento con niveles y audiencia, paquetes de contexto, perfiles, `CLAUDE.md`/`AGENTS.md` generados, servidor MCP propio, menciones `@`, y los adaptadores de Codex, opencode y Cursor.

```bash
highset kb add notas/privado.md --level local --audience-agent claude-code
highset context explain T-0001 --agent claude-code     # la nota aparece
highset context explain T-0001 --agent codex           # la nota NO aparece, y dice por qué
highset sync --dry-run                                  # muestra qué cambiaría en CLAUDE.md y AGENTS.md
highset pack list
highset profile show builder --resolved
```

En la TUI, escribe un prompt con `@task:T-0001` y `@kb:…` y revisa qué se adjunta. Pídele a un agente que «busque en el conocimiento del proyecto»: debe usar las herramientas de HighSet.

Pregúntate: ¿los agentes ya no necesitan que les expliques el proyecto? Este es el punto #1 de tus prioridades.

## M4: Método y plataforma

Qué debe funcionar: el flujo completo idea → spec → plan → implementación → revisión → PR, con tus aprobaciones; memoria, standup, costos, plugins, búsqueda y todo en español.

1. Captura una idea y conviértela en tarea `feature`.
2. `highset task start T-n`: el planner escribe la spec. Apruébala (`r`).
3. Aprueba el plan. El builder implementa y el revisor revisa.
4. Revisa el diff en la TUI (`d`), apruébalo y termina con PR.
5. Al día siguiente: `highset standup`.
6. Revisa la bandeja (inbox) de memorias propuestas: aprueba unas y rechaza otras.
7. `highset cost --since 7d` y `highset budget set …`.
8. `highset plugin inspect github:…`: revisa los permisos antes de instalar.
9. `highset search "algo"`.

Pregúntate: ¿el método ayuda o estorba? ¿Las aprobaciones llegan en el momento correcto?

## M5: Versión 0.1.0

Qué debe funcionar: instalación con un comando, guía de inicio, rendimiento verificado.

1. En otro usuario o máquina limpia: `brew install …` y sigue `docs/user/es/quickstart.md` desde cero.
2. Lee `docs/perf/v0.1.0.md`: todos los presupuestos deben estar en verde.
3. Usa HighSet un día completo de trabajo real y anota lo que te estorbe. Eso alimenta v0.2.
