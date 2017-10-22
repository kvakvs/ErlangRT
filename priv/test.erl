-module(test).
-export([start/0]).

start() ->
    test1([1, 2, 3, 4]).

test1(X) -> lists:reverse(X).
