# Compiladores — Teoría de la Computación

Implementaciones en **C++** de tres problemas clásicos de **Teoría de la Computación**
(formales, autómatas y gramáticas), desarrolladas durante el curso.

| | |
|---|---|
| **Autor** | Eduardo Efraín García Suárez |
| **Curso** | Teoría de la Computación |
| **Lenguaje** | C++ (C++17) |
| **Fecha** | 2024 |
| **Estado** | 🔴 **Código legacy** — se conserva tal cual, sin refactorizar |

> [!WARNING]
> **Código legacy.** Este repositorio es un archivo histórico: el código se sube
> **tal como estaba**, sin correcciones ni modernizaciones, para conservar el
> estado original de los trabajos. Se documentan abajo los defectos conocidos en
> lugar de arreglarlos.

---

## Contenido

| Proyecto | Carpeta | Descripción |
|---|---|---|
| **Algoritmo CYK** | `cpp/CYK/` | Prueba de pertenencia a gramáticas libres de contexto en **FNC** |
| **Autómata de números reales** | `cpp/AFD/` | **DFA** que reconoce literales numéricos en notación científica |
| **Operaciones con lenguajes** | `cpp/OCL/` | Generación aleatoria de conjuntos de cadenas y operaciones de conjuntos |

---

## Compilar y ejecutar

Todo el código C++ vive bajo `cpp/`. Hay un `Makefile` raíz y uno por proyecto.

```bash
make -C cpp            # compila los tres proyectos
make -C cpp help       # lista todos los objetivos
```

| Objetivo | Efecto |
|---|---|
| `make -C cpp` | compila CYK, AFD y OCL |
| `make -C cpp run` | ejecuta los tres en orden (consume `stdin`) |
| `make -C cpp build-<P>` | compila un proyecto: `CYK`, `AFD`, `OCL` |
| `make -C cpp run-<P>` | ejecuta un proyecto |
| `make -C cpp clean` | borra los artefactos de los tres |
| `make -C cpp distclean` | `clean` + borra `cpp/build/` |

Cada proyecto también se compila de forma independiente desde su carpeta:

```bash
cd cpp/CYK && make && make run
```

Variables de configuración: `CXX` (por defecto `g++`), `STD` (por defecto `c++17`),
`CXXFLAGS` (por defecto `-std=$(STD) -Wall -Wextra -O2`).

```bash
make -C cpp all STD=c++20 CXX=clang++
```

Los binarios y objetos se generan en `cpp/<proyecto>/build/` y quedan fuera del control
de versiones. Los `.d` (`-MMD -MP`) permiten recompilar solo lo que cambió al tocar
un `.h`.

---

## 1. Algoritmo CYK — Prueba de pertenencia a gramáticas

Determina si una cadena de entrada **pertenece** al lenguaje generado por una gramática
libre de contexto, llenando la tabla de CYK (*Cocke–Younger–Kasami*).

La gramática modela **expresiones aritméticas**:

```text
Q → E ? Q : Q | E
E → T + E | T - E | T
T → F * T | F / T | F
F → (Q) | -F | n
```

El programa la transforma a **Forma Normal de Chomsky (FNC)**, donde toda producción
es `X → a` o `Y → YZ`. Para evitar usar caracteres como identificadores, cada símbolo
se representa con su código ASCII como `int`:

- **Generativos:** `Q`=81, `E`=69, `T`=84, `F`=70
- **Terminales:** `?`=63, `:`=58, `+`=43, `-`=45, `*`=42, `/`=47, `(`=40, `)`=41, `n`=110
- **Rango auxiliar:** `X*` a partir de `1000` (terminales), `Y*` a partir de `2000` (generativos)

La gramática en FNC queda hardcodeada en `main()`, y la tabla de CYK se representa como
`map<int, map<int, set<int>>>` (`typedef table`).

### Compilar y ejecutar

```bash
make -C cpp run-CYK
```

### Ejemplo

```text
$ make -C cpp run-CYK
n+n
Alcanzable

$ make -C cpp run-CYK
???
Inalcanzable
```

> [!NOTE]
> Este proyecto vivía duplicado en `Prueba_de_Membresia/` y
> `prueba_De_Membresia-Efrain_Garcia/CYK/`, con un `main.cpp` **idéntico** en ambas
> (solo cambiaban los finales de línea: CRLF vs LF). Se conservó una sola copia en
> `cpp/CYK/` y se eliminó el duplicado.

---

## 2. Autómata finito para números reales (`cpp/AFD`)

Implementa un **autómata finito determinista (DFA)** de 8 estados que reconoce
números reales en notación científica:

```text
34.9        → reconocida
0.43e+4     → reconocida
3.1416      → reconocida
1.5e10      → reconocida
12          → reconocida
0E-43       → rechazada   (ver "Defectos conocidos")
3..1416     → rechazada
01416       → rechazada
Hola mundo  → rechazada
```

**Estructura de la clase `automaton`** (`cpp/AFD/class/automaton.h`):

```cpp
typedef std::map<char, unsigned>    event;      // símbolo  -> estado destino
typedef std::map<unsigned, event>   transition; // estado    -> eventos
transition delta;                                 // función de transición
int startState = 0;                               // estado inicial
bool finalStates[8];                              // estados de aceptación
```

El método `zeroToNineEvents()` evita repetir a mano las 10 transiciones de los dígitos
(`0`-`9`) hacia un mismo destino. `run()` consume la cadena completa y devuelve `true`
solo si el estado final alcanzado es de aceptación.

### Compilar y ejecutar

```bash
make -C cpp run-AFD
```

### Ejemplo

```text
$ make -C cpp run-AFD
:> 4
34.9
0.43e+4
3..1416
Hola
Cadenas reconocidas: [ 34.9 0.43e+4 ]
Cadenas rechazadas: [ 3..1416 Hola ]
```

> [!NOTE]
> Originalmente este proyecto traía las cadenas de prueba **hardcodeadas** en el código.
> La versión de aquí las pide por entrada estándar (`stdin`).

---

## 3. Operaciones con lenguajes (`cpp/OCL`)

Genera **aleatoriamente** conjuntos de cadenas sobre un alfabeto y calcula las
operaciones clásicas entre lenguajes.

- **Operaciones:** unión, intersección, diferencia, concatenación y potencia (`Aⁿ`).
- **Generación aleatoria:** se pide el tamaño del conjunto, la longitud de las cadenas,
  el exponente de la potencia y el rango de caracteres (`inicio`–`fin`).
- La cadena vacía se representa con el marcador `<void>`.
- Las operaciones de conjuntos usan **variadic templates con fold expressions**
  (`(...), ...`), por lo que aceptan un número arbitrario de conjuntos.

### Compilar y ejecutar

Las *fold expressions* de C++17 son suficientes; no hace falta C++20.

```bash
make -C cpp run-OCL
```

### Ejemplo

```text
$ make -C cpp run-OCL
Enter the length of the string set: 5
Enter a string lengt: 4
Enter a string pow: 2
Enter the first element of the string: a
Enter the last element of the string: c
A: [ ab aa ac ]
B: [ bc cb ]
Union: [ aa ab ac bc cb ]
Intersection: [ ]
Difference: [ bc cb ]
Concatenation: [ ... ]
A power 2: [ abaa abab ... ]
B power 2: [ bcbc bccb ... ]
```

> [!NOTE]
> Este proyecto se desarrolló originalmente en **Visual Studio 2022 (MSVC, C++20)**.
> Los archivos `.vcxproj` y la carpeta `x64/` **no** están en el repositorio por
> decisión propia; los comandos de arriba compilan el mismo código con `g++`.

---

## Estructura del repositorio

```text
Compiladores/
├── .gitignore
├── README.md
└── cpp/
    ├── Makefile                           # Coordina los tres proyectos
    ├── CYK/
    │   ├── Makefile
    │   ├── main.cpp                       # Algoritmo CYK
    │   └── build/                         # Generado: cyk, *.o, *.d
    ├── AFD/
    │   ├── Makefile
    │   ├── main.cpp                       # Programa principal
    │   ├── class/
    │   │   ├── automaton.h                # Clase automaton (declaración)
    │   │   └── automaton.cpp              # Clase automaton (implementación)
    │   └── build/                         # Generado: afd, *.o, *.d
    └── OCL/
        ├── Makefile
        ├── main.cpp                       # Menú principal
        ├── Funtions.h                     # Entrada del usuario e impresión
        ├── Funtions.cpp
        ├── operationsStrings.h            # Generación aleatoria y operaciones
        ├── operationsStrings.cpp
        ├── opwLanguages.cpp               # Orquestación de las operaciones
        └── build/                         # Generado: ocl, *.o, *.d
```

---

## Defectos conocidos

No se corrigen a propósito: el repositorio preserva el código original.

### Compilación

Todos los proyectos compilan sin errores con `g++ -std=c++17 -Wall -Wextra`. Solo
aparecen advertencias, ninguna bloquea el build:

| Ubicación | Advertencia |
|---|---|
| `cpp/CYK/main.cpp:8` | `-Wnon-c-typedef-for-linkage`: `produ` es un `struct` anónimo con inicializadores por defecto, renombrado vía `typedef`. |
| `cpp/CYK/main.cpp:148` | `-Wunused-variable`: `prod` no se usa en el `for`. |
| `cpp/OCL/Funtions.cpp:19` | `-Wunused-parameter`: `usePow` no se usa en `userImputs(OperationStrings&, bool)`. |

### Lógica

| Ubicación | Detalle |
|---|---|
| `cpp/CYK/main.cpp:139` | Usa `\|\|` donde CYK exige `&&`. Con `\|\|` basta con que **uno** de los dos rangos contenga el no-terminal, así que se insertan no-terminales de más. Debería comprobar que el hijo izquierdo **y** el derecho derivan `prod.G3`. |
| `cpp/CYK/main.cpp:150` | Valida el símbolo inicial `1084` (`T`), pero el axioma de la gramática es `Q` = `1081`. |
| `cpp/CYK/main.cpp:148` | El `for` que envuelve la comprobación final es redundante: `count(1084)` no depende de `prod`. |
| `cpp/CYK/main.cpp:119,150` | Con entrada vacía, `n == 0` y se indexa `virTable[0][-1]`: comportamiento indefinido. |
| `cpp/OCL/main.cpp:52` | El menú **no tiene forma de salir**: el bucle es `while (option != 0)` pero el `switch` no incluye `case 0`, así que la opción `0` cae al `default`, que fija `option = -1`. Al agotarse `stdin`, `cin >> option` deja `option` en `0` y el programa entra en bucle infinito. |
| `cpp/OCL/main.cpp:25` | `languageOperations()` se ejecuta **antes** del bucle del menú, así que la opción `1` solo limpia la pantalla y la opción `2` repite el cálculo. |
| `cpp/OCL/operationsStrings.cpp:7-11` | Al generar una cadena vacía se inserta `<void>` **y** también `""`, por lo que ambos aparecen en el conjunto. |
| `cpp/AFD/class/automaton.h:11` | `bool finalStates[8];` no se inicializa; el constructor solo marca `1, 2, 4, 7` como finales. Los índices `0, 3, 5, 6` quedan **indeterminados**, lo que es comportamiento indefinido. Una entrada como `3.` termina en el estado 3 y lee memoria sin inicializar. |
| `cpp/AFD/class/automaton.cpp:5-18` | El DFA **no admite exponente sin punto decimal**: desde el estado 2 (parte entera) la única salida es `.`, por lo que `0E-43` y `1e5` se rechazan, aunque el README original los daba por reconocidos. |
| `cpp/AFD/class/automaton.cpp:28` | `fori()` itera con `i < 5`, por lo que solo recorre los estados `0`-`4` de los 8 existentes. El método tampoco se invoca desde `main()`. |
| `cpp/AFD/main.cpp:1` | Usa `string` sin incluir `<string>`; compila gracias a los includes transitivos de `automaton.h`. |
| `cpp/OCL/*.h` | Nombre de la clase `Funtions` con `t` (debería ser `Functions`), igual que `opwLanguages.cpp`. Nombres heredados, conservados. |
| `cpp/CYK/main.cpp:43-107` | La gramática y el símbolo de inicio están hardcodeados; `printCYKTable()` quedó comentado. |
| Codificación | Los archivos mezcla finales de línea **CRLF y LF**, y algunos `.cpp` originales estaban en ISO-8859-1. Se preserva tal cual. |

---

## Notas

- El repositorio contiene **únicamente código fuente** (`.cpp` / `.h`) y los `Makefile`.
  Los `.zip` de respaldo, los proyectos de Visual Studio (`.vcxproj`) y los binarios de
  compilación (`.obj`, `.pdb`, `.ilk`, `x64/`) están excluidos por `.gitignore`.
- No hay sistema de pruebas automatizadas. Cada proyecto se valida ejecutándolo
  manualmente; los tres tienen un `main()` independiente, por lo que **no se pueden
  compilar juntos** en un mismo binario.
- Las carpetas se movieron con `git mv` para conservar el historial: las versiones
  originales eran `Prueba_de_Membresia/`, `rSistem/` y `tests/`.
