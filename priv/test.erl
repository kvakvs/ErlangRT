-module(test).
-export([start/0]).

start() ->
    test1([a, 123]).

test1(X) -> lists:reverse(X).
