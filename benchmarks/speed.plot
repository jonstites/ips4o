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

set yrange [0:5]

set xlabel 'Item Count [log_2(n)]'
set ylabel 'Run Time / n log_2n / [Nanoseconds]'


set style line 1 lt 1 lc 2 lw 1
set style line 2 lt 1 lc 3 lw 1
set style line 3 lt 1 lc 8 lw 1


#SQL DELETE FROM stats WHERE LOG(2, size) < 2
set title "uniform, i32" 
set yrange [0:5] 
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
set yrange [0:5] 
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

set title "sort90, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 6 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 7 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 8 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort99, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 9 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 10 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 11 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "asc, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 12 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 13 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 14 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "desc, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 15 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 16 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 17 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dupsq, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 18 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 19 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 20 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dup8, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 21 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 22 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 23 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "mod8, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 24 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 25 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 26 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "ones, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 27 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 28 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 29 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "organ, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 30 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 31 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 32 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "merge, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "true" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 33 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 34 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 35 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "uniform, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 36 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 37 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 38 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort50, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 39 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 40 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 41 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort90, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 42 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 43 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 44 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort99, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 45 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 46 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 47 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "asc, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 48 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 49 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 50 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "desc, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 51 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 52 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 53 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dupsq, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 54 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 55 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 56 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dup8, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 57 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 58 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 59 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "mod8, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 60 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 61 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 62 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "ones, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 63 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 64 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 65 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "organ, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 66 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 67 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 68 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "merge, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "true" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 69 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 70 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 71 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "uniform, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 72 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 73 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 74 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort50, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 75 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 76 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 77 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort90, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 78 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 79 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 80 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort99, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 81 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 82 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 83 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "asc, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 84 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 85 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 86 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "desc, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 87 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 88 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 89 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dupsq, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 90 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 91 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 92 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dup8, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 93 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 94 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 95 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "mod8, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 96 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 97 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 98 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "ones, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 99 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 100 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 101 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "organ, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 102 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 103 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 104 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "merge, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "true" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 105 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 106 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 107 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "uniform, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 108 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 109 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 110 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort50, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 111 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 112 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 113 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort90, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 114 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 115 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 116 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "sort99, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 117 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 118 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 119 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "asc, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 120 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 121 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 122 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "desc, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 123 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 124 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 125 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dupsq, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 126 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 127 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 128 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "dup8, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 129 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 130 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 131 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "mod8, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 132 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 133 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 134 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "ones, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 135 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 136 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 137 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "organ, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 138 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 139 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 140 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "merge, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "true" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 141 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 142 title "rayon::par_sort" noenhanced with linespoints, \
    'speed-data.txt' index 143 title "rayon::par_sort_unstable" noenhanced with linespoints

set title "uniform, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 144 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 145 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 146 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 147 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 148 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 149 title "std::sort_unstable" noenhanced with linespoints

set title "sort90, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 150 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 151 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 152 title "std::sort_unstable" noenhanced with linespoints

set title "sort99, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 153 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 154 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 155 title "std::sort_unstable" noenhanced with linespoints

set title "asc, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 156 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 157 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 158 title "std::sort_unstable" noenhanced with linespoints

set title "desc, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 159 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 160 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 161 title "std::sort_unstable" noenhanced with linespoints

set title "dupsq, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 162 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 163 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 164 title "std::sort_unstable" noenhanced with linespoints

set title "dup8, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 165 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 166 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 167 title "std::sort_unstable" noenhanced with linespoints

set title "mod8, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 168 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 169 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 170 title "std::sort_unstable" noenhanced with linespoints

set title "ones, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 171 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 172 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 173 title "std::sort_unstable" noenhanced with linespoints

set title "organ, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 174 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 175 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 176 title "std::sort_unstable" noenhanced with linespoints

set title "merge, i32" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 177 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 178 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 179 title "std::sort_unstable" noenhanced with linespoints

set title "uniform, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 180 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 181 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 182 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 183 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 184 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 185 title "std::sort_unstable" noenhanced with linespoints

set title "sort90, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 186 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 187 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 188 title "std::sort_unstable" noenhanced with linespoints

set title "sort99, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 189 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 190 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 191 title "std::sort_unstable" noenhanced with linespoints

set title "asc, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 192 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 193 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 194 title "std::sort_unstable" noenhanced with linespoints

set title "desc, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 195 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 196 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 197 title "std::sort_unstable" noenhanced with linespoints

set title "dupsq, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 198 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 199 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 200 title "std::sort_unstable" noenhanced with linespoints

set title "dup8, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 201 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 202 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 203 title "std::sort_unstable" noenhanced with linespoints

set title "mod8, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 204 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 205 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 206 title "std::sort_unstable" noenhanced with linespoints

set title "ones, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 207 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 208 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 209 title "std::sort_unstable" noenhanced with linespoints

set title "organ, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 210 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 211 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 212 title "std::sort_unstable" noenhanced with linespoints

set title "merge, i64" 
set yrange [0:5] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 213 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 214 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 215 title "std::sort_unstable" noenhanced with linespoints

set title "uniform, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 216 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 217 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 218 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 219 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 220 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 221 title "std::sort_unstable" noenhanced with linespoints

set title "sort90, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 222 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 223 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 224 title "std::sort_unstable" noenhanced with linespoints

set title "sort99, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 225 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 226 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 227 title "std::sort_unstable" noenhanced with linespoints

set title "asc, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 228 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 229 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 230 title "std::sort_unstable" noenhanced with linespoints

set title "desc, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 231 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 232 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 233 title "std::sort_unstable" noenhanced with linespoints

set title "dupsq, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 234 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 235 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 236 title "std::sort_unstable" noenhanced with linespoints

set title "dup8, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 237 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 238 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 239 title "std::sort_unstable" noenhanced with linespoints

set title "mod8, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 240 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 241 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 242 title "std::sort_unstable" noenhanced with linespoints

set title "ones, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 243 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 244 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 245 title "std::sort_unstable" noenhanced with linespoints

set title "organ, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 246 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 247 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 248 title "std::sort_unstable" noenhanced with linespoints

set title "merge, String" 
set yrange [0:20] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "false" AND type = "String" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 249 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 250 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 251 title "std::sort_unstable" noenhanced with linespoints

set title "uniform, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 252 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 253 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 254 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 255 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 256 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 257 title "std::sort_unstable" noenhanced with linespoints

set title "sort90, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 258 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 259 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 260 title "std::sort_unstable" noenhanced with linespoints

set title "sort99, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 261 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 262 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 263 title "std::sort_unstable" noenhanced with linespoints

set title "asc, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 264 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 265 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 266 title "std::sort_unstable" noenhanced with linespoints

set title "desc, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 267 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 268 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 269 title "std::sort_unstable" noenhanced with linespoints

set title "dupsq, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 270 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 271 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 272 title "std::sort_unstable" noenhanced with linespoints

set title "dup8, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 273 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 274 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 275 title "std::sort_unstable" noenhanced with linespoints

set title "mod8, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 276 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 277 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 278 title "std::sort_unstable" noenhanced with linespoints

set title "ones, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 279 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 280 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 281 title "std::sort_unstable" noenhanced with linespoints

set title "organ, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 282 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 283 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 284 title "std::sort_unstable" noenhanced with linespoints

set title "merge, BigString" 
set yrange [0:50] 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "false" AND type = "BigString" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 285 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 286 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 287 title "std::sort_unstable" noenhanced with linespoints

