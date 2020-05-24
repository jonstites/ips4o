all: release

debug:
	cargo build

build:
	cargo build --release

run:
	../target/release/benchmarks > stats.txt

_sqlplot:
	cp speed_plot_template speed.plot
	sqlplot-tools speed.plot

_gnuplot:
	gnuplot speed.plot

_fixup:
# set average line style
	sed -i 's/title "algo=ips4o::par_sort_unstable"/title "ips4o" noenhanced/' speed.plot
	sed -i 's/title "algo=rayon::par_sort"/title "par_sort" noenhanced/' speed.plot
	sed -i 's/title "algo=rayon::par_sort_unstable"/title "par_sort_unstable" noenhanced/' speed.plot

plot: _sqlplot _fixup _gnuplot
