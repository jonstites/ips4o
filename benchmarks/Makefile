all: release

debug:
	cargo build

release:
	cargo build --release

run:
	../target/release/benchmarks > stats.txt

_sqlplot:
	sqlplot-tools speed.plot

_gnuplot:
	gnuplot speed.plot

_fixup:
# fixup standard deviation lines
	sed -i 's/title "algo=par_sort,a=\(-\)*1"\( notitle ls 3\)* with lines\(points\)*/notitle ls 3 with lines/' speed.plot
	sed -i 's/title "algo=par_sort_unstable,a=\(-\)*1"\( notitle ls 5\)* with lines\(points\)*/notitle ls 5 with lines/' speed.plot
# set average line style
	sed -i 's/title "algo=par_sort,a=0"\( ls 4\)*/title "par_sort" noenhanced ls 4/' speed.plot
	sed -i 's/title "algo=par_sort_unstable,a=0"\( ls 6\)*/title "par_sort_unstable" noenhanced ls 6/' speed.plot

plot: _sqlplot _fixup _gnuplot
