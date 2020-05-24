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

set style line 3 lt 2 lw .2
set style line 4 lt 2 lw 1

set style line 5 lt 3 lw .2
set style line 6 lt 3 lw 1


#SQL DELETE FROM stats WHERE LOG(2, size) < 12

# most cross-DB way of doing what "SELECT ..., a FROM ..., (VALUES(-1), (0),
# (1)) as dev (a)" does in postgres
# SQL CREATE TABLE IF NOT EXISTS temp_add (a INTEGER)
# SQL DELETE FROM temp_add
# SQL INSERT INTO temp_add VALUES (-1), (0), (1)

set title 'Random'
## MULTIPLOT(algo, a) SELECT LOG(2, size) AS x,
## AVG(time + a * stddev) / (size * log(2, size)) AS y,
## MULTIPLOT
## FROM stats, temp_add WHERE dist = 'random'
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 0 notitle ls 3 with lines, \
    'speed-data.txt' index 1 title "par_sort" noenhanced ls 4 with linespoints, \
    'speed-data.txt' index 2 notitle ls 3 with lines, \
    'speed-data.txt' index 3 notitle ls 5 with lines, \
    'speed-data.txt' index 4 title "par_sort_unstable" noenhanced ls 6 with linespoints, \
    'speed-data.txt' index 5 notitle ls 5 with lines

# SQL DROP TABLE temp_add
