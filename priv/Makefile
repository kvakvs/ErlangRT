compile: \
    lists.S lists.beam \
    test.S test.beam \
    erlang.S erlang.beam

%.S: %.erl
	erlc -S $^

%.beam: %.erl
	erlc $^
