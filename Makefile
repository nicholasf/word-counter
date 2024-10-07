PHONY: .download_texts, test_simple

download_texts:
	rm -rf files
	mkdir files
	curl https://www.gutenberg.org/cache/epub/98/pg98.txt -o files/a-tale-of-two-cities.txt
	curl https://www.gutenberg.org/cache/epub/2814/pg2814.txt -o files/dubliners.txt

test_simple:
	curl --header "Content-Type: application/json" \
  	--request POST \
  	--data '{"title":"A test", "word": "atrabilious", "word_number":1}' \
  	http://localhost:3000/
	