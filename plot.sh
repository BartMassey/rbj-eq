#!/bin/sh
cargo run --release --example sweep 0.2 >sweep.dat
cargo run --release --example transfer 0.2 >transfer.dat
gnuplot <<'EOF'
plot "sweep.dat" with lines, "transfer.dat" with lines;
pause mouse close;
EOF
