set terminal pngcairo size 1750,962 enhanced font 'Verdana,20'
set output 'data.png'

set border linewidth 1.5

unset key
unset tics

set border lw 1.5
set style line 1 lc rgb 'gray' lt 1 lw 1
set style line 2 lc rgb 'black' lt 1 lw 1
set style line 3 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 1.5

set xrange [-200:200]
set yrange [-80:80]

plot 'world-50.txt' with filledcurves ls 1, \
    '' with l ls 2, \
     'data.dat' with linespoints linestyle 3
