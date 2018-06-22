open Core
open Async
open Fn

let (>>) a b = a >>= const b
let rec read_msg r fw buf =
  Reader.read r buf
  >>= function
  | `Eof -> Deferred.unit
  | `Ok x ->
    let rec f iter s =
      if iter = x then s
      else f (iter+1) (Bytes.get buf iter |> String.make 1 |> (^) s)
    in
    f 0 "" |> Writer.write fw;
    Writer.flushed fw >> read_msg r fw buf

let rec write_msg w fr buf =
  Reader.read fr buf
  >>= function
  | `Eof -> Deferred.unit
  | `Ok x ->
    let rec f iter s =
      if iter = x then s
      else f (iter+1) (Bytes.get buf iter |> String.make 1 |> (^) s)
    in
    f 0 "" |> Writer.write w;
    Writer.flushed w >> write_msg w fr buf

let callback fw fr a r w =
  let t,r' = Lwt.task () in
  let reader = Lwt.(t >>= (fun () -> read_msg r fw (Bytes.create 10000) |> return)) in
  let writer = Lwt.(t >>= (fun () -> write_msg w fr (Bytes.create 10000) |> return)) in
  Lwt.wakeup r' ();
  Lwt.pick [reader;writer] |> ignore;
  Deferred.unit

let run fw fr =
  let server = Tcp.Server.create
      ?max_connections:(Some 1)
      ?backlog:(Some 5)
      (Tcp.Where_to_listen.of_port 8080)
      ~on_handler_error:`Raise
      (callback fw fr)
  in
  server >>= fun _ -> Deferred.unit

