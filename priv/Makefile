compile: lists.S lists.beam test.S test.beam

%.S: %.erl
	erlc -S $^

%.beam: %.erl
	erlc $^
