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


set style line 1 lt 1 lc 2 lw 1
set style line 2 lt 1 lc 3 lw 1
set style line 3 lt 1 lc 8 lw 1

#SQL DELETE FROM stats WHERE LOG(2, size) < 5
set title "uniform, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 0 title "par_sort" noenhanced with linespoints

set title "uniform, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size

