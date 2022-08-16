[![CI](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/rust.yml/badge.svg)](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/rust.yml)
[![Coverage](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/coverage.yml/badge.svg)](https://github.com/boerdy/graphalgorithms-feedback-arc-set/actions/workflows/coverage.yml)

# Algorithmen für Feedback-Arc-Set
## Berger and Shor, 1990
- [Saab](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.7745&rep=rep1&type=pdf), Seite 236
- Einfache Heuristik, die das FAS anhand der Anzahl ein- und ausgehender Kanten eines Knoten aufbaut
- **Einschränkung:** Laut Paper nur auf planaren Graphen korrekt
- **Implementierung:** *src/fas/simple_heuristic.rs*

## Eades, Smyth and Lin, 1989
- [Saab](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.7745&rep=rep1&type=pdf), Seite 238/239
- Divide-And-Conquer Heuristik, die 
  - den Graphen in Supgraphen zerlegt (1. und 2. Hälfte der topologischen Sortierung)
  - sich zu Nutze macht, dass in einer topologischen Sortierung die linksgerichteten Kanten ein FAS bilden
- Qualität abhängig von gewählter Sortierung
- **Verwendete Unter-Algorithmen:**
  - Topologische Sortierung nach Anzahl eingehender Knoten (*order/topological_sort.rs*) 
- **Implementierung:** *src/fas/divide_and_conquer_by_order_heuristic.rs*

## Saab, 2001
- [Saab](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.7745&rep=rep1&type=pdf), Seite 241
- Divide-And-Conquer Heuristik, die 
  - den Graphen in Supgraphen (Strongly Connected Components) zerlegt
  - auf SCCs eine ausgeglichene Bisektion bildet und Kanten, welche von B2 nach B1 verlaufen, in das FAS aufnimmt
- Qualität abhängig von Kosten der Bisektion (Anzahl Kanten zwischen beiden)
- **Verwendete Unter-Algorithmen:**
  - Tarjan's SCC (*scc/tarjan.rs*) 
  - Bisektion durch Annäherung an Optimum niedriger Kosten (*bisection/stochastical_evolution.rs*)
- **Enschränkungen:**
  - Fehler im Paper auf Seite 243: Statt ```Cpre = cost(V1, V2)``` muss ```Cpre = cost(B1, B2)``` sein, da die Kosten ja immmer besser werden sollen!
  - Fehler im Paper auf Seite 241: Der Algorithmus macht einen endlosen rekursiven Abstieg. Der Code ```fas(G[V1 ]) ∪ fas(G[V2 ])``` ist unserer Meinung nach überflüssig, da V1 und V2 die Bisektion einer SCC sind und somit keinen Zyklus haben KÖNNEN.
  - Der Algorithmus funktioniert dennoch nicht korrekt. **Daher Tests deaktiviert und keine Benchmarks durchgeführt!** Vermutlich weiterer Logik-Fehler   
  - **Implementierung:** *src/fas/divide_and_conquer_by_bisection.rs*

## Eades, Smyth and Lin, 1993
- [Saab](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.7745&rep=rep1&type=pdf), Seite 238/239
- Grundversion mit zufälliger Knotenauswahl auch in [Saab](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.7745&rep=rep1&type=pdf), Seite 238 beschrieben
- Greedy Heuristik, die 
  - Sinks, Sources und - falls nicht vorhanden - Knoten mit maximalen Delta zwischen Anzahl ein-/ausgehender Kanten betrachtet
  - sich zu Nutze macht, dass in einer topologischen Sortierung die linksgerichteten Kanten ein FAS bilden
- Qualität abhängig von Auswahl gewählten Knotens, wenn keine Sinks/Sources vorhanden
- **Implementierung:** *src/fas/greedy_heuristic.rs*

# Tests
```bash
cargo test
```
- Tests der Unter-Algorithmen in gleicher Datei wie Implementierung
- Alle FAS-Algorithmen durchlaufen die gleichen Tests
  - Definiert in *src/fas/feedback_arc_set.rs*  

# Benchmarks
## Testsystem
![Benchmark System](benches/test_system.png)

## Definition & Durchführung
```bash
cargo bench
```
- **Achtung!** Die Benchmarks laufen nicht parallel, sondern nur auf **einer* CPU
- Definition: *benches/benchmark.rs*

## Ergebnisse
- *some/path*

# Literatur
- PACE 2022
  - Input format: https://pacechallenge.org/2022/tracks/#input-format
  - Example cyclic graphs: https://pacechallenge.org/2022/01/12/public-instances/
- Feedback-Arc-Set:
  - Problem Description: https://en.wikipedia.org/wiki/Feedback_arc_set
  - Exact (greedy) Algorithm: https://www.mat.univie.ac.at/~herman/fwf-P27891-N32/minimum_feedback_arc_set.pdf
  - Heuristic (divide and conquer) Algorithm: https://link.springer.com/content/pdf/10.1023/A:1011315014322.pdf
  - Heuristic (simple, using count of incoming/outgoing arcs) Algorithm: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.47.7745&rep=rep1&type=pdf
- Graph-Bisection (used in Feedback-Arc-Set):
  - Problem Description: https://tracer.lcc.uma.es/problems/bisect/bisect.htm
  - Heuristic (Stochastical Evolution) Algorithm: See FAS Heuristic Paper
  - Heuristic (Dynamic Clustering) Algorithm: See FAS Heuristic Paper
- Strongly-Connected-Components (used in Feedback-Arc-Set):
  - Problem Description: https://en.wikipedia.org/wiki/Strongly_connected_component
  - Kosaraju's Algorithm: https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
  - Tarjan's Algorithm: https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm

# Bibliotheken
- Benchmarking: https://docs.rs/criterion/latest/criterion/
- Graph datastructure & Greedy Feedbac Arc Set Algorithm: https://docs.rs/petgraph/latest/petgraph/
- Coverage: https://github.com/xd009642/tarpaulin

