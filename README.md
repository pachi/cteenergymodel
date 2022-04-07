# CTE Energy Model

Modelo energético para cálculo de indicadores de eficiencia energética

Este conjunto de librerías y aplicaciones busca definir un formato de intercambio sencillo para la modelización energética orientada a la evaluación y cumplimiento de indicadores reglamentarios basados en la EPBD y el CTE.

## bemodel

La biblioteca ```bemodel``` define la estructura de datos y métodos del modelo energético básico.

Este modelo se usa en las aplicaciones ```hulc2model``` y [`EnvolventeCTE`](https://pachi.github.io/envolventecte).

## hulc

La biblioteca ```hulc``` permite la intepretación de archivos usados por la Herramienta Unificada LIDER-CALENER (HULC).

## hulc2model

La aplicación ```hulc2model```  permite exportar los datos de un proyecto de la `Herramienta unificada LIDER-CALENER (HULC)` al formato JSON que utiliza la aplicación web para el cálculo de parámetros energéticos de la envolvente térmica [`EnvolventeCTE`](https://pachi.github.io/envolventecte).

Esta versión está preparada para funcionar con las versiones de HULC adaptadas al CTE DB-HE 2019.

### Instalación

En la [página de versiones publicadas del proyecto](http://github.com/pachi/hulc2model/releases) puede encontrar los archivos necesarios para el funcionamiento del programa, que no necesita instalación.

Los archivos distribuidos permiten el uso de la aplicación en sistemas GNU/Linux y MS-Windows:

- `hulc2model` - ejecutable para GNU/Linux
- `hulc2model.exe` - ejecutable para MS-Windows
- `hulc2model.zip` - código fuente comprimido en formato ZIP
- `hulc2model.tar.gz` - código fuente comprimido en formato .tar.gz

### Uso

Esta aplicación se utiliza desde la línea de comandos, y debe inidicar como parámetro el directorio del proyecto de HULC que desea exportar, redirigiendo la salida a un archivo para su posterior importación desde la interfaz web de EnvolventeCTE:

```bash
    hulc2model datos/proyecto/hulc > salida.json
```

En sistemas MS-Windows al ejecutar el programa se lanza una interfaz gráfica simple en la que se puede indicar el directorio de proyecto de HULC sobre el que se quiere trabajar, y en el que se realizará la exportación del archivo `.json` generado.

## Licencia

Estas librerías y aplicaciones relacionadas son software libre y se distribuyen bajo una licencia MIT. Consulte el archivo LICENSE para el texto completo.

El código fuente se encuentra disponible en [http://github.com/pachi/cteenergymodel]

## Autores

Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>,  Daniel Jiménez González <danielj@ietcc.csic.es>, Marta Sorribes Gil <msorribes@ietcc.csic.es>
