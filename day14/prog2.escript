#!/usr/bin/env escript

-record(reindeer, {name, speed, flytime, resttime, distance, lead_points}).

run(FileName, Time) ->
    Reindeer = readlines(FileName),
    FinalReindeer = advance(Reindeer, 0, Time),
    BestDistReindeer = best_distance(FinalReindeer),
    io:fwrite("Best distance: ~w (by ~p)\n", [BestDistReindeer#reindeer.distance, BestDistReindeer]),
    MostPointsReindeer = most_points(FinalReindeer),
    io:fwrite("Most points: ~w (by ~p)\n", [MostPointsReindeer#reindeer.lead_points, MostPointsReindeer]).


%% see https://erlangcentral.org/wiki/index.php?title=Read_File_to_List
readlines(FileName) ->
    {ok, Device} = file:open(FileName, [read]),
    get_all_lines(Device, []).

get_all_lines(Device, Accum) ->
    case io:get_line(Device, "") of
        eof -> file:close(Device), lists:reverse(Accum);
        Line -> get_all_lines(Device, [parse_line(Line)|Accum])
    end.

parse_line(Line) ->
    case re:run(Line, "\\s*(\\S+) can fly (\\d+) km/s for (\\d+) seconds?, but then must rest for (\\d+) seconds?\\.?\\s*", [{capture, all_but_first, list}]) of
        {match, [Name,Speed,Flytime,Resttime]} -> 
            case {string:to_integer(Speed), string:to_integer(Flytime), string:to_integer(Resttime)} of
                {{SpeedVal, ""}, {FlyVal, ""}, {RestVal, ""}} ->
                    #reindeer{name=Name, speed=SpeedVal, flytime=FlyVal, resttime=RestVal, distance=0, lead_points=0};
                _ -> io:fwrite("Bad line: ~s\n", [Line]),
                     exit(1)
            end;
        _ -> io:fwrite("Bad line: ~s\n", [Line]),
             exit(1)
    end.

advance(Reindeer, X, X) -> Reindeer;
advance(Reindeer, Time, MaxTime) ->
    MovedReindeer = lists:map(fun(R) -> move_single(R, Time) end, Reindeer),
    ScoredReindeer = reward_leaders(MovedReindeer),
%    lists:foreach(fun(R) -> io:fwrite("~p\n", [R]) end, ScoredReindeer),
%    io:fwrite("----\n", []),
    advance(ScoredReindeer, Time+1, MaxTime).

move_single(R, Time) ->
    Ft = R#reindeer.flytime,
    Rt = R#reindeer.resttime,
    Amount = if (Time rem (Ft + Rt)) < Ft -> R#reindeer.speed; true -> 0 end,
    R#reindeer{distance=R#reindeer.distance + Amount}.

reward_leaders(Reindeer) ->
    SortedReindeer = lists:reverse(lists:keysort(#reindeer.distance, Reindeer)),
    FarthestReindeer = lists:filter(fun(R) -> R#reindeer.distance >= (hd(SortedReindeer))#reindeer.distance end, Reindeer),
    RewardedReindeer = lists:map(fun(R) -> R#reindeer{lead_points=R#reindeer.lead_points + 1} end, FarthestReindeer),
    NotFarthestReindeer = lists:filter(fun(R) -> R#reindeer.distance < (hd(SortedReindeer))#reindeer.distance end, Reindeer),
    RewardedReindeer ++ NotFarthestReindeer.

best_distance(FinalReindeer) ->
    Folder = fun(R, A) -> 
                     if R#reindeer.distance > A#reindeer.distance -> R; 
                        true -> A 
                     end
             end,
    lists:foldl(Folder, #reindeer{name="Dummy", distance=-1}, FinalReindeer).

most_points(FinalReindeer) ->
    Folder = fun(R, A) -> 
                     if R#reindeer.lead_points > A#reindeer.lead_points -> R; 
                        true -> A 
                     end
             end,
    lists:foldl(Folder, #reindeer{name="Dummy", lead_points=-1}, FinalReindeer).

main(Args) ->
    case Args of
        [FileName, Time|_] ->
            case string:to_integer(Time) of
                {TimeVal, ""} -> run(FileName, TimeVal)
            end
    end.
