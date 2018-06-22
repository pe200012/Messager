(**
 *    Copyright 2018 pe200012
 * 
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 * 
 *        http://www.apache.org/licenses/LICENSE-2.0
 * 
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 *)

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

