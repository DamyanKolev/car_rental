@echo off
cd %~dp0\car-rental-frontend
npm install && start npm start

cd %~dp0
cargo build && cargo run
