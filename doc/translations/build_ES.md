# Mugle - Compilación, configuración y ejecución

*Lea esto en otros idiomas: [English](../build.md), [日本語](build_JP.md), [Korean](build_KR.md), [简体中文](build_ZH-CN.md).*

## Plataformas soportadas

En un largo plazo, es probable que la mayoría de las plataformas sean compatibles en cierta medida.
El lenguaje de programación de Mugle `rust` ha compilado metas para la mayoría de las plataformas.

¿Qué funciona hasta ahora?

* Linux x86\_64 y MacOS [mugle + mining + development]
* Todavía no funciona con windows 10 [mugle kind-of builds. No mining yet. Help wanted!]

## Requisitos

* rust 1.34+ (usa [rustup]((https://www.rustup.rs/))- por ejemplo, `curl https://sh.rustup.rs -sSf | sh; source $HOME/.cargo/env`)
  * Si rust está instalado, puede simplemente actualizar la versión con  `rustup update`
* clang
* ncurses y libs (ncurses, ncursesw5)
* zlib libs (zlib1g-dev or zlib-devel)
* pkg-config
* libssl-dev
* linux-headers (reportado como necesario en Alpine linux)
* llvm

Para las distribuciones basadas en Debian (Debian, Ubuntu, Mint, etc), todo en un comando (exceptuando Rust):

```sh
apt install build-essential cmake git libgit2-dev clang libncurses5-dev libncursesw5-dev zlib1g-dev pkg-config libssl-dev llvm
```

Para las Mac:

```sh
xcode-select --install
brew install --with-toolchain llvm
brew install pkg-config
brew install openssl
```

## Pasos para la compilación

```sh
git clone https://github.com/mugleproject/mugle.git
cd mugle
cargo build --release
```

Mugle también puede compilarse en modo debug (sin la etiqueta `--release`, pero usando la etiqueta `--debug` o `--verbose`) esto hará que la sincronización rápida sea excesivamente lenta debido a la gran sobrecarga de las operaciones criptográficas.

## Errores de compilación

Vea [Solución de problemas](https://github.com/mugleproject/docs/wiki/Troubleshooting)

## ¿Qué se ha compilado?

Con una compilación finalizada se obtiene:

* `target/release/mugle` - los binarios principales de mugle

Todos los datos, configuración y archivos de registro creados y utilizados por Mugle se encuentran en el directorio oculto `~/.mugle` (bajo el directorio home del usuario) por defecto. Puede modificar toda la configuración editando el archivo `~/.mugle/main/mugle-server.toml`.

También es posible hacer que Mugle cree sus propios archivos de datos en el directorio actual. Para ello ejecute:

```sh
mugle server config
```

Lo que generará un archivo `mugle-server.toml` en el directorio actual, preconfigurado para usar el directorio actual para todos sus datos. Ejecutando Mugle desde un directorio que contiene el archivo `mugle-server.toml` usará los valores de ese archivo en lugar de los valores por defecto de `~/.mugle/main/mugle-server.toml`.

Durante las pruebas, ponga el binario de Mugle en su ruta de esta manera:

```sh
export PATH=/path/to/mugle/dir/target/release:$PATH
```

Donde `path/to/mugle/dir` es su ruta absoluta al directorio raíz de la instalación de Mugle.

Puede ejecutar `mugle` directamente (pruebe `mugle help` para más opciones).

## Configuración

Mugle se ejecuta con valores predeterminados, y puede configurarse aún más a través del archivo `mugle-server.toml`. Este fichero es generado por mugle en su primera ejecución, y contiene documentación sobre cada opción disponible.

Aunque se recomienda que realice toda la configuración de mugle server a través de `mugle-server.toml`, también es posible suministrar cambios de comandos para mugle que anulan cualquier configuración en el archivo.

Para obtener ayuda sobre los comandos de mugle y sus cambios intente:

```sh
mugle help
mugle server --help
mugle client --help
```

## Docker

```sh
docker build -t mugle -f etc/Dockerfile .
```

Puede ubicar la caché de Mugle para que se ejecute dentro del contenedor

```sh
docker run -it -d -v $HOME/.mugle:/root/.mugle mugle
```
## Compilación multiplataforma

Rust (cargo) puede compilar Mugle para muchas plataformas, así que en teoría ejecutar `mugle` como un nodo de validación en un dispositivo de baja potencia podría ser posible. Para hacer una compilación cruzada `mugle` en una plataforma x86 Linux y generar binarios de ARM, por ejemplo para Raspberry-pi.

## Usando Mugle

La página de la wiki [Cómo usar mugle](https://github.com/mugleproject/docs/wiki/How-to-use-mugle) y las páginas de enlaces tienen más información sobre las características que disponemos, resolución de problemas, etc.

## Minando en Mugle

Tenga en cuenta que todas las funciones de minería de Mugle se han trasladado a un paquete independiente llamado [mugle_minner](https://github.com/mugleproject/mugle-miner). Una vez que el nodo de mugle esté listo y funcionando, puede empezar a minar compilando y ejecutando mugle-miner con su nodo Mugle en funcionamiento.
