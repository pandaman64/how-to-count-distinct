with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "how-to-count-distinct";
  propagatedBuildInputs = [
    bashInteractive
    rustup
    pkgconfig
    openssl
    postgresql
    gcc
    docker
  ];
}
