
all: c
	echo "Fourth"

c: b a
	echo "Third"

b: a
	echo "Second"

a:
	echo "First"