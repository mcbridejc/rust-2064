# Painful dtrace profiling process
```
cargo run --release -- -f &
sudo dtrace  -p <PIDofABOVE> -o profile.stacks -n 'profile-997 /pid == $target/ { @[ustack(100)] = count(); }'
~/install/FlameGraph/stackcollapse.pl profile.stacks | ~/install/FlameGraph/flamegraph.pl > flame.svg
```
