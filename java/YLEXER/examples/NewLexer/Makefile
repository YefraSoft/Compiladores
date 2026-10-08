# Genera, compila y ejecuta NewLexer con el jar de JFlexFork (../../target),
# sin Maven. El lexer generado va a build/, nunca a src/.
JFLEX_JAR := ../../target/jflex-1.5.0-fork.jar
SRC       := src/main/java/Jflextest
GEN       := build/generated/Jflextest
CLASSES   := build/classes

.PHONY: all generate build run gui clean

all: build

$(JFLEX_JAR):
	mvn -q -f ../../pom.xml package -DskipTests

generate: $(GEN)/NewLexer.java

$(GEN)/NewLexer.java: $(SRC)/NewLexer.flex $(JFLEX_JAR)
	mkdir -p $(GEN)
	java -jar $(JFLEX_JAR) -q --nobak -d $(GEN) $<

build: generate
	javac -encoding UTF-8 -d $(CLASSES) $(SRC)/*.java $(GEN)/NewLexer.java

run: build
	java -cp $(CLASSES) Jflextest.TestConsole

gui: build
	java -cp $(CLASSES) Jflextest.TestClass

clean:
	rm -rf build target
