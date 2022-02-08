#!/bin/sh
cargo run --release --example sweep 2000 >sweep.dat
cargo run --release --example transfer 2000 >transfer.dat
gnuplot <<'EOF'
plot "sweep.dat" with lines, "transfer.dat" with lines;
pause mouse close;
EOF
