# On its own line
#With or without spaces
all: foo # On a rule definition line
	echo "Pass" # On a command line
# Between rules
foo: # On a rule definition without deps
	# On a rule without commands.
# At the end of the file.