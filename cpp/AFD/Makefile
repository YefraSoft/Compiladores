# ==============================================================================
#  Proyecto: AFD  --  Automato finito determinista
#  Reconoce numeros reales en notacion cientifica (8 estados).
#
#  Uso:   make          compila
#         make run      compila y ejecuta
#         make clean    borra los artefactos
# ==============================================================================

CXX      ?= g++
STD      ?= c++17
CXXFLAGS ?= -std=$(STD) -Wall -Wextra -O2
LDFLAGS  ?=

TARGET := build/afd
SRCS   := main.cpp class/automaton.cpp
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
