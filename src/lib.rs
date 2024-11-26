use std::io;

pub async fn is_open (host: &str, port :u16, timeout: u64) -> bool {
    matches! (tokio::time::timout(Duration::from_secs(timeout), tokio::net::TcpStream::connect
    (format!("{}:{}", host, port))).await, Ok(Ok(_)))
}

fn parametres() -> (String, u16, u16, u64) {
    // Demander à l'utilisateur d'entrer un hôte
    print!("Entrez l'hôte : ");
    io::stdout().flush().unwrap();
    let mut host = String::new();
    io::stdin().read_line(&mut host).unwrap();
    let host = host.trim().to_string();

    // Demander à l'utilisateur d'entrer le port principal
    print!("Entrez le port principal : ");
    io::stdout().flush().unwrap();
    let mut port_main = String::new();
    io::stdin().read_line(&mut port_main).unwrap();
    let port_main: u16 = port_main.trim().parse().expect("Veuillez entrer un nombre valide");

    // Demander à l'utilisateur d'entrer le port maximum
    print!("Entrez le port maximum : ");
    io::stdout().flush().unwrap();
    let mut port_max = String::new();
    io::stdin().read_line(&mut port_max).unwrap();
    let port_max: u16 = port_max.trim().parse().expect("Veuillez entrer un nombre valide");

    // Demander à l'utilisateur d'entrer le timeout
    print!("Entrez le timeout (en secondes) : ");
    io::stdout().flush().unwrap();
    let mut timeout = String::new();
    io::stdin().read_line(&mut timeout).unwrap();
    let timeout: u64 = timeout.trim().parse().expect("Veuillez entrer un nombre valide");

    (host, port_main, port_max, timeout)
}