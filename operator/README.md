# Aos Operator

### Run operator

```shell
cargo build --release

./target/release/operator-runer -i postgres://postgres:hetu@0.0.0.0:5432/operator_db

./target/release/operator-runer  -c .config-operator.yaml
```