# png
set terminal pngcairo size 4950,1962 enhanced font 'Verdana,20'
set output 'log.png'

set border linewidth 1.0

set style line 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 1.5

unset key

set yrange [0:4500000]
set xrange [0:120740]

plot 'log1.dat' with linespoints linestyle 1
