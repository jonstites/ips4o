all: release

debug:
	cargo build

build:
	cargo build --release

run:
	../target/release/benchmarks > stats.txt

_sqlplot:
	cp speed_plot_template speed.plot
	awk -f add_plots.awk >> speed.plot
	sqlplot-tools speed.plot

_gnuplot:
	gnuplot speed.plot

_fixup:
# set average line style
	sed -i 's/title "algo=ips4o::par_sort_unstable"/title "ips4o::sort_unstable" noenhanced/' speed.plot
	sed -i 's/title "algo=rayon::par_sort"/title "rayon::par_sort" noenhanced/' speed.plot
	sed -i 's/title "algo=rayon::par_sort_unstable"/title "rayon::par_sort_unstable" noenhanced/' speed.plot

	sed -i 's/title "algo=ips4o::sort_unstable"/title "ips4o::sort_unstable" noenhanced/' speed.plot
	sed -i 's/title "algo=std::sort"/title "std::sort" noenhanced/' speed.plot
	sed -i 's/title "algo=std::sort_unstable"/title "std::sort_unstable" noenhanced/' speed.plot
plot: _sqlplot _fixup _gnuplot
