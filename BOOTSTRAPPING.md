# 🚀 Aura Language Bootstrapping & Self-Hosting Guide

Este documento describe la arquitectura, las extensiones al núcleo del lenguaje, y el proceso completo de **Self-Hosting Bootstrapping** de Aura, mediante el cual el compilador de Aura (`aurac`) ha sido reescrito en el propio lenguaje Aura y es capaz de compilarse a sí mismo y a otros programas Aura.

---

## 📑 Tabla de Contenidos
1. [Visión General y Arquitectura](#-visión-general-y-arquitectura)
2. [Fases del Proceso de Bootstrapping](#-fases-del-proceso-de-bootstrapping)
3. [Nuevas Capacidades del Lenguaje Implementadas](#-nuevas-capacidades-del-lenguaje-implementadas)
4. [Estructura del Compilador Self-Hosted](#-estructura-del-compilador-self-hosted)
5. [Guía Paso a Paso: Compilación y Uso](#-guía-paso-a-paso-compilación-y-uso)
6. [Suite de Verificación y Pruebas Automatizadas](#-suite-de-verificación-y-pruebas-automatizadas)

---

## 🧠 Visión General y Arquitectura

El bootstrapping es el proceso mediante el cual un lenguaje de programación implementa su propio compilador utilizando su propia sintaxis y semántica. 

```
┌────────────────────────────────────────────────────────┐
│                   Stage 0 (Rust)                       │
│    Compilador Primordial aurac escrito en Rust         │
└──────────────────────────┬─────────────────────────────┘
                           │ Compila src/aura_compiler/*.aura
                           ▼
┌────────────────────────────────────────────────────────┐
│             Stage 1 (JavaScript / Node.js)             │
│   Compilador Self-Hosted en dist/aurac.mjs             │
└──────────────────────────┬─────────────────────────────┘
                           │ Compila programas .aura (ej. hello.aura)
                           ▼
┌────────────────────────────────────────────────────────┐
│               Stage 2 (Output Generado)                │
│   Código ES6 ejecutable con Node.js o el Navegador     │
└────────────────────────────────────────────────────────┘
```

---

## 🔄 Fases del Proceso de Bootstrapping

### Etapa 0: Compilador Primordial (Rust)
- Escrito en Rust en `src/`.
- Proporciona el lexer, parser, verificador de tipos y generador de código ES6 inicial.
- Se extendió con soporte para estructuras imperativas estructuradas y resolución de módulos relativos.

### Etapa 1: Compilador Escrito en Aura
- Ubicado en `src/aura_compiler/`.
- Módulos modulares: `ast.aura`, `lexer.aura`, `parser.aura`, `codegen.aura`, `main.aura`.
- Se compila usando el binario de Rust:
  ```bash
  cargo run --bin aurac -- compile src/aura_compiler/main.aura -o dist/aurac.mjs
  ```

### Etapa 2: Ejecución del Compilador Self-Hosted
- El compilador generado `dist/aurac.mjs` se ejecuta en Node.js para compilar archivos fuente `.aura`:
  ```bash
  node dist/aurac.mjs examples/bootstrap_demo/hello.aura -o examples/bootstrap_demo/hello.js
  node examples/bootstrap_demo/hello.js
  ```

---

## 🛠️ Nuevas Capacidades del Lenguaje Implementadas

Para permitir que el compilador se exprese de forma ergonómica y eficiente en Aura, se añadieron las siguientes características esenciales:

### 1. Bucles de Control de Flujo (`while`, `for..in`, `break`, `continue`)
```aura
let mut i = 0;
while (i < 10) {
  if (i == 5) {
    break;
  }
  i = i + 1;
}

for (item in items) {
  println(item);
}
```

### 2. Operador de Desenvolvimiento Rápido de Errores `?` (Try Operator)
Permite propagar variantes `Err(e)` o `None` inmediatamente hacia el llamador de la función sin anidar bloques `match`:
```aura
fn parseExpr(p: Parser): Result<ExprNode, String> {
  let left = parseAdditive(p)?;
  return Ok(left);
}
```

### 3. Asignación Generalizada de Variables y Miembros
Soporte para reasignación de variables mutables y propiedades:
```aura
let mut total = 0;
total = total + step;
p.pos = p.pos + 1;
```

### 4. Tipos y Métodos Integrados para Colecciones y Cadenas
- Métodos de `String`: `length`, `charAt`, `charCodeAt`, `substring`, `indexOf`, `includes`, `replace`, `startsWith`, `endsWith`.
- Métodos de `List`: `push`, `pop`, `shift`, `join`, `slice`, `includes`, `length`.
- `Map<K, V>` y `Set<T>` integrados.
- Preludio de APIs estándar de Node.js: `fs`, `path`, `process`, `parseInt`, `parseFloat`, `JSON`.

---

## 📂 Estructura del Compilador Self-Hosted

| Archivo | Responsabilidad |
| :--- | :--- |
| [`src/aura_compiler/ast.aura`](file:///Users/mavro/Projects/aura-lang/src/aura_compiler/ast.aura) | Definición de tipos de datos del AST (tokens, expresiones, sentencias, declaraciones de módulos). |
| [`src/aura_compiler/lexer.aura`](file:///Users/mavro/Projects/aura-lang/src/aura_compiler/lexer.aura) | Tokenizador léxico que escanea palabras clave, operadores, cadenas, identificadores y números. |
| [`src/aura_compiler/parser.aura`](file:///Users/mavro/Projects/aura-lang/src/aura_compiler/parser.aura) | Parser por descenso recursivo que construye el AST validado. |
| [`src/aura_compiler/codegen.aura`](file:///Users/mavro/Projects/aura-lang/src/aura_compiler/codegen.aura) | Generador de código JavaScript ES6 / Node.js con primitives funcionales. |
| [`src/aura_compiler/main.aura`](file:///Users/mavro/Projects/aura-lang/src/aura_compiler/main.aura) | CLI de entrada para compilar archivos `.aura` desde la línea de comandos. |

---

## 📖 Guía Paso a Paso: Compilación y Uso

### 1. Construir el compilador Self-Hosted (Stage 1)
Compila todos los módulos del compilador en Aura hacia la carpeta `dist/`:

```bash
cargo run --bin aurac -- compile src/aura_compiler/ast.aura -o dist/ast.mjs
cargo run --bin aurac -- compile src/aura_compiler/lexer.aura -o dist/lexer.mjs
cargo run --bin aurac -- compile src/aura_compiler/parser.aura -o dist/parser.mjs
cargo run --bin aurac -- compile src/aura_compiler/codegen.aura -o dist/codegen.mjs
cargo run --bin aurac -- compile src/aura_compiler/main.aura -o dist/aurac.mjs
```

### 2. Verificar la CLI del Compilador Self-Hosted
Ejecuta la CLI generada con Node.js:

```bash
node dist/aurac.mjs
```
*Salida esperada:*
```text
Aura Self-Hosted Compiler (v0.2.0)
Usage: node aurac.mjs <file.aura> [-o <output.js>]
```

### 3. Compilar un programa Aura usando el Compilador Self-Hosted
Toma cualquier archivo `.aura` (ejemplo `examples/bootstrap_demo/hello.aura`):

```bash
node dist/aurac.mjs examples/bootstrap_demo/hello.aura -o examples/bootstrap_demo/hello.js
```
*Salida esperada:*
```text
✓ Successfully compiled 'examples/bootstrap_demo/hello.aura' -> 'examples/bootstrap_demo/hello.js'
```

### 4. Ejecutar el programa compilado
```bash
node examples/bootstrap_demo/hello.js
```
*Salida:*
```text
Hello Developer from Self-Hosted Aura Compiler!
Sum of 1..100: 5050
```

---

## 🧪 Suite de Verificación y Pruebas Automatizadas

El proyecto incluye tests de integración que validan automáticamente el ciclo de bootstrapping completo en dos etapas:

```bash
cargo test --test bootstrap_tests
```

Para correr todos los 99 tests del compilador:
```bash
cargo test
```
