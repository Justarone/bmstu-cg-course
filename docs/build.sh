#!/bin/bash

soffice --headless --convert-to pdf word_src/TZ_Perestoronin.doc
mv TZ_Perestoronin.pdf pdfs

soffice --headless --convert-to pdf word_src/RPZ_title.doc

cd tex_src/

pdflatex report.tex 
bibtex report 
pdflatex report.tex 
pdflatex report.tex

cd ..
mv tex_src/report.pdf .

pdfunite RPZ_title.pdf report.pdf RPZ.pdf
mv RPZ.pdf pdfs

rm RPZ_title.pdf report.pdf tex_src/*.aux tex_src/*.log tex_src/*.out tex_src/*.toc tex_src/*.blg tex_src/*.bbl
