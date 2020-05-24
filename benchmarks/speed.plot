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
    'speed-data.txt' index 0 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 1 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 2 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort50, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 3 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 4 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 5 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "uniform, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 6 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 7 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 8 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort50, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 9 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 10 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 11 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "uniform, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 12 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 13 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 14 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 15 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 16 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 17 title "std::sort_unstable" noenhanced with linespoints

set title "uniform, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 18 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 19 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 20 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 21 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 22 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 23 title "std::sort_unstable" noenhanced with linespoints

