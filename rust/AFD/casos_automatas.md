# Casos de automatas

## AFD valido: termina en `a`

Archivo: `valid_termina_en_a.yeff`.

| Cadenas aceptadas | Cadenas rechazadas |
| --- | --- |
| `a` | cadena vacia |
| `ba` | `b` |
| `abba` | `aab` |

## AFD invalido: dos estados iniciales

Archivo: `invalid_dos_iniciales.yeff`.

Tiene `q0s` y `q2s`, por lo que el programa debe rechazar su construccion con `More than one start state.` No se pueden evaluar cadenas aceptadas o rechazadas porque no existe un AFD valido.

## AFD invalido: transicion no determinista

Archivo: `invalid_no_determinista.yeff`.

Define dos transiciones para `q0` con el simbolo `a`. El programa debe rechazar su construccion con `Non deterministic transition.` No se pueden evaluar cadenas aceptadas o rechazadas porque no existe un AFD valido.
