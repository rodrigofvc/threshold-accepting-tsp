# png
set terminal pngcairo size 1950,1962 enhanced font 'Verdana,20'
set output 'log.png'

set border linewidth 1.0

set style line 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 1.5

unset key

set yrange [6000:19000]
set xrange [0:4300]

plot 'log.dat' with linespoints linestyle 1
