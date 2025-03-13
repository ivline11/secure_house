# Makefile

all: secure_house

secure_house: secure_house.rs
	rustc secure_house.rs -o secure_house

clean:
	rm -f secure_house
