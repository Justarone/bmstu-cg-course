.PHONY: all clean

all: RPZ

TZ:
	soffice --headless --convert-to pdf word_src/TZ_Perestoronin.doc
	mv TZ_Perestoronin.pdf pdfs

RPZ:
	soffice --headless --convert-to pdf word_src/RPZ_title.doc
	cd tex_src/ && make -f Makefile_tex
	pdfunite RPZ_title.pdf tex_src/report.pdf RPZ.pdf
	rm RPZ_title.pdf
	mv RPZ.pdf pdfs

clean: 
	rm RPZ_title.pdf report.pdf tex_src/*.aux tex_src/*.log tex_src/*.out tex_src/*.toc tex_src/*.blg tex_src/*.bbl
