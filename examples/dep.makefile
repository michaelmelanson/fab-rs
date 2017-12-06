
all: a
	echo "Fourth"

a: b c
	echo "Third"

b:
	echo "First"

c: b
	echo "Second"