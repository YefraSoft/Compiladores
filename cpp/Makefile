# ==============================================================================
#  Compiladores  --  Teoria de la Computacion
#  Makefile raiz: compila y ejecuta los tres proyectos de cpp/.
#
#  Uso:   make                 compila los tres proyectos
#         make run             ejecuta los tres en orden (lee de stdin)
#         make build-<P>       compila un proyecto
#         make run-<P>         ejecuta un proyecto
#         make clean           borra los artefactos de los tres
#         make distclean       clean + borra cpp/build/
#         make help            lista los objetivos disponibles
#
#  Proyectos: CYK (algoritmo CYK), AFD (automato finito), OCL (conjuntos de
#  lenguajes).  Cada uno tiene su propio Makefile; este solo coordina.
# ==============================================================================

PROJECTS := CYK AFD OCL

CXX      ?= g++
STD      ?= c++17
CXXFLAGS ?= -std=$(STD) -Wall -Wextra -O2
LDFLAGS  ?=

BUILD_TARGETS := $(addprefix build-,$(PROJECTS))
RUN_TARGETS   := $(addprefix run-,$(PROJECTS))

MAKEFLAGS += --no-print-directory

.PHONY: all run clean distclean help $(BUILD_TARGETS) $(RUN_TARGETS)

all: $(BUILD_TARGETS)

# Regla de patron estatico: delega la compilacion al Makefile del proyecto.
$(BUILD_TARGETS): build-%:
	@$(MAKE) -C $* CXX="$(CXX)" CXXFLAGS="$(CXXFLAGS)" LDFLAGS="$(LDFLAGS)"

$(RUN_TARGETS): run-%: build-%
	@$(MAKE) -C $* run

run:
	@for p in $(PROJECTS); do \
		echo "===== $$p ====="; \
		$(MAKE) -C $$p run; \
		echo; \
	done

clean:
	@for p in $(PROJECTS); do $(MAKE) -C $$p clean; done

distclean: clean
	rm -rf build

help:
	@echo "make                 compila los tres proyectos"
	@echo "make run             ejecuta los tres en orden (consume stdin)"
	@echo "make build-<P>       compila un proyecto: $(PROJECTS)"
	@echo "make run-<P>         ejecuta un proyecto: $(PROJECTS)"
	@echo "make clean           borra los artefactos de los tres"
	@echo "make distclean       clean + borra cpp/build/"
	@echo "make help            esta ayuda"
	@echo
	@echo "CXX      = $(CXX)"
	@echo "STD      = $(STD)"
	@echo "CXXFLAGS = $(CXXFLAGS)"
