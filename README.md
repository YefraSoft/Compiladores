# Compiladores — Teoría de la Computación

Implementaciones en **C++** de tres problemas clásicos de **Teoría de la Computación**
(formales, autómatas y gramáticas), desarrolladas durante el curso.

| | |
|---|---|
| **Autor** | Eduardo Efraín García Suárez |
| **Curso** | Teoría de la Computación |
| **Lenguaje** | C++ (C++17 / C++20) |
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
| **Algoritmo CYK** | `Prueba_de_Membresia/`, `prueba_De_Membresia-Efrain_Garcia/CYK/` | Prueba de pertenencia a gramáticas libres de contexto en **FNC** |
| **Autómata de números reales** | `rSistem/` | **DFA** que reconoce literales numéricos en notación científica |
| **Operaciones con lenguajes** | `tests/` | Generación aleatoria de conjuntos de cadenas y operaciones de conjuntos |

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
cd Prueba_de_Membresia
g++ -std=c++17 main.cpp -o cyk
./cyk
```

### Ejemplo

```text
$ ./cyk
1+2*3
Alcanzable

$ ./cyk
???
Inalcanzable
```

### Nota sobre las carpetas duplicadas

`Prueba_de_Membresia/` y `prueba_De_Membresia-Efrain_Garcia/CYK/` contienen **copias
idénticas** del mismo `main.cpp` (difieren únicamente en los finales de línea: CRLF vs LF).
Ambas se conservan porque el repositorio es un archivo del estado original.

---

## 2. Autómata finito para números reales (`rSistem`)

Implementa un **autómata finito determinista (DFA)** de 8 estados que reconoce
números reales en notación científica:

```text
34.9        → reconocida
0E-43       → reconocida
0.43e+4     → reconocida
3.1416      → reconocida
3..1416     → rechazada
01416       → rechazada
Hola mundo  → rechazada
```

**Estructura de la clase `automaton`** (`rSistem/class/automaton.h`):

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
cd rSistem
g++ -std=c++17 main.cpp class/automaton.cpp -o rsystem
./rsystem
```

### Ejemplo

```text
$ ./rsystem
:> 4
34.9
0E-43
3..1416
Hola
Cadenas reconocidas: [ 0E-43 34.9 ]
Cadenas rechazadas: [ 3..1416 Hola ]
```

> [!NOTE]
> Originalmente este proyecto traía las cadenas de prueba **hardcodeadas** en el código.
> La versión de aquí las pide por entrada estándar (`stdin`).

---

## 3. Operaciones con lenguajes (`tests`)

Genera **aleatoriamente** conjuntos de cadenas sobre un alfabeto y calcula las
operaciones clásicas entre lenguajes.

- **Operaciones:** unión, intersección, diferencia, concatenación y potencia (`Aⁿ`).
- **Generación aleatoria:** se pide el tamaño del conjunto, la longitud de las cadenas,
  el exponente de la potencia y el rango de caracteres (`inicio`–`fin`).
- La cadena vacía se representa con el marcador `<void>`.
- Las operaciones de conjuntos usan **variadic templates con fold expressions**
  (`(...), ...`), por lo que aceptan un número arbitrario de conjuntos.

### Compilar y ejecutar

Requiere **C++20** por las fold expressions.

```bash
cd tests
g++ -std=c++20 main.cpp Funtions.cpp operationsStrings.cpp opwLanguages.cpp -o tests
./tests
```

### Ejemplo

```text
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
├── Prueba_de_Membresia/
│   └── main.cpp                          # Algoritmo CYK (copia 1)
├── prueba_De_Membresia-Efrain_Garcia/
│   └── CYK/
│       └── main.cpp                      # Algoritmo CYK (copia 2, idéntica)
├── rSistem/
│   ├── main.cpp                          # Programa principal
│   └── class/
│       ├── automaton.h                   # Clase automaton (declaración)
│       └── automaton.cpp                 # Clase automaton (implementación)
└── tests/
    ├── main.cpp                          # Menú principal
    ├── Funtions.h                        # Entrada del usuario e impresión
    ├── Funtions.cpp
    ├── operationsStrings.h               # Generación aleatoria y operaciones
    ├── operationsStrings.cpp
    └── opwLanguages.cpp                 # Orquestación de las operaciones
```

---

## Defectos conocidos

No se corrigen a propósito: el repositorio preserva el código original.

| Ubicación | Detalle |
|---|---|
| `automaton.h` | `bool finalStates[8];` no se inicializa; el constructor solo marca `1, 2, 4, 7` como finales. Los índices `0, 3, 5, 6` quedan **indeterminados**, lo que es comportamiento indefinido. Una entrada como `3.` termina en el estado 3 y lee memoria sin inicializar. |
| `automaton.cpp` | `fori()` itera con `i < 5`, por lo que solo recorre los estados `0`-`4` de los 8 existentes. El método tampoco se invoca desde `main()`. |
| `rSistem/main.cpp` | Usa `string` sin incluir `<string>`; compila gracias a los includes transitivos de `automaton.h`. |
| `tests/main.cpp` | `Funtions::languageOperations()` se ejecuta **antes** del bucle del menú, así que la opción `1` solo limpia la pantalla y la opción `2` repite el cálculo. |
| `tests/*.h` | Nombre de la clase `Funtions` con `t` (debería ser `Functions`), igual que `opwLanguages.cpp`. Nombres heredados, conservados. |
| `Prueba_de_Membresia/main.cpp` | La gramática y el símbolo de inicio (`T` = 84) están hardcodeados; `printCYKTable()` quedó comentado. |
| Codificación | Los archivos mezcla finales de línea **CRLF y LF**, y los originales incluían `README.md` en **UTF-16** e ISO-8859-1 en algunos `.cpp`. Se preserva tal cual. |

---

## Notas

- El repositorio contiene **únicamente código fuente** (`.cpp` / `.h`). Los `.zip`
  de respaldo, los proyectos de Visual Studio (`.vcxproj`) y los binarios de
  compilación (`.obj`, `.pdb`, `.ilk`, `x64/`) están excluidos por `.gitignore`.
- No hay sistema de pruebas automatizadas ni archivos `CMakeLists.txt` /
  `Makefile`: cada proyecto se compila manualmente con los comandos indicados arriba.
- Los tres proyectos tienen un `main()` independiente, por lo que **no se pueden
  compilar juntos**; hay que compilar y ejecutar uno por uno.
