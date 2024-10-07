PHONY: .download_texts, test_simple

download_texts:
	mkdir files
	curl https://www.gutenberg.org/cache/epub/98/pg98.txt -o files/a-tale-of-twi-cities.txt

test_simple:
	curl --header "Content-Type: application/json" \
  	--request POST \
  	--data '{"title":"A test", "word": "atrabilious", "word_number":1}' \
  	http://localhost:3000/