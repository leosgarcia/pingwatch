## Rótulos da UI
label-rank = Rank
label-target = Alvo
label-ip = IP
label-last-rtt = Últ. RTT
label-avg-rtt = Média RTT
label-max = Máx
label-min = Mín
label-jitter = Jitter
label-loss = Perda

## Visualização Tabela
table-view-title = Visualização em Tabela do PingWatch

## Visualização Ponto
point-view-title = Visualização em Ponto do PingWatch
point-view-legend = Saudável
point-view-high-latency = Latência Alta (acima de 80% do máximo)
point-view-timeout = Timeout

## Visualização Sparkline
sparkline-view-title = Visualização SparkLine do PingWatch
sparkline-view-note = Área em branco significa timeout ou erro

## Visualização Gráfico
graph-view-title = Visualização em Gráfico do PingWatch

## Mensagens de Erro
error-target-required = Erro: endereço IP alvo ou nome de host é obrigatório
error-output-exists = Arquivo de saída já existe: {$path}
error-unable-shutdown = Impossível aguardar sinal de shutdown: {$error}
error-ping-init-failed = host({$host}) erro de ping, motivo: falha na inicialização do ping, erro: {$error}
error-ping-unknown = host({$host}) erro de ping, motivo: desconhecido, erro: {$error}
error-ping-recv-failed = host({$host}) erro de ping, motivo: falha na recepção, erro: {$error}
error-encode-metrics = Erro ao codificar métricas: {$error}
error-serve-connection = Erro ao servir conexão: {$error}
error-accept-connection = Falha ao aceitar conexão: {$error}
error-restore-terminal = Falha ao restaurar terminal: {$error}

## Métricas e Unidades
unit-ms = ms
unit-percent = %
metric-less-than = < 0,01ms
metric-zero = 0,0ms

## Modo Exporter
exporter-mode-title = Modo Exporter do PingWatch
exporter-interval-help = Intervalo em segundos entre pings
exporter-port-help = Porta HTTP de métricas Prometheus

## Ajuda da Linha de Comando
arg-target-help = endereço IP alvo ou nome de host para fazer ping
arg-count-help = Número de pings a enviar
arg-interval-help = Intervalo em segundos entre pings
arg-ipv6-help = Forçar uso de IPv6
arg-multiple-help = Especificar o número máximo de endereços alvo, funciona apenas em um endereço alvo
arg-view-help = Modo de visualização graph/table/point/sparkline
arg-output-help = Arquivo de saída para salvar resultados de ping

## Medalhas de Rank
rank-first = 🥇
rank-second = 🥈
rank-third = 🥉
rank-top-10 = 🏆
rank-slow = 🐢

## Geral
app-about = 🏎  PingWatch - Uma Ferramenta de Ping em Rust com Dados em Tempo Real e Visualizações
app-version = v0.6.0
app-author = hanshuaikang<https://github.com/hanshuaikang>
