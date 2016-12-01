#!/usr/bin/env escript

-record(reindeer, {name, speed, flytime, resttime}).

run(FileName, Time) ->
    Reindeer = readlines(FileName),
    {BestReindeer, BestDistance} = lists:foldl(fun(R, A) -> best_flyer(R, Time, A) end, {none, -1}, Reindeer),
    io:fwrite("Best distance: ~w (by ~p)\n", [BestDistance, BestReindeer]).

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
                    #reindeer{name=Name, speed=SpeedVal, flytime=FlyVal, resttime=RestVal};
                _ -> io:fwrite("Bad line: ~s\n", [Line]),
                     exit(1)
            end;
        _ -> io:fwrite("Bad line: ~s\n", [Line]),
             exit(1)
    end.

best_flyer(Reindeer, Time, {PrevBestReindeer, PrevBestDistance}) ->
    Distance = fly_for(Reindeer, Time),
    if Distance > PrevBestDistance -> {Reindeer, Distance};
       true -> {PrevBestReindeer, PrevBestDistance}
    end.

fly_for(#reindeer{name=_, speed=Speed, flytime=Flytime, resttime=Resttime}, Time) ->
    FullIntervals = Time div (Flytime + Resttime),
    FullIntervalDistance = FullIntervals * Flytime * Speed,
    %% in the partial interval they're either flying for the full time and
    %% resting for a bit, or flying for a bit but not the full time
    PartialIntervalSecs = min(Time rem (Flytime + Resttime), Flytime),
    PartialIntervalDistance = PartialIntervalSecs * Speed,
    FullIntervalDistance + PartialIntervalDistance.

main(Args) ->
    case Args of
        [FileName, Time|_] ->
            case string:to_integer(Time) of
                {TimeVal, ""} ->run(FileName, TimeVal)
            end
    end.
