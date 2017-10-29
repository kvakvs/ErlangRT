-module(test2).
-export([test/0
        ]).

test() ->
    [
    [5,4,3,2,1] = rev([1,2,3,4,5]),
    0 = recurse(10),
    false = test_eq(),
    test_list_ops(),
    test_extcalls(),
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
    test_apply(lists, erlang),
    done].

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
    mochijson:encode({struct, [ {test_neg, -10000}, {test, 10000} ]}).
    % we need to go deeper to debuf
    %mochijson:json_encode_proplist([{hello, "world"}], {encoder, unicode, null}).

test_hof_nested() ->
    C = fun(X, _) -> X end,
    %[{b} | Z] = lists:foldl(C, [], [a,b]),
    Z = lists:foldl(C, [], [a]),
    lists:reverse(Z).

test_hof() ->
    F = fun(A,B) -> A =< B end,
    [1,2,3,4] = lists:sort(F, [3,2,4,1]).

test_hof_fold() ->
    % test fold
    M = 2,
    G = fun(X, A) -> (X + A) * M end,
    15 = my_foldl(G, 0, [1,2,3,4,5]). % sum fold

my_foldl(F, Accu, [Hd|Tail]) ->
    my_foldl(F, F(Hd, Accu), Tail);
my_foldl(F, Accu, []) when is_function(F, 2) -> Accu.

test_case() ->
    [1,2,3,4] = lists:sort([3,2,4,1]).

test_extcalls() ->
    [1,2,3] = lists:reverse([3,2,1]).

test_eq() ->
    [1,2,3,4] == rev([1,2,3,4]).
    
test_list_ops() -> 
    X = [1,2,3,4,5],
    5 = my_last2(X),
    4 = my_but_last2(X),
    2 = element_at(2, X),
    5 = len(X),
    [5,4,3,2,1] = rev(X),
    false = is_palindrome([]),
    false = is_palindrome(X),
    true = is_palindrome([1,2,3,2,1]).

%f_test() ->
%    F = fun(X) -> X * 2 end,
%    F(2).

recurse(X) when X > 0 -> recurse(X-1);
recurse(X) -> X.

%% From 99 problems: P01 Find the last box of a list.
%% Variant without using list reverse
my_last2([]) -> false;
my_last2([H|[]]) -> H;
my_last2([_H|T]) when length(T) == 1 ->
  [H1|[]] = T,
  H1;
my_last2([_H|T]) ->
  my_last2(T).

%% From 99 problems: P02 Find the last but one box of a list.
%% Variant without using list reverse 
my_but_last2([])-> false;
my_but_last2([_H|[]]) -> false;
my_but_last2([H|T]) when length(T) == 1 -> H;
my_but_last2([_H|T]) -> my_but_last2(T).

%% From 99 problems: P03 Find the K'th element of a list.
%% Find the K'th element of a list (1-based)
element_at(K,L) when length(L) < K -> false;
element_at(K,L)-> element_at(K,L,1).
element_at(K,[H|_T],C) when C == K-> H;
element_at(K,[_H|T],C) -> element_at(K,T,C+1).

%% From 99 problems: P04 Find the number of elements of a list.
len([])-> 0;
len(L) -> len(L,0).

len([],Count) -> Count;
len([_H|T],Count)-> len(T,Count+1).

%% From 99 problems: P05 Reverse a list.
rev([])-> [];
rev(L) -> rev(L,[]).
rev([],R)-> R;
rev([H|T],R)-> rev(T,[H|R]).

%% P06 Find out whether a list is a palindrome.
is_palindrome([])-> false;
is_palindrome(L) when length(L) == 1 -> false;
is_palindrome(L) -> case L == rev(L) of true -> true; false -> false end.
