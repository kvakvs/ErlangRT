compile: \
    test.S test.beam \
    test2.S test2.beam \
    ring.S ring.beam \
    mochijson.S mochijson.beam

%.S: %.erl
	erlc -S $^

%.beam: %.erl
	erlc $^
