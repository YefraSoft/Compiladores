# ==============================================================================
#  Proyecto: OCL  --  Operaciones con Conjuntos de Lenguajes
#  Generacion aleatoria de conjuntos de cadenas (union, interseccion, diferencia,
#  concatenacion y potencia) usando variadic templates con fold expressions.
#
#  Uso:   make          compila
#         make run      compila y ejecuta
#         make clean    borra los artefactos
# ==============================================================================

CXX      ?= g++
STD      ?= c++17
CXXFLAGS ?= -std=$(STD) -Wall -Wextra -O2
LDFLAGS  ?=

TARGET := build/ocl
SRCS   := main.cpp Funtions.cpp operationsStrings.cpp opwLanguages.cpp
OBJS   := $(SRCS:%.cpp=build/%.o)
DEPS   := $(OBJS:.o=.d)

.PHONY: all run clean

all: $(TARGET)

$(TARGET): $(OBJS)
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) $(OBJS) -o $@ $(LDFLAGS)

build/%.o: %.cpp
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -MMD -MP -c $< -o $@

run: $(TARGET)
	@./$(TARGET)

clean:
	rm -rf build

-include $(DEPS)
