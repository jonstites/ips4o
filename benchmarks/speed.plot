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

set title "sort90, i32" 
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

set title "uniform, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 72 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 73 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 74 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 75 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 76 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 77 title "std::sort_unstable" noenhanced with linespoints

set title "sort90, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 78 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 79 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 80 title "std::sort_unstable" noenhanced with linespoints

set title "sort99, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 81 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 82 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 83 title "std::sort_unstable" noenhanced with linespoints

set title "asc, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 84 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 85 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 86 title "std::sort_unstable" noenhanced with linespoints

set title "desc, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 87 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 88 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 89 title "std::sort_unstable" noenhanced with linespoints

set title "dupsq, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 90 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 91 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 92 title "std::sort_unstable" noenhanced with linespoints

set title "dup8, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 93 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 94 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 95 title "std::sort_unstable" noenhanced with linespoints

set title "mod8, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 96 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 97 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 98 title "std::sort_unstable" noenhanced with linespoints

set title "ones, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 99 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 100 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 101 title "std::sort_unstable" noenhanced with linespoints

set title "organ, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 102 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 103 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 104 title "std::sort_unstable" noenhanced with linespoints

set title "merge, i32" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "false" AND type = "i32" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 105 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 106 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 107 title "std::sort_unstable" noenhanced with linespoints

set title "uniform, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "uniform" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 108 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 109 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 110 title "std::sort_unstable" noenhanced with linespoints

set title "sort50, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort50" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 111 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 112 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 113 title "std::sort_unstable" noenhanced with linespoints

set title "sort90, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort90" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 114 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 115 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 116 title "std::sort_unstable" noenhanced with linespoints

set title "sort99, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "sort99" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 117 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 118 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 119 title "std::sort_unstable" noenhanced with linespoints

set title "asc, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "asc" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 120 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 121 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 122 title "std::sort_unstable" noenhanced with linespoints

set title "desc, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "desc" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 123 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 124 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 125 title "std::sort_unstable" noenhanced with linespoints

set title "dupsq, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dupsq" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 126 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 127 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 128 title "std::sort_unstable" noenhanced with linespoints

set title "dup8, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "dup8" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 129 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 130 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 131 title "std::sort_unstable" noenhanced with linespoints

set title "mod8, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "mod8" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 132 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 133 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 134 title "std::sort_unstable" noenhanced with linespoints

set title "ones, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "ones" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 135 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 136 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 137 title "std::sort_unstable" noenhanced with linespoints

set title "organ, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "organ" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 138 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 139 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 140 title "std::sort_unstable" noenhanced with linespoints

set title "merge, i64" 
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, 
## time / (size * log(2, size)) AS y, 
## MULTIPLOT 
## FROM stats 
## WHERE dist = "merge" AND parallelism = "false" AND type = "i64" 
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size
plot \
    'speed-data.txt' index 141 title "ips4o::sort_unstable" noenhanced with linespoints, \
    'speed-data.txt' index 142 title "std::sort" noenhanced with linespoints, \
    'speed-data.txt' index 143 title "std::sort_unstable" noenhanced with linespoints

