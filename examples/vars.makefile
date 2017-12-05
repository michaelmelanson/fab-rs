
all: foo

foo: bar
	echo "Target is $@ (should be 'foo')"
	echo "Dependency is $< (should be 'bar')"

bar:
