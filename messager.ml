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
open MyServer
open Fn
open CLI

let () =
  let argc, args = CLI.init () in
  if argc = 1
  then begin
    printf "usage:\n%s <fport> <port>" Sys.argv.(0);
    ignore (exit (-1) >>= const Deferred.unit)
  end
  else begin
    let fport = CLI.get_int [] args in
    let port = CLI.get_int [] args in
    let s = Socket.create Socket.Type.tcp |> flip (Socket.bind_inet ~reuseaddr:true) (Socket.Address.Inet.create_bind_any ~port:fport) |> Socket.listen ~backlog:1 in
    ignore ((Socket.accept s
             >>= function
             | `Socket_closed -> assert false
             | `Ok (c,a) ->
               let fd = Socket.fd c in
               let reader = Reader.create fd in
               let writer = Writer.create fd in
               run writer reader port) >>= const Deferred.unit);
    never_returns (Scheduler.go ())
  end
