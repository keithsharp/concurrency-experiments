# Concurrency Experiments
Small experiments in writing parallel, concurrent, and asynchronous Rust programs.  Clone the repository and use `cargo run` with the `-p <crate name>` flag to run one of programs.

+ [MPSC Oneshot](https://github.com/keithsharp/concurrency-experiments/tree/main/mpsc-oneshot) - Communicating between [Tasks](https://docs.rs/tokio/latest/tokio/task/index.html) with [Tokio MPSC and Oneshot channels](https://docs.rs/tokio/latest/tokio/sync/index.html).
+ [Events and Tasks](https://github.com/keithsharp/concurrency-experiments/tree/main/events-tasks) - Combining events and long running tasks.
+ [Tokio Tutorial](https://github.com/keithsharp/concurrency-experiments/tree/main/tokio-tutorial) - Working through the [Tokio tutorial](https://tokio.rs/tokio/tutorial).

# Copyright and License
Copyright 2022, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.