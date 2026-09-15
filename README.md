# Desarrollo de API REST con Rust y Actix Web

Se requiere construir una API REST para gestionar operaciones de un sistema de préstamos. La API debe permitir la creación, lectura, actualización y eliminación de préstamos. Los préstamos tienen atributos como 'id','monto', 'tasa de interés', 'fecha de vencimiento' y 'estado'. La API debe manejar correctamente los errores de validación y proporcionar respuestas idempotentes. El sistema debe asegurar que no se puedan crear préstamos con montos negativos o tasas de interés inválidas. Además, debe mantener la consistencia de los datos al actualizar o eliminar préstamos.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l1 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Node.js 18+, npm, VS Code o similar.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Ejecuta `npm install && npm run build` (o `npm start`). Si no hay errores, estás listo.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición de endpoints y modelos de datos

**Objetivo:** Definir los endpoints necesarios para la API y los modelos de datos correspondientes.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Identificar los endpoints requeridos para crear, leer, actualizar y eliminar préstamos.
- Definir los modelos de datos para los préstamos, incluyendo sus atributos y restricciones de validación.
- Asegurar que la API maneje correctamente los errores de validación y proporcione respuestas idempotentes.

**Entregable:** Definición de endpoints y modelos de datos para la API REST.

<details>
<summary>Pistas de conocimiento</summary>

- Considera las restricciones de negocio al definir los modelos de datos.
- Piensa en cómo manejar los errores de validación de manera efectiva.

</details>

### Fase 2: Implementación de endpoints y lógica de negocio

**Objetivo:** Implementar los endpoints definidos y la lógica de negocio correspondiente.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Implementar los endpoints para crear, leer, actualizar y eliminar préstamos.
- Asegurar que la lógica de negocio se aplique correctamente al crear, actualizar o eliminar préstamos.
- Mantener la consistencia de los datos y manejar los errores de manera efectiva.

**Entregable:** Implementación de endpoints y lógica de negocio para la API REST.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo mantener la consistencia de los datos al implementar la lógica de negocio.
- Piensa en cómo manejar los errores de manera efectiva y proporcionar respuestas idempotentes.

</details>

### Fase 3: Pruebas y optimización

**Objetivo:** Realizar pruebas unitarias y de integración para asegurar la funcionalidad correcta de la API y optimizar su rendimiento.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Escribir pruebas unitarias para los modelos de datos y la lógica de negocio.
- Realizar pruebas de integración para asegurar que los endpoints funcionen correctamente.
- Optimizar el rendimiento de la API identificando y resolviendo cuellos de botella.

**Entregable:** Pruebas unitarias y de integración, y optimización de rendimiento para la API REST.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo escribir pruebas efectivas para asegurar la funcionalidad correcta de la API.
- Piensa en cómo optimizar el rendimiento de la API identificando y resolviendo cuellos de botella.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es un endpoint en una API REST y cuál es su propósito?
- **paraQueSirve**: ¿Para qué sirve la lógica de negocio en una API REST y cómo se aplica?
- **comoSeUsa**: ¿Cómo se usan las pruebas unitarias y de integración para asegurar la funcionalidad correcta de una API REST?
- **erroresComunes**: ¿Cuáles son los errores comunes al implementar una API REST y cómo se pueden manejar?
- **queDecisionesImplica**: ¿Qué decisiones implica la optimización del rendimiento de una API REST y cómo se toman?

## Criterios de Evaluacion

- Definición correcta de endpoints y modelos de datos.
- Implementación efectiva de endpoints y lógica de negocio.
- Pruebas unitarias y de integración efectivas.
- Optimización efectiva del rendimiento de la API.

## Como trabajar con un asistente de IA

Hay dos caminos, elegi uno:

- **AGENTS.md** (recomendado) — instrucciones nativas del repo. Abri esta carpeta con tu agente local (Claude Code, Cursor, Codex, Copilot, Gemini) y las carga solo. Sabe que archivos faltan y con que comando se verifica, y completa el scaffold escribiendo en disco.
- **PROMPT_MEJORA.md** — para copiar y pegar en un chat (claude.ai, ChatGPT). Devuelve un ZIP con el proyecto. Sirve si no tenes un agente en el IDE.

Ninguno de los dos resuelve las fases del reto: eso es tu trabajo.

## Verificacion

El proyecto esta listo para trabajar cuando este comando corre sin errores:

```bash
el comando de build o arranque canonico del stack elegido
```

---

*Reto generado automaticamente por Challenge Generator - Pragma*
