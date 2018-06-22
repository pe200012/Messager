open Core
open Async
open MyServer

let () =
    ignore (run ());
    never_returns (Scheduler.go ())