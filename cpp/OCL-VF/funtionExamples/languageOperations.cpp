#include "Funtions.h"
/*
 AUTOR: Eduardo Efrain Garcia Suarez
 CURSO: Teoría de la Computación
 PROGRAMA: Generación aleatoria de cadenas
 FECHA: 01/09/2024
*/

void Funtions::languageOperations() {
  OperationStrings operationStrings;
  Funtions::userImputs(operationStrings, true);
  set<string> setA = operationStrings.getRandomSymbolStringSet();
  set<string> setB = operationStrings.getRandomSymbolStringSet();
  Funtions::printSets(setA, setB);
  set<string> setUnion = OperationStrings::operationsSets::unionSets(setA, setB);
  set<string> setIntersection = OperationStrings::operationsSets::intersectionSets(setA, setB);
  set<string> setDifference = OperationStrings::operationsSets::differenceSets(setB, setA);
  set<string> setConcat = OperationStrings::operationsSets::concatSets(setA, setB);
  set<string> setPowA = OperationStrings::operationsSets::powerSet(setA, operationStrings.getPow());
  set<string> setPowB = OperationStrings::operationsSets::powerSet(setB, operationStrings.getPow());
  Funtions::printSet(setUnion, "Union");
  Funtions::printSet(setIntersection, "Intersection");
  Funtions::printSet(setDifference, "Difference");
  Funtions::printSet(setConcat, "Concatenation");
  string powerA = "A power " + to_string(operationStrings.getPow());
  string powerB = "B power " + to_string(operationStrings.getPow());
  Funtions::printSet(setPowA, powerA);
  Funtions::printSet(setPowB, powerB);
}