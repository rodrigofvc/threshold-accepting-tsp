# png
set terminal pngcairo size 950,962 enhanced font 'Verdana,20'
set output 'data/data.png'

set border linewidth 1.5

set style line 1 \
    linecolor rgb '#0060ad' \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 1.5

unset key

set yrange [10900:12900]
set xrange [41500:43500]

plot 'data/data.dat' with linespoints linestyle 1
