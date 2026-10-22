{ ... }: {
  perSystem = { pkgs, ... }: {
    devShells.default = pkgs.mkShell {
      buildInputs = [
        pkgs.cargo
        pkgs.clippy
        pkgs.rustfmt
        pkgs.pkg-config
        pkgs.dbus
        pkgs.go
        pkgs.gcc
      ];
      shellHook = ''
        echo "Доступны: Rust, Go, GCC"
      '';
    };
  };
}
