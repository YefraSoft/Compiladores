#include "funtionExamples/Funtions.h"
#include <iostream>
#include <unistd.h>
using namespace std;

/*
 AUTOR: Eduardo Efrain Garcia Suarez
 CURSO: Teoría de la Computación
 PROGRAMA: Generación aleatoria de cadenas
 FECHA: 01/09/2024
*/

void displayMenu() {
  cout << "***********************************\n";
  cout << "*       Menu Options        *\n";
  cout << "***********************************\n";
  cout << "1. Generate random strings\n";
  cout << "2. String operations\n";
  cout << "***********************************\n\n";
}

int main() {
  // int option;

  Funtions::languageOperations();

  /*do
  {
      displayMenu();
      cout << "Select an option: ";
      cin >> option;
      switch (option)
      {
      case 1:
          (void)system("clear");
          cout << "1: Generate random strings" << endl
              << endl;
          cout << endl;
          break;
      case 2:
          cout << "2: String operations" << endl
              << endl;
          Funtions::languageOperations();
          cout << endl;
          break;
      default:
          cout << "Invalid option. Try again.\n";
          option = -1;
          cin.clear();
          cin.ignore(3, '\n');
          (void)system("clear");
      }
  } while (option != 0);*/
  return 0;
}