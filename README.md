# Detector de Idioma Inglês

Este projeto Rust simples determina se um texto fornecido está em inglês, comparando suas palavras com uma lista de palavras em inglês.

## Funcionalidades

*   Lê palavras de um arquivo `english_words.txt`.
*   Conta quantas palavras em um texto fornecido existem na lista de palavras em inglês.
*   Calcula a porcentagem de palavras em inglês no texto.
*   Determina se o texto é considerado inglês com base em um limite de porcentagem (70%).

## Como funciona

O programa lê um arquivo chamado `english_words.txt`, que contém uma lista de palavras em inglês, uma por linha. Ele então recebe um texto como entrada e o divide em palavras. Cada palavra do texto é comparada com a lista de palavras em inglês. Se uma palavra for encontrada na lista, um contador é incrementado. Finalmente, a porcentagem de palavras em inglês no texto é calculada. Se essa porcentagem for maior ou igual a 70%, o programa considera o texto como inglês.

## Código (Pseudo-código)

```
função contar_palavras(texto, palavras_inglesas):
  inicializa contador de palavras correspondentes
  para cada palavra em texto (em maiúsculas):
    se palavra existe em palavras_inglesas:
      incrementa contador
  retorna contador

função texto_eh_ingles(texto, palavras_inglesas):
  conta palavras correspondentes
  calcula porcentagem de palavras correspondentes
  se porcentagem >= 70%:
    retorna verdadeiro
  senão:
    retorna falso
```

## Execução

`cargo run`
