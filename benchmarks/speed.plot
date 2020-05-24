# IMPORT-DATA stats stats.txt

set terminal pdf size 13.33cm,10cm linewidth 2.0
set output "speed.pdf"

set style line 11 lc rgb "#333333" lt 1
set border 3 back ls 11
set tics nomirror

# define grid
set style line 12 lc rgb "#333333" lt 0 lw 1
set grid back ls 12

set grid xtics ytics

set key top left

set yrange [0:4]

set xlabel 'Item Count [log_2(n)]'
set ylabel 'Run Time / n log_2n [Nanoseconds]'

set style line 1 lt 1 lw .2
set style line 2 lt 1 lw 1

set style line 3 lt 2 lc 2 lw .2
set style line 4 lt 1 lc 2 lw 1

set style line 5 lt 2 lc 3 lw .2
set style line 6 lt 1 lc 3 lw 1

set style line 7 lt 2 lc 8 lw .2
set style line 8 lt 1 lc 8 lw 1

#SQL DELETE FROM stats WHERE LOG(2, size) < 10

# most cross-DB way of doing what "SELECT ..., a FROM ..., (VALUES(-1), (0),
# (1)) as dev (a)" does in postgres
# SQL CREATE TABLE IF NOT EXISTS temp_add (a INTEGER)
# SQL DELETE FROM temp_add
# SQL INSERT INTO temp_add VALUES (-1), (0), (1)

set title 'Random'
## MULTIPLOT(algo, a) SELECT LOG(2, size) AS x,
## AVG(time + a * stddev) / (size * log(2, size)) AS y,
## MULTIPLOT
## FROM stats, temp_add WHERE dist = 'uniform' AND parallelism = 'true'
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 0 title "algo=rayon::par_sort,a=-1" with linespoints, \
    'speed-data.txt' index 1 title "algo=rayon::par_sort,a=0" with linespoints, \
    'speed-data.txt' index 2 title "algo=rayon::par_sort,a=1" with linespoints

# SQL DROP TABLE temp_add
