# OCL-VF — Versión anterior (archivo `.zip` de respaldo)

> [!IMPORTANT]
> **Código histórico, preservado tal cual. No se corrige.**
> No hay `Makefile` en esta carpeta a propósito: este proyecto está fuera del
> sistema de build de `cpp/`. Ver [Estado](#estado) más abajo.

Esta carpeta contiene la **versión anterior** del proyecto de *Operaciones con
lenguajes*, extraída del archivo de respaldo
`Operaciones con lenguajes - Efrain Garcia.zip` (fechado el **12-04-2024**).

El proyecto que sí se compila y mantiene está en [`cpp/OCL/`](../cpp/OCL/).
Esta carpeta se conserva únicamente como **referencia histórica** para comparar
las dos versiones.

---

## Estructura

```text
OCL-VF/
├── libraryTest.cpp                          # main() con el menú DESACTIVADO
├── funtionExamples/
│   ├── Funtions.h                           # incluye ../operationsLibrary/...
│   ├── languageOperations.cpp               # == cpp/OCL/opwLanguages.cpp
│   └── utilities.cpp                        # == cpp/OCL/Funtions.cpp (versión vieja)
└── operationsLibrary/
    ├── operationsStrings.h
    └── operationsStrings.cpp
```

`libraryTest.cpp` está en la **raíz** del zip, no dentro de `funtionExamples/`.

### Compilar (opcional, solo para verificar)

```bash
cd OCL-VF
g++ -std=c++17 libraryTest.cpp \
    funtionExamples/languageOperations.cpp \
    funtionExamples/utilities.cpp \
    operationsLibrary/operationsStrings.cpp -o /tmp/ocl-vf
```

Compila limpio con `g++ -std=c++17`. No genera artefactos en el repositorio.

---

## Por qué se conserva

Al comparar ambas versiones aparecieron **diferencias de comportamiento** que
explican los defectos documentados de `cpp/OCL/`. Ninguna se corrigió: el
repositorio sigue siendo legacy.

---

## Hallazgos

### 1. El menú ya estaba desactivado en esta versión

`libraryTest.cpp:22-55` — el menú completo está comentado, incluida la
declaración de la variable:

```cpp
int main() {
  // int option;                 <-- línea 23, comentada

  Funtions::languageOperations();

  /*do                            <-- línea 27
  {
      displayMenu();
      cout << "Select an option: ";
      cin >> option;
      switch (option)
      {
      ...
      } while (option != 0);*/   <-- línea 53
  return 0;
}
```

Es decir: **el autor ya se había topado con el bucle infinito y optó por
desactivar el menú entero.**

La versión actual ([`cpp/OCL/main.cpp`](../cpp/OCL/main.cpp)) **descomentó el
menú pero conservó el defecto**: mantiene `while (option != 0)`
(`main.cpp:52`) sin ningún `case 0`, y el `default` (`main.cpp:45-47`) fuerza
`option = -1`. Al re-activarlo, el bucle infinito regresó.

**Origen del cuelgue:** con la opción `0` el `switch` cae en `default`, que fija
`option = -1`; al agotarse `stdin`, `cin >> option` deja `option` en `0` y el
programa entra en bucle eterno. **El menú no tiene forma de salir.**

### 2. `<void>` no se insertaba como elemento del conjunto

Aquí ε (la cadena vacía) se representa siempre con `""`:

| | `getRandomSymbolStringSet()` | `powerSet(pow=0)` |
|---|---|---|
| `OCL-VF/operationsLibrary/operationsStrings.cpp` | `stringSet.insert("")` (línea 8) | `result.insert("")` (línea 84) |
| `cpp/OCL/operationsStrings.cpp` | `stringSet.insert("<void>")` (línea 8) | `result.insert("<void>")` (línea 81) |

En la versión actual, `operationsStrings.cpp:7-10` inserta `"<void>"` **y
después** cae en el `insert(newStr)` con `newStr == ""`. Resultado: el conjunto
contiene **dos** entradas para ε (`""` y `"<void>"`), y solo una es válida.

En esta versión `<void>` es solo un **token de impresión** en `printSet`
(`utilities.cpp:50-75`), no un elemento del conjunto. Por eso aquí no hace
falta el filtro que la versión actual sí necesita en `concatSet`.

**Consecuencia: la concatenación de `cpp/OCL/` es incorrecta.** El filtro de
`operationsStrings.cpp:70` descarta cualquier producto que toque ε, pero **ε es
el neutro de la concatenación**, así que esos productos son válidos. Medido con
`A = {a,b,c}`, `B = {"", "<void>", bb, c}`:

```text
Concatenation: [ abb ac bbb bc cbb cc ]        <-- lo que produce cpp/OCL/
Concatenation: [ a b c abb ac bbb bc cbb cc ]  <-- lo correcto
```

Faltan `a`, `b`, `c` (los productos `a·ε`, `b·ε`, `c·ε`). Con el diseño de esta
versión (`ε = ""`, sin filtro) el resultado sí es correcto.

### 3. Defectos que esta versión **también** tiene

Estos **no** se pueden arreglar con este archivo; hay que escribir el arreglo.

**`powerSet` siempre devuelve A², nunca Aⁿ.** A `concatSet` le falta el
`base = rest;` final que sí tienen `intersectionSet` (línea 43) y
`differenceSet` (línea 60) en `cpp/OCL/operationsStrings.cpp`. Medido con el
binario de **esta** versión (A = `{a}`, `pow = 3`):

```text
A power 3: [ aa ]     <-- debería ser { aaa }
```

Solo se manifiesta con `pow >= 3`; con `pow = 2` el resultado coincide, por eso
el ejemplo del README no lo delata.

**`differenceSets` devuelve Y\X en vez de X\Y.** En `opwLanguages.cpp:17` la
llamada es `differenceSets(setB, setA)`, así que la etiqueta "Difference"
imprime `A\B`.

**`intersectionSets` con 3+ conjuntos es incorrecto**: solo itera el conjunto
nuevo y nunca borra los sobrantes. Latente, porque el programa solo usa 2
conjuntos.

**No hay `srand` en ninguna de las dos versiones**: la "generación aleatoria"
produce siempre el mismo resultado.

### 4. Un defecto que esta versión tiene y la actual **no**

**No copies `randomSymbolSequence` desde aquí.** En
`operationsLibrary/operationsStrings.cpp:15-23`:

```cpp
string OperationStrings::randomSymbolSequence() {
    string str;
    int lengt = rand() % stringsLengt;   // línea 17: ANTES de la guarda
    if (stringsLengt > 0) {              // línea 18
        for (int i = 0; i < lengt; i++) { ... }
    }
    return str;
}
```

`rand() % stringsLengt` se evalúa **antes** del `if (stringsLengt > 0)`, así que
hay **división entre cero** si el usuario escribe `0` como longitud. La versión
actual (`cpp/OCL/operationsStrings.cpp:15-22`) integra la expresión dentro del
bucle ya protegido, y es correcta.

---

## Resumen de diferencias

| Aspecto | `OCL-VF/` (2024-04-12) | `cpp/OCL/` (actual) |
|---|---|---|
| Menú | desactivado (comentado) | activo, pero con bucle infinito |
| ε en los conjuntos | `""` | `""` **y** `"<void>"` (duplicado) |
| `<void>` | solo al imprimir | elemento real del conjunto |
| Filtro en `concatSet` | innecesario | necesario, pero excluye `a·ε` |
| `powerSet` con `pow >= 3` | devuelve A² (bug) | devuelve A² (bug) |
| `differenceSets(A,B)` | devuelve B\A (bug) | devuelve B\A (bug) |
| `srand` | ausente | ausente |
| División por cero con longitud `0` | **sí** | no |
| Codificación de los comentarios | UTF-8 | ISO-8859-1 (`Teor�a`) |
| Estructura | `operationsLibrary/` + `funtionExamples/` | plana |

Los archivos `funtionExamples/languageOperations.cpp` y
`operationsLibrary/operationsStrings.h` son **idénticos** a sus equivalentes
actuales, salvo la indentación y la codificación de los comentarios en español.

---

## Referencias

- Versión mantenida: [`cpp/OCL/`](../cpp/OCL/) — compilar con
  `make -C cpp run-OCL`; ver los defectos conocidos en el
  [README principal](../README.md)
- `libraryTest.cpp` es el equivalente de `cpp/OCL/main.cpp`
- `funtionExamples/utilities.cpp` es el equivalente de `cpp/OCL/Funtions.cpp`
