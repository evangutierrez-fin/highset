# HighSet: respuestas del cuestionario

> Fuente original de todas las decisiones de este repo. Recogido el 2026-10-03 con el cuestionario
> https://claude.ai/artifact/YERgRcsSh1vBJ4Qjrj6ZiT. Las ADRs y el PRD citan estas respuestas por número (p. ej. «Q3.2»).
> No editar: si una decisión cambia, se registra en una ADR nueva.

Estado: enviado · actualizado 2026-10-03T23:12:57.342Z

## 1. Visión y alcance

- **1.1 Nombre del proyecto**: HighSet _(recomendada)_
- **1.2 Comando en la terminal**: highset _(cambiada)_
- **1.3 Descríbela en una o dos frases, como se la contarías a alguien**: Un centro de mando para el trabajo con agentes de IA, donde puedo controlar los flujos, información para ia y herramientas, permitiéndome ser mas eficiente para en mi trabajo.
- **1.4 Qué problemas debe resolver sí o sí**: Re-explicar el proyecto a cada agente; Perder el hilo de qué hizo cada agente; Configurar todo por separado en cada CLI; Falta de método; Coordinar varios agentes sin que se pisen; Conocimiento que se pierde al cerrar la sesión _(recomendada)_
- **1.5 Quién la va a usar**: Open source público desde el día 1 _(cambiada)_
- **1.6 Para qué tipo de trabajo usas agentes**: Desarrollo de software; Investigación y análisis; Datos y análisis cuantitativo; Automatización y operaciones _(cambiada)_
- **1.7 Sistemas operativos**: macOS y Linux _(recomendada)_
- **1.8 Para cuándo quieres un MVP usable**: 4 a 6 semanas _(recomendada)_

## 2. Forma del producto

- **2.1 Qué forma tiene la app**: TUI a pantalla completa + CLI con subcomandos _(recomendada)_
- **2.2 ¿Los agentes siguen corriendo si cierras la interfaz?**: Sí: un daemon local administra los agentes; TUI y CLI son clientes _(recomendada)_
- **2.3 Qué papel juega la app frente a los agentes**: Orquestador _(recomendada)_
- **2.4 Tareas internas que la app resuelve con un modelo, sin abrir una sesión de agente**: Resumir cada sesión al terminar; Extraer decisiones y aprendizajes; Clasificar y etiquetar conocimiento nuevo; Convertir una idea suelta en borrador de spec; Nombrar ramas, commits y tareas _(cambiada)_
  - Nota: Si lo de nombrar ramas, comimos y tareas es como lo de cursor con @, agrégalo, sino no.
- **2.5 Tecnología de interfaz**: Ratatui + crossterm _(recomendada)_
- **2.6 Idioma de la interfaz**: Ambos (i18n desde el inicio)

## 3. Agentes y modelos

- **3.1 Qué agentes quieres controlar desde la app**: Claude Code; Codex CLI; opencode; Cursor CLI _(cambiada)_
- **3.2 Cómo se conecta la app con cada agente**: ACP + terminal embebida de respaldo _(recomendada)_
- **3.3 Trabajo en paralelo**: Varios agentes, cada uno en su propio git worktree y rama _(recomendada)_
- **3.4 Perfiles de agente incluidos**: Investigador; Arquitecto o planner; Implementador; Revisor; Depurador; Documentador _(cambiada)_
- **3.5 Cómo comparten información los agentes con la app**: La app expone su propio servidor MCP _(recomendada)_
- **3.6 Autonomía por defecto**: Semiautónomo _(recomendada)_
- **3.7 Proveedores de modelos para las llamadas directas**: Anthropic; OpenAI; Ollama o LM Studio (locales) _(cambiada)_
- **3.8 Rastreo de costos y tokens**: Por sesión, tarea y proyecto, con presupuestos y alertas _(recomendada)_
- **3.9 Veo que tienes Superset instalado. ¿Qué relación tendrá con HighSet?**: HighSet lo reemplaza
  - Nota: Desinstala Superset, según yo ya lo había desinstalado

## 4. Organización

- **4.1 Jerarquía principal**: Workspace → Proyecto → Tarea → Sesión _(recomendada)_
- **4.2 Qué es un proyecto**: Una carpeta o repo, con opción de vincular repos extra _(recomendada)_
- **4.3 Sistema de tareas**: Propio, con sincronización externa vía plugins después _(recomendada)_
- **4.4 Estados de una tarea**: Inbox → Spec → Plan → En curso → Revisión → Hecho _(recomendada)_
- **4.5 Datos de cada tarea**: Prioridad; Etiquetas; Agente o perfil asignado; Dependencias entre tareas; Enlaces a archivos, commits y PRs; Costo acumulado _(recomendada)_
- **4.6 Qué guardar de cada sesión de agente**: Transcript completo + resumen + diff, todo buscable _(recomendada)_
- **4.7 Tipos de búsqueda**: Fuzzy instantánea (nombres, comandos, archivos); Texto completo en todo (tareas, notas, transcripts); Filtros por metadatos (estado, etiqueta, agente) _(recomendada)_
- **4.8 Registros automáticos**: ADRs: registro de decisiones de arquitectura; Changelog por proyecto; Diario de trabajo: qué se hizo cada día _(recomendada)_
- **4.9 Captura rápida de ideas**: Comando `hs add "idea"` + tecla en la TUI, directo al Inbox _(recomendada)_

## 5. Conocimiento y contexto

- **5.1 Qué tipo de conocimiento vas a guardar**: Docs del proyecto (arquitectura, convenciones); Notas personales; Prompts y plantillas; Specs y planes; Transcripts de sesiones; Páginas web; PDFs y papers; Snippets de código; Imágenes y diagramas; Datos (CSV, JSON) _(cambiada)_
- **5.2 Formato de almacenamiento**: Markdown con frontmatter + índice SQLite _(recomendada)_
- **5.3 Dónde vive el conocimiento**: Global (`~/.highset/`) + por proyecto (`.highset/` dentro del repo) _(recomendada)_
  - Nota: Hay conocimientos que aveces solo quieres que lo tenga una ia por proyecto individual, haz algo para eso
- **5.4 Cómo llega el contexto a los agentes**: Generar CLAUDE.md, AGENTS.md y GEMINI.md desde una sola fuente; Bajo demanda con el servidor MCP de la app; Adjuntar paquetes de contexto al prompt inicial de cada tarea _(recomendada)_
  - Nota: En base a archivos markdown que podemos llamar de una forma para un propósito, como CLAUDE.md o SKILLS.md o AGENTS.md
- **5.5 Paquetes de contexto (context packs)**: Sí, como pieza central _(recomendada)_
- **5.6 Formas de agregar conocimiento**: Arrastrar archivos o `hs kb add <archivo>`; URL → Markdown limpio; PDF → texto; Repo → grafo de conocimiento (integrando graphify, que ya usas); Importar desde Notion u Obsidian; Imágenes y capturas _(cambiada)_
- **5.7 Memoria entre sesiones**: Automática con aprobación _(recomendada)_
- **5.8 Búsqueda semántica (RAG)**: No en v1 _(recomendada)_
- **5.9 Conectar con tus herramientas de notas**: (ninguna)
  - Nota: No en mi caso.
- **5.10 Mantener el conocimiento al día**: Al cerrar una tarea, detectar docs desactualizados y proponer cambios _(recomendada)_

## 6. Arnés

- **6.1 Qué define un perfil de arnés**: Instrucciones o system prompt; Modelo y parámetros; Servidores MCP activos; Skills; Slash commands y prompts reutilizables; Permisos: herramientas y comandos permitidos o bloqueados; Hooks; Subagentes _(recomendada)_
- **6.2 Quién manda sobre la configuración**: La app es la fuente de verdad y sincroniza cada CLI _(recomendada)_
- **6.3 Formato de los archivos de configuración**: TOML para configuración + Markdown con frontmatter para prompts, perfiles y skills _(recomendada)_
- **6.4 Capas de configuración**: Global → Workspace → Proyecto → Tarea _(recomendada)_
- **6.5 Aislamiento de seguridad**: Sandbox de cada agente + permisos centralizados en la app _(recomendada)_
- **6.6 Eventos que disparan hooks**: Antes de iniciar una sesión de agente; Al terminar una sesión; Al cambiar el estado de una tarea; Antes de un commit o merge; Cuando un agente pide permiso o espera tu respuesta _(recomendada)_
- **6.7 Mejorar arneses con el tiempo**: Historial en git _(recomendada)_

## 7. Plugins y extensibilidad

- **7.1 Modelo de plugins**: Dos capas: paquetes declarativos + plugins de código como procesos externos _(recomendada)_
- **7.2 Qué puede aportar un plugin**: Comandos nuevos; Perfiles de agente y arneses; Skills y prompts; Servidores MCP; Adaptadores para agentes nuevos; Importadores de conocimiento; Hooks; Integraciones (Linear, GitHub, Slack…) _(recomendada)_
- **7.3 Compatibilidad con ecosistemas existentes**: Plugins y marketplaces de Claude Code; Skills en formato SKILL.md (Agent Skills); Cualquier servidor MCP _(recomendada)_
- **7.4 Cómo se instalan los plugins**: Desde repos git (`hs plugin add github:usuario/repo`) o carpetas locales; registro propio después _(recomendada)_
- **7.5 Gestión de servidores MCP**: Central: instalas una vez, lo activas por perfil o proyecto y se sincroniza a cada agente _(recomendada)_
- **7.6 Permisos de los plugins**: Cada plugin declara lo que necesita (red, archivos, comandos) y tú lo apruebas al instalar _(recomendada)_

## 8. Metodología

- **8.1 Metodología base**: Adaptativa: SDD completo para features, ruta corta para cambios chicos _(recomendada)_
- **8.2 Checkpoints donde tú apruebas**: Aprobar la spec; Aprobar el plan; Revisar el diff antes del merge _(recomendada)_
- **8.3 Cómo se verifica el trabajo**: Tests automáticos obligatorios; Lint, formato y tipos; Checklist de criterios de aceptación de la spec; Revisión por un segundo agente _(recomendada)_
- **8.4 Qué tan estricto es el método**: Guía: sugiere el siguiente paso, pero puedes saltarlo _(recomendada)_
- **8.5 Rituales automáticos**: Standup diario: qué hicieron los agentes y qué espera tu revisión; Revisión semanal: avance, costos y aprendizajes _(recomendada)_
- **8.6 Plantillas de spec, plan, ADR, PR y retro**: Incluidas y editables _(recomendada)_
- **8.7 Flujo con git**: Rama + worktree por tarea; Commits automáticos en cada checkpoint; Conventional Commits; Crear un PR al terminar _(recomendada)_

## 9. Experiencia en la terminal

- **9.1 Distribución de la pantalla principal**: Tipo IDE _(recomendada)_
- **9.2 Atajos de teclado**: Navegación estilo Vim + acciones de una tecla con ayuda visible _(recomendada)_
  - Nota: También convencional
- **9.3 Paleta de comandos (Ctrl+K) con búsqueda fuzzy de todo**: Sí _(recomendada)_
- **9.4 Relación con tu terminal**: La app trae sus propias terminales embebidas _(recomendada)_
- **9.5 Vistas que quieres**: Dashboard: agentes activos, tareas y costos; Kanban; Sesión de agente en vivo; Visor de diffs; Explorador de conocimiento; Editor de perfiles y arneses; Gestor de plugins; Timeline o historial; Grafo de conocimiento _(cambiada)_
- **9.6 Avisos**: Notificación del sistema cuando un agente termina; Cuando un agente pide permiso o una respuesta; Resumen en la barra de estado _(recomendada)_
- **9.7 Colores**: Usar la paleta de tu terminal _(recomendada)_
- **9.8 Mouse**: Sí, opcional _(recomendada)_

## 10. Datos, seguridad y sync

- **10.1 Base de datos local**: SQLite _(recomendada)_
- **10.2 Dónde guardar API keys y tokens**: Llavero del sistema (Keychain o Secret Service) _(recomendada)_
- **10.3 Ocultar secretos en transcripts y conocimiento antes de guardarlos**: Sí _(recomendada)_
- **10.4 Sincronización entre máquinas**: Ninguna en v1; lo de cada proyecto viaja con git _(recomendada)_
  - Nota: Local es lo principal, pero debemos dejar que nuestros usuarios puedan usar nube o global sincronizada
- **10.5 Respaldos de `~/.highset`**: Snapshot automático diario _(recomendada)_
- **10.6 Telemetría**: Ninguna, todo local _(recomendada)_

## 11. Integraciones

- **11.1 Servicios a integrar (como plugins)**: GitHub _(recomendada)_
- **11.2 Tareas programadas**: Sí, en v2 _(recomendada)_
- **11.3 Control desde otras herramientas**: El socket del daemon + la CLI (cualquier script puede usarlos) _(recomendada)_

## 12. Ingeniería

- **12.1 Tu nivel de Rust**: Nunca lo he usado
- **12.2 ¿Vas a leer o modificar el código?**: No, que lo mantengan los agentes
- **12.3 Estructura del repo**: Cargo workspace con crates separados _(recomendada)_
- **12.4 Stack base (desmarca lo que no quieras)**: tokio (async); ratatui + crossterm (TUI); clap (CLI); serde + toml (configuración); rusqlite (SQLite); tantivy (búsqueda de texto completo); nucleo (fuzzy, el de Helix); portable-pty + vt100 (terminal embebida); rmcp (SDK oficial de MCP); agent-client-protocol (ACP); keyring (llavero); tracing (logs); CLI de git para worktrees _(recomendada)_
- **12.5 Calidad**: clippy en modo estricto; rustfmt; Tests unitarios y de integración; Snapshot tests de la TUI (insta); CI en GitHub Actions; Sin `unsafe` salvo justificación _(recomendada)_
- **12.6 Distribución de la app**: Homebrew tap; cargo install; Binarios en GitHub Releases (cargo-dist) _(recomendada)_
- **12.7 Licencia**: Privado por ahora; decidir después _(recomendada)_
- **12.8 Idioma de código, commits y docs técnicas**: Inglés _(recomendada)_

## 13. Plan de construcción

- **13.1 Qué debe hacer el MVP (v0.1)**: Proyectos y tareas con kanban; Lanzar agentes en worktrees y verlos en vivo; Base de conocimiento + generación de CLAUDE.md y AGENTS.md; Perfiles de arnés sincronizados a Claude Code y Codex; Servidor MCP propio; Flujo guiado Spec → Plan → Implementar; Búsqueda de texto completo; Costos y tokens; Sistema de plugins _(cambiada)_
- **13.2 Qué te entrego para el constructor**: PRD (requisitos del producto); ARCHITECTURE.md con diagramas; ADRs de las decisiones de este cuestionario; Roadmap por fases e hitos; Backlog de tareas con criterios de aceptación; CLAUDE.md y AGENTS.md para el constructor; Specs por módulo; Skills y subagentes para el constructor (planner, revisor) _(recomendada)_
- **13.3 Dónde vive el backlog de construcción**: Markdown en el repo (`docs/tasks/`) _(recomendada)_
- **13.4 Cómo se construye**: Mixto: el núcleo en secuencia, los módulos independientes en paralelo _(recomendada)_
- **13.5 Tu participación durante la construcción**: Reviso al final de cada fase _(recomendada)_
- **13.6 Usar HighSet para construir HighSet en cuanto exista el MVP**: Sí _(recomendada)_
- **13.7 Ordena tus prioridades**: 1. Calidad del contexto para los agentes · 2. Organización · 3. Tener el MVP rápido · 4. Robustez y seguridad · 5. Extensibilidad (plugins) · 6. Rendimiento · 7. Pulido visual _(recomendada)_
- **13.8 Apps que te inspiran (estética o flujo)**: Linear; Superset
- **13.9 Lo que NO quieres**: Lo que haría que odie la app es que no sea rápida o que requiera mucho computo
- **13.10 Algo más que deba saber**: Algo que debes saber es que debe poder integrarse al lo que hoy por hoy profesionalmente se esta usando y la metodología que se esta usando para seguir el estándar.
