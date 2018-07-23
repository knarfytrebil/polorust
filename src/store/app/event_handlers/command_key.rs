impl AppState {
    fn command_key_handler(&mut self, evt: event::Key) {
        match evt {
            event::Key::Esc => {
                self.set_mode("normal");
            }
            event::Key::Backspace => {
                if self.command == ":" {
                    self.set_mode("normal");
                } else {
                    self.command.pop();
                }
            }
            // Must be above Char(_char)
            event::Key::Char('\n') => {
                let cmd = self.command.split_off(1);
                if &cmd == "q" {
                    self.exiting = true;
                } else {
                    info!("Command Issued: {:?}", cmd);
                    let line = format!("{{fg=green [command] }} {}\n", &cmd);
                    self.console_txt.push_str(&line);
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    let quotes = vec![
                        Quote {
                            symbol: String::from("btc"),
                            bid_price: 1000,
                            ask_price: 1100,
                            bid_size: 100,
                            ask_size: 100,
                            timestamp: 13213123,
                        },
                        Quote {
                            symbol: String::from("btc"),
                            bid_price: 1000,
                            ask_price: 1100,
                            bid_size: 100,
                            ask_size: 100,
                            timestamp: 13213123,
                        },
                    ];
                    let res = run_python(
                        py,
                        &quotes,
"\n\nimport random\n\ndef trade():\n    if random.randint(0, 2) == 0:\n        return 1    \n    return sum([i['ask_price'] for i in data])\n"
                    );
                    self.console_txt
                        .push_str(&format!("{{fg=yellow [pyout] }}{:?}\n", &res));
                    let res = py.eval("sum([1, 2, 3, 4])", None, None);
                    self.console_txt
                        .push_str(&format!("{{fg=yellow [pyout] }}{:?}\n", &res));
                }
            }
            event::Key::Char(_char) => {
                self.command.push(_char);
            }
            _ => {}
        }
    }
}
