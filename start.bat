@echo off
cd %~dp0\car-rental-frontend
npm install && start npm run

cd %~dp0
cargo build && cargo run
