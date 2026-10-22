{ ... }: {
  perSystem = { pkgs, ... }: {
    packages.crates = pkgs.rustPlatform.buildRustPackage {
      pname = "workspace";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;

      nativeBuildInputs = [ pkgs.pkg-config ];
      buildInputs = [ pkgs.dbus ];
    };
  };
}
