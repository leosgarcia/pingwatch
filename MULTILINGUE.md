# Suporte Multilíngue - PingWatch

Este documento descreve como o PingWatch agora suporta múltiplos idiomas.

## Idiomas Suportados

- **English** (en)
- **Português Brasileiro** (pt-BR)  
- **Español** (es)

## Como Usar

### 1. Selecionar Idioma via Linha de Comando

Use a flag `--lang` para escolher o idioma:

```bash
# Usar português brasileiro
./pingwatch google.com --lang pt-BR

# Usar espanhol
./pingwatch google.com --lang es

# Usar inglês (padrão)
./pingwatch google.com --lang en
```

### 2. Variável de Ambiente

Você pode definir o idioma padrão usando a variável de ambiente `PINGWATCH_LANG`:

```bash
# Windows (PowerShell)
$env:PINGWATCH_LANG="pt-BR"
./pingwatch google.com

# Linux/macOS (Bash)
export PINGWATCH_LANG="pt-BR"
./pingwatch google.com
```

### 3. Auto-Detecção (Padrão)

Se nenhum idioma for especificado, o PingWatch tenta detectar automaticamente o idioma do sistema:

```bash
# Usar idioma do sistema automaticamente
./pingwatch google.com
```

## Prioridade de Idioma

O idioma é selecionado na seguinte ordem:

1. **Flag de linha de comando** (`--lang`) - maior prioridade
2. **Variável de ambiente** (`PINGWATCH_LANG`)
3. **Idioma do sistema** (auto-detectado)
4. **Inglês** (padrão) - se nenhuma das opções anteriores funcionar

## Exemplos Completos

### Português Brasileiro

```bash
./pingwatch 8.8.8.8 1.1.1.1 --lang pt-BR --count 5 --view table
```

Saída esperada: Interface em português com rótulos como "Alvo", "Perda", etc.

### Espanhol

```bash
./pingwatch 8.8.8.8 --lang es --interval 1 --view sparkline
```

Saída esperada: Interface em espanhol com rótulos como "Objetivo", "Pérdida", etc.

### Exporter Mode com Idioma

O Exporter mode também respeita o idioma selecionado para mensagens de erro:

```bash
./pingwatch exporter google.com --lang pt-BR --port 9090
```

## Estrutura de Arquivos de Tradução

As traduções estão organizadas em arquivos `.ftl` (Fluent):

```
locales/
├── en/
│   └── main.ftl          # Traduções em inglês
├── pt-BR/
│   └── main.ftl          # Traduções em português
└── es/
    └── main.ftl          # Traduções em espanhol
```

## Adicionando Novos Idiomas

Para adicionar suporte a um novo idioma:

1. Crie um diretório: `locales/{lang_code}/`
2. Copie o arquivo `locales/en/main.ftl` para o novo diretório
3. Traduza as strings em `main.ftl`
4. Atualize `src/i18n.rs` para incluir o novo idioma na lista

Exemplo para adicionar Francês (fr):

```bash
mkdir -p locales/fr
cp locales/en/main.ftl locales/fr/main.ftl
# Editar locales/fr/main.ftl com as traduções francesas
```

Depois atualize `src/i18n.rs`:

```rust
for lang in &["en", "pt-BR", "es", "fr"] {
    // ...
}
```

## Notas Técnicas

- Implementação usa a biblioteca `fluent` para gerenciamento de i18n
- As traduções são embarcadas no binário compilado usando `rust-embed`
- Sistema de fallback automático para inglês se uma tradução não for encontrada
- Detecção de idioma do sistema funciona em Windows, Linux e macOS

## Strings Traduzidas

Atualmente, as seguintes strings foram traduzidas:

- **Rótulos da UI**: Rank, Target, IP, RTT, Max, Min, Jitter, Loss
- **Vistas**: Table, Point, Sparkline, Graph
- **Medalhas de Rank**: 🥇 🥈 🥉 🏆 🐢
- **Mensagens de Erro**: Alvo requerido, arquivo existe, etc.
- **Unidades**: ms, %
- **Ajuda da linha de comando**: Descrições de argumentos

Para adicionar mais traduções, edite os arquivos `.ftl` em cada diretório de idioma.
