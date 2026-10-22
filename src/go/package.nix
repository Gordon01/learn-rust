{ ... }: {
  perSystem = { pkgs, ... }: {
    packages.go = pkgs.buildGoModule rec {
      pname = "stack-go";
      version = "0.1.0";
      src = ./.;
      vendorHash = null;

      postPatch = ''
        go mod init ${pname}
      '';
    };
  };
}
