#!/usr/bin/env bash
set -e

# ==============================================================================
# Suite Runner: Patrones de Diseño en Aura Language (aurac)
# ==============================================================================

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${CYAN}================================================================${NC}"
echo -e "${CYAN}   🚀 EJECUTANDO LA SUITE DE PATRONES DE DISEÑO EN AURA LANG    ${NC}"
echo -e "${CYAN}================================================================${NC}\n"

PATTERNS=(
  "01_factory_method.aura:Creational - Factory Method (ADTs & Despacho Tipado)"
  "02_builder.aura:Creational - Builder Inmutable (Fluent API & Records)"
  "03_singleton.aura:Creational - Singleton Concurrente Seguro (sync.Once)"
  "04_adapter.aura:Structural - Adapter (Duck Typing Estructural)"
  "05_decorator.aura:Structural - Decorator (Funciones de Orden Superior & defer)"
  "06_facade.aura:Structural - Facade (Orquestación de Subsistemas)"
  "07_composite.aura:Structural - Composite (Tipos Suma Recursivos)"
  "08_proxy.aura:Structural - Proxy (Caché en Memoria e Intercepción)"
  "09_strategy.aura:Behavioral - Strategy (Interfaces & Pipeline |>)"
  "10_observer.aura:Behavioral - Observer / Pub-Sub (CSP Channels & Fibers)"
  "11_command.aura:Behavioral - Command con Undo Stack (LIFO History)"
  "12_state.aura:Behavioral - State (Máquina de Estados Finita Exhaustiva)"
  "13_chain_of_responsibility.aura:Behavioral - Chain of Responsibility (Middleware Pipeline)"
  "14_worker_pool.aura:Concurrency - Worker Pool (CSP Fibers & TCO Recursivo)"
  "15_object_pool.aura:Creational/Perf - Object Pool (Primitiva Nativa Pool)"
  "16_dependency_injection.aura:Architectural - Dependency Injection & IoC (Pure DI & Duck Typing)"
)

PASSED=0
TOTAL=${#PATTERNS[@]}

for item in "${PATTERNS[@]}"; do
  FILE="${item%%:*}"
  DESC="${item##*:}"
  FULL_PATH="$DIR/$FILE"

  echo -e "${BLUE}▶ [${FILE}]${NC} ${YELLOW}${DESC}${NC}"
  aurac check "$FULL_PATH" > /dev/null
  aurac run "$FULL_PATH"
  echo -e "${GREEN}✓ OK - Finalizado exitosamente${NC}\n"
  PASSED=$((PASSED + 1))
done

echo -e "${GREEN}================================================================${NC}"
echo -e "${GREEN}   ✨ Todos los ${PASSED}/${TOTAL} patrones de diseño se ejecutaron con éxito! ${NC}"
echo -e "${GREEN}================================================================${NC}"
