use FileExplorer::bash_commands::bash_commands::*;

fn main() {
    let mut actual_dir = "/Users/ilianus/Desktop/EPITA/S4/PROJET/test";
    create_dir(actual_dir, "/NOUVO");

    // Changer le répertoire de travail
    let nouveau_repertoire = "/Users/ilianus/Desktop/EPITA/S4/PROJET/test/NOUVO";
    change_d(nouveau_repertoire);

    actual_dir = "/Users/ilianus/Desktop/EPITA/S4/PROJET/test/NOUVO";

    create_dir(actual_dir, "/NOUVO2");

    dif
}
