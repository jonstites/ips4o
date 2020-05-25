BEGIN { 
    p_n=split("true false", p); 
    t_n=split("i32 i64 String BigString", t);
    y_n=split("5 5 20 50", y_max);
    d_n=split("uniform sort50 sort90 sort99 asc desc dupsq dup8 mod8 ones organ merge", d);
    for (p_i=1;p_i<=p_n;++p_i) 
    for (t_i=1;t_i<=t_n;++t_i) 
    for (d_i=1;d_i<=d_n;++d_i) 
printf "set title \"%2$s, %1$s\" \n\
set yrange [0:%4$s] \n\
## MULTIPLOT(algo) SELECT LOG(2, size) AS x, \n\
## time / (size * log(2, size)) AS y, \n\
## MULTIPLOT \n\
## FROM stats \n\
## WHERE dist = \"%2$s\" AND parallelism = \"%3$s\" AND type = \"%1$s\" \n\
## GROUP BY MULTIPLOT, size ORDER BY MULTIPLOT, size\n\n", t[t_i], d[d_i], p[p_i], y_max[t_i]}
