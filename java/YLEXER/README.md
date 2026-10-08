# JFlexFork (YLEXER)

JFlex 1.5.0 recompilado desde sus fuentes originales (distribución oficial `jflex-1.5.0`) con **Java 25**,
como proyecto Maven independiente (sin `parent.xml` ni plugins de JFlex/CUP).

## Compilar

```bash
mvn clean verify          # genera target/jflex-1.5.0-fork.jar y corre los 53 tests
bin/jflex --version       # lanzador (bin/jflex.bat en Windows)
```

`generate-sources` hace el *bootstrap* con los binarios originales de `bootstrap/`:

- `java-cup-11a.jar`: `src/main/cup/LexParse.cup` → `LexParse.java`, `sym.java`
- `jflex-1.5.0.jar`: `src/main/jflex/LexScan.flex` (+ `skeleton.nested`) → `LexScan.java`

Lo generado queda en `target/generated-sources/jflex/`.

## Diferencias con el original

- Bytecode Java 25 (major 69) en vez de Java 6; mismas 116 clases/recursos que el jar original.
- `jflex/gui/MainFrame.java`: `Thread.stop()` (lanza `UnsupportedOperationException` desde Java 20)
  se reemplazó por `Thread.interrupt()`.
- Dependencias actualizadas: Ant 1.10.15, JUnit 4.13.2.
- `--version` sigue diciendo `1.5.0-SNAPSHOT`, igual que el jar original.

La salida generada es idéntica byte a byte a la del jar original (probado con
`NewLexer.flex` y `examples/java/java.flex`).

## Ejemplo (`examples/NewLexer`)

Proyecto aparte (su propio `pom.xml`) que usa el jar de JFlexFork. Requiere haber hecho
`mvn package` en esta carpeta antes.

```bash
cd examples/NewLexer
mvn package                                         # .flex -> target/generated-sources -> jar
echo 'if x = 3 + abc12' | java -jar target/newlexer-1.0.jar

make run          # alternativa sin Maven (consola)
make gui          # TestClass original (JOptionPane)
```

`NewLexer.java` es **generado**: nunca va en `src/`. Se edita `NewLexer.flex` y se regenera.

En IntelliJ, `examples/NewLexer/pom.xml` debe estar importado como proyecto Maven
(*Maven → + → Add Maven Project*); tras importarlo, *Generate Sources and Update Folders*.
