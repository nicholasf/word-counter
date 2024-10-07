PHONY .download_texts

download_texts:
	mkdir files
	curl https://www.gutenberg.org/cache/epub/98/pg98.txt -o files/a-tale-of-twi-cities.txt

