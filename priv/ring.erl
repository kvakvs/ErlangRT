%%% Token Ring
%%% The following example has been taken from
%%% http://trigonakis.com/blog/2011/05/26/introduction-to-erlang-message-passing/
%%%
%%% This application creates NumNodes processes and arranges them in a ring
%%% (every process has one next process).
%%% Then the coordinator “inserts” a token in the first node of the ring.
%%% Every node (process) receiving the token increases its value by 1 and
%%% sends it to the next node. The application stops when the token has value
%%% greater than the MAXVAL.
-module(ring).
-export([create/1, node/2, connect/1]).

-define(MAXVAL, 10).

%% creates the ring's nodes, connects them in a ring, sends the token in the
%% ring, and collects the exit messages from the nodes
create(NumNodes) when is_integer(NumNodes), NumNodes > 1 ->
  Nodes = [spawn(?MODULE, node, [ID, self()]) || ID <- lists:seq(1, NumNodes)],

  %% notice that the above expression denotes a list using
  %% the mechanism of list comprehension, similar to Haskell list comprehension
  %% Remember that the function spawn returns a process identifier,
  %% so to the Nodes variable we associate the list of the created process nodes.
  ring:connect(Nodes),
  hd(Nodes) ! {token, 0},
  getexits(Nodes).

%% collects the exit messages from the nodes
getexits([]) ->
  %io:format("[Coord] Done.~n"),
  ok;
getexits(Nodes) ->
  receive
    {Node, exit} ->
      case lists:member(Node, Nodes) of
	true ->
	  getexits(lists:delete(Node, Nodes));
	_ ->
	  getexits(Nodes)
      end
  end.

%% little trick in order to connect the last with the first node
%% handle the [nd0, nd1, ..., ndN] list as [nd0, nd1, ..., ndN, nd0]
%%
%% Notice the use of a particular sort of pattern matching, enabling
%% to associate the whole input to the variable N and the head of the
%% input list to the variable H.
%% This particular sort of pattern matching is present also in Haskell (as-pattern)
%% and PICT (layered pattern).
%% In Haskell and PICT the following notation is used:  x@p (where x is a variable and p a pattern).
connect(N = [H | _]) ->
  connect_(N ++ [H]).

%% connects the nodes to form a ring
connect_([]) ->
  connected;
connect_([_]) ->
  connected;
connect_([N1, N2 | Nodes]) ->
  N1 ! {self(), connect, N2},
  connect_([N2 | Nodes]).

%% The computation in each process node consists in the evaluation of
%% the node function below.
%% The node function initially waits for the next node's pid and then proceed
%% by evaluating the other node function (that can be recognized as different from
%% the first one, since it has three arguments instead of two.
node(ID, CrdId) ->
  receive
    {CrdId, connect, NxtNdId} ->
      %io:format("[~p:~p] got my next ~p~n", [ID, self(), NxtNdId]),
      node(ID, CrdId, NxtNdId)
  end.

%% the main functionality of a node; receive the token, increase its value and
%% send it to the next node on the ring
node(ID, CrdId, NxtNdId) ->
  receive
    {token, Val} ->
      if
	 Val < ?MAXVAL ->
	  NxtNdId ! {token, Val + 1},
	  node(ID, CrdId, NxtNdId);
	true ->
	  %io:format("[~p:~p] token value ~p~n", [ID, self(), Val]),
	  case erlang:is_process_alive(NxtNdId) of
	    true ->
	      NxtNdId ! {token, Val + 1};
	    _ ->
	      ok
	  end,
	  CrdId ! {self(), exit},
	  done
      end
    end.
