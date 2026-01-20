## Etiquetas de UI
label-rank = Rango
label-target = Objetivo
label-ip = IP
label-last-rtt = Últ. RTT
label-avg-rtt = Avg RTT
label-max = Máx
label-min = Mín
label-jitter = Jitter
label-loss = Pérdida

## Vista de Tabla
table-view-title = Vista de Tabla de PingWatch

## Vista de Punto
point-view-title = Vista de Punto de PingWatch
point-view-legend = Saludable
point-view-high-latency = Latencia Alta (más del 80% del máximo)
point-view-timeout = Timeout

## Vista de Sparkline
sparkline-view-title = Vista SparkLine de PingWatch
sparkline-view-note = El área en blanco significa timeout o error

## Vista de Gráfico
graph-view-title = Vista de Gráfico de PingWatch

## Mensajes de Error
error-target-required = Error: se requiere dirección IP objetivo o nombre de host
error-output-exists = Archivo de salida ya existe: {$path}
error-unable-shutdown = No se puede escuchar la señal de apagado: {$error}
error-ping-init-failed = host({$host}) error de ping, razón: falló la inicialización del ping, error: {$error}
error-ping-unknown = host({$host}) error de ping, razón: desconocida, error: {$error}
error-ping-recv-failed = host({$host}) error de ping, razón: falló la recepción, error: {$error}
error-encode-metrics = Error al codificar métricas: {$error}
error-serve-connection = Error al servir conexión: {$error}
error-accept-connection = Error al aceptar conexión: {$error}
error-restore-terminal = Error al restaurar terminal: {$error}

## Métricas y Unidades
unit-ms = ms
unit-percent = %
metric-less-than = < 0.01ms
metric-zero = 0.0ms

## Modo Exporter
exporter-mode-title = Modo Exporter de PingWatch
exporter-interval-help = Intervalo en segundos entre pings
exporter-port-help = Puerto HTTP de métricas Prometheus

## Ayuda de Línea de Comandos
arg-target-help = dirección IP objetivo o nombre de host para hacer ping
arg-count-help = Número de pings a enviar
arg-interval-help = Intervalo en segundos entre pings
arg-ipv6-help = Forzar el uso de IPv6
arg-multiple-help = Especificar el número máximo de direcciones objetivo, solo funciona en una dirección objetivo
arg-view-help = Modo de vista graph/table/point/sparkline
arg-output-help = Archivo de salida para guardar resultados de ping

## Medallas de Rango
rank-first = 🥇
rank-second = 🥈
rank-third = 🥉
rank-top-10 = 🏆
rank-slow = 🐢

## General
app-about = 🏎  PingWatch - Una Herramienta de Ping en Rust con Datos en Tiempo Real y Visualizaciones
app-version = v0.6.0
app-author = hanshuaikang<https://github.com/hanshuaikang>
