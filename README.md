# Haversine-alpha

A CLI tool to get the shortest distance (km) between multiple coordinates

## 1. Algorithm used
[Haversine formula](https://en.wikipedia.org/wiki/Haversine_formula)

![](/img/formula.png)


## 2. Instalation
### 2.1 Cargo

    cargo install haversine-alpha


### 2.2 Ready-to-use executable

|OS|Architecture| File*|
|--|--|--|
|Linux|x86_64|[haversine-alpha](https://github.com/costa86/haversine-alpha/blob/master/haversine-alpha)|
|Windows|x86_64|[haversine-alpha.exe](https://github.com/costa86/haversine-alpha/blob/master/haversine-alpha.exe)|


## 3. Usage

    Lorenzo Costa <http://www.costa86.tech>
    Gets the distance (Km) between multiple coordinates using Haversine formula

    USAGE:
        harversine --coordinates <LATITUDE,LONGITUDE>

    OPTIONS:
        -c, --coordinates <LATITUDE,LONGITUDE>
                Comma-separated pairs of latitudes and longitudes. Accepts more than 2 coordinates

        -h, --help
                Print help information

        -V, --version
                Print version information

## 4. Example

| Place | Latitude| Longitude  |
|--|--|--|
| Paris | 48.96817 | 2.34246 |
| Bruxels | 50.49890 | 4.71467 |
| Frankfurt | 50.29843 | 8.42832|


![map](/img/map.png)

```sh
./haversine-alpha -c 48.96817,2.34246,50.49890,4.71467,50.29843,8.42832
```

```sh
505.03
```
Just a reminder about coordinates:

||Min|Max|
|-|-|-|
|Latitude|-90|90
|Longitude|-180|180
