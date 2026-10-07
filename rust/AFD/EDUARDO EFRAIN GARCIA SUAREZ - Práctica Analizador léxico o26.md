# Práctica Analizador léxico 

# Integrantes:       	

# Instrucciones

Implementa un analizador léxico (scanner) para el siguiente diagrama de estados, puedes utilizar el lenguaje de programación de tu preferencia y utilizar la estrategia que decidas para su implementación. La solución que implementes es exactamente este diagrama, no añadas reglas adicionales, sigue la especificación.  
   
Clases de tokens son los estados finales en la imagen, la etiqueta muestra el nombre del token e indica si para detectar su patrón utilizó un  símbolo adicional marcándolo con \*.  En la impresión ignoramos los tokens de espacio.

## 

## Ejemplo 1 con error 

Entrada : 4 \+3.4 \+98.1E5  
Salida: Tokens  indicando los números  encontrados  
Ejem: \<entero,4\>\<suma, \+\> \< flotante,3.4 \>\<suma, \+\>  \<error,98.1E5\>

## 

## Ejemplo 2 sin error 

Entrada : 4 \+3.4 \+98.1E+5  
Salida: Tokens  indicando los números  encontrados  
Ejem: \<entero,4\>\<suma, \+\> \< flotante,3.4 \>\<suma, \+\>  \<exponente,98.1E+5\>

## Ejemplo 3 con error 

Entrada : 4 \+3.4 \+98.1E5  
Salida: Tokens  indicando los números  encontrados  
Ejem: \<entero,4\>\<suma, \+\> \< flotante,3.4 \>\<suma, \+\>  \<error,98.1E5\>	

## Ejemplo 4 con error

Entrada :  4 3\. 98 E5

Salida: Tokens  indicando los números  encontrados

Ejem: \<entero,  4\> \< error , 3\. \>  \< entero ,98\> \<error, E\> \< entero, 5\>

$\ \ \ \ \ {\ }$

## a.	Describe tu  algoritmo.

## 

## b. Código

 

## c.   Enlace con código  con el siguiente caso : 4 \+3.4 \+98.1E5 listo para compilar en [https://www.onlinegdb.com/](https://www.onlinegdb.com/)

 

## d.	Casos de prueba(captura de pantalla para cada uno):

1\.   4.8++3.4+ 98.1E+5

2\.   4 3.4 98 E-5

3\.   5.25+23\*325Ee12.5