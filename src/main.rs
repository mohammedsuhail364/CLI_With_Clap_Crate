use clap::{Arg, ArgGroup, Command, command};

fn main() {
    let _ = command!()
        .about("This application registers people with their doctors'office.")
        .subcommand(
            Command::new("register-name")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        .help("The person's first name"), // .conflicts_with("lastname"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("The person's last name"),
                ),
        )
        // .group(
        //     ArgGroup::new("person-register")
        //         .arg("firstname")
        //         .arg("lastname"),
        // )
        // .group(ArgGroup::new("dog-register").arg("petname"))
        .subcommand(
            Command::new("register-pet").arg(
                Arg::new("petname")
                    .short('p')
                    .long("pet-name")
                    .aliases(["pname", "petname"])
                    .required(true)
                    .help("The pet name"),
            ),
        )
        .arg(Arg::new("fluffy").long("fluffy"))
        .get_matches();
}
