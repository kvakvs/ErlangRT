{module, test}.  %% version = 0

{exports, [{module_info,0},{module_info,1},{start,0}]}.

{attributes, []}.

{labels, 9}.


{function, start, 0, 2}.
  {label,1}.
    {line,[{location,"test.erl",4}]}.
    {func_info,{atom,test},{atom,start},0}.
  {label,2}.
    {move,{literal,[a,123]},{x,0}}.
    {call_only,1,{f,4}}.


{function, test1, 1, 4}.
  {label,3}.
    {line,[{location,"test.erl",7}]}.
    {func_info,{atom,test},{atom,test1},1}.
  {label,4}.
    {line,[{location,"test.erl",7}]}.
    {call_ext_only,1,{extfunc,lists,reverse,1}}.


{function, module_info, 0, 6}.
  {label,5}.
    {line,[]}.
    {func_info,{atom,test},{atom,module_info},0}.
  {label,6}.
    {move,{atom,test},{x,0}}.
    {line,[]}.
    {call_ext_only,1,{extfunc,erlang,get_module_info,1}}.


{function, module_info, 1, 8}.
  {label,7}.
    {line,[]}.
    {func_info,{atom,test},{atom,module_info},1}.
  {label,8}.
    {move,{x,0},{x,1}}.
    {move,{atom,test},{x,0}}.
    {line,[]}.
    {call_ext_only,2,{extfunc,erlang,get_module_info,2}}.
