# Gradientes da ferramenta de aparência

`validate` e `resources` são exportadas pelo módulo PyO3 `gradient_rs`.
A classe GTK `Controls` continua em `apps/emwm-appearance/gradient.py`, junto
com as funções Python de referência usadas pelo rust-anything.

Para compilar e testar a partir da raiz do repositório:

```sh
sh scripts/build-gradient.sh build/gradient-target build/appearance
PYTHONPATH="$PWD/build/appearance" RUST_ANYTHING_MODE=new python3 -m unittest discover -s apps/emwm-appearance/tests
```

O pacote Debian e o instalador independente usam o mesmo script. O módulo
nativo é o padrão quando instalado. Para executar a referência Python em
uma comparação, use `RUST_ANYTHING_UNIT_GRADIENT=original`.

Para repetir a comparação com a implementação Python:

```sh
rust-anything record rust/gradient-rs --count 24 --inputs rust/gradient-rs/inputs.jsonl --from-tests -- python3 -m unittest discover -s tests -p test_gradient.py
rust-anything gate rust/gradient-rs
rust-anything bench rust/gradient-rs
```
