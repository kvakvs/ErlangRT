-module(test2).
-export([test/0]).

test() ->
    %% Test 1: reverse a list
    [5, 4, 3, 2, a] = rev([a, 2, 3, 4, 5]),
    %% Test 2: simple recursion
    0 = recurse(10),
    %% Test 3: Reverse and equality check
    false = test_eq(),
    test_list_ops(),
    test_calls(),
    test_case(),
    test_hof(),
    test_hof_fold(),
    test_hof_nested(),
    test_lc(),
    test_send_receive(),
    test_ring(),
    test_try_catch(),
    test_try_of_catch(),
    test_mochijson(),
    test_apply(lists, erlang).

%%-----------------------------------------------
test_apply(X, Y) ->
    apply(X, any, [test1, test2]),
    Y:is_boolean(derp).

test_try_catch() ->
    try erlang:error(hello)
    catch error:E -> E = hello
    end.

test_try_of_catch() ->
    try self() of
        X when is_pid(X) -> erlang:error(hello);
        Y -> not_ok
    catch error:E -> E = hello
    end.

test_send_receive() ->
    self() ! test,
    receive
        test -> ok;
        X -> {fail, X}
    end.

test_lc() ->
    NumNodes = 5,
    [ID || ID <- lists:seq(1, NumNodes)].

test_ring() ->
    ring:create(5).

test_mochijson() ->
    mochijson:encode({struct, [{test_neg, -10000}, {test, 10000}]}).
% we need to go deeper to debuf
%mochijson:json_encode_proplist([{hello, "world"}], {encoder, unicode, null}).

test_hof_nested() ->
    C = fun(X, _) -> X end,
    %[{b} | Z] = lists:foldl(C, [], [a,b]),
    Z = lists:foldl(C, [], [a]),
    lists:reverse(Z).

test_hof() ->
    F = fun(A, B) -> A =< B end,
    [10, 20, 30, 40] = lists:sort(F, [30, 20, 40, 10]),
    [g1, g2, g3, g4] = lists:sort(F, [g3, g2, g4, g1]).

test_hof_fold() ->
    % test fold
    M = 2,
    G = fun(X, A) -> (X + A) * M end,
    15 = my_foldl(G, 0, [1, 2, 3, 4, 5]). % sum fold

my_foldl(F, Accu, [Hd | Tail]) ->
    my_foldl(F, F(Hd, Accu), Tail);
my_foldl(F, Accu, []) when is_function(F, 2) -> Accu.

test_case() ->
    [f1, f2, f3, f4] = lists:sort([f3, f2, f4, f1]).

test_calls() ->
    F1 = fun lists:reverse/1,
    [e1a, e2a, e3a] = F1([e3a, e2a, e1a]),

    L2 = [e1b, e2b, e3b],
    F2 = fun() -> lists:reverse(L2) end,
    L2 = F2([e3b, e2b, e1b]).

test_eq() ->
    [d1, d2, d3, d4] == rev([d1, d2, d3, d4]).

test_list_ops() ->
    b1aa = my_last2([b1aa]),
    b2ab = my_last2([b1ab, b2ab]),
    b3ac = my_last2([b1ac, b2ac, b3ac]),
%%    b5a = my_last2([b1a, b2a, b3a, b4a, b5a]),

    b4b = my_but_last2([b1b, b2b, b3b, b4b, b5b]),
    b2c = element_at(2, [b1c, b2c, b3c, b4c, b5c]),
    5 = len([b1, b2, b3, b4, b5]),
    [b5d, b4d, b3d, b2d, b1d] = rev([b1d, b2d, b3d, b4d, b5d]),
    false = is_palindrome([]),
    false = is_palindrome([b1e, b2e, b3e, b4e, b5e]),
    true = is_palindrome([c1, c2, c3, c2, c1]).

%f_test() ->
%    F = fun(X) -> X * 2 end,
%    F(2).

recurse(X) when X > 0 -> recurse(X - 1);
recurse(X) -> X.

%% From 99 problems: P01 Find the last box of a list.
%% Variant without using list reverse
my_last2([]) -> false;
my_last2([H | []]) -> H;
my_last2([_H | T]) when length(T) == 1 ->
    [H1 | []] = T,
    H1;
my_last2([_H | T]) ->
    my_last2(T).

%% From 99 problems: P02 Find the last but one box of a list.
%% Variant without using list reverse 
my_but_last2([]) -> false;
my_but_last2([_H | []]) -> false;
my_but_last2([H | T]) when length(T) == 1 -> H;
my_but_last2([_H | T]) -> my_but_last2(T).

%% From 99 problems: P03 Find the K'th element of a list.
%% Find the K'th element of a list (1-based)
element_at(K, L) when length(L) < K -> false;
element_at(K, L) -> element_at(K, L, 1).
element_at(K, [H | _T], C) when C == K -> H;
element_at(K, [_H | T], C) -> element_at(K, T, C + 1).

%% From 99 problems: P04 Find the number of elements of a list.
len([]) -> 0;
len(L) -> len(L, 0).

len([], Count) -> Count;
len([_H | T], Count) -> len(T, Count + 1).

%% From 99 problems: P05 Reverse a list.
rev([]) -> [];
rev(L) -> rev(L, []).
rev([], R) -> R;
rev([H | T], R) -> rev(T, [H | R]).

%% P06 Find out whether a list is a palindrome.
is_palindrome([]) -> false;
is_palindrome(L) when length(L) == 1 -> false;
is_palindrome(L) -> case L == rev(L) of true -> true; false -> false end.
