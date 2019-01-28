compile: \
    lists.S lists.beam \
    test.S test.beam \
    test2.S test2.beam \
    ring.S ring.beam \
    mochijson.S mochijson.beam \
    erlang.S erlang.beam

%.S: %.erl
	erlc -S $^

%.beam: %.erl
	erlc $^
