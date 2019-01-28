-module(test).
-export([start/0]).

start() ->
    [4, 3, 2, 1] = test1([1, 2, 3, 4]).

test1(X) -> lists:reverse(X).
